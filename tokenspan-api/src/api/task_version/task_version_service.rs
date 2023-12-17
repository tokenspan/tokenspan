use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};
use tracing::info;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::dto::parameter_input::ParameterMutationInput;
use crate::api::model::model_cache::ModelCacheDyn;
use entity::sea_orm_active_enums::TaskVersionStatus;
use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::models::{Parameter, TaskVersion};
use crate::api::task_version::dto::{
    TaskVersionArgs, TaskVersionCreateInput, TaskVersionUpdateInput,
};
use crate::api::task_version::task_version_error::TaskVersionError;

#[async_trait::async_trait]
pub trait TaskVersionServiceExt {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskVersion>>;
    async fn find_by_semver(&self, task_id: Uuid, version: String) -> Result<Option<TaskVersion>>;
    async fn find_latest(&self, task_id: Uuid) -> Result<Option<TaskVersion>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<TaskVersion>>;
    async fn create(&self, input: TaskVersionCreateInput, owner_id: Uuid) -> Result<TaskVersion>;
    async fn update_by_id(&self, id: Uuid, input: TaskVersionUpdateInput) -> Result<TaskVersion>;
    async fn delete_by_id(&self, id: Uuid) -> Result<TaskVersion>;
    async fn release(&self, id: Uuid) -> Result<TaskVersion>;
}

pub type TaskVersionServiceDyn = Arc<dyn TaskVersionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct TaskVersionService {
    db: DatabaseConnection,
    model_cache: ModelCacheDyn,
}

#[async_trait::async_trait]
impl TaskVersionServiceExt for TaskVersionService {
    async fn paginate(&self, args: TaskVersionArgs) -> Result<Pagination<Cursor, TaskVersion>> {
        let take = args.take.unwrap_or(10);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::task_version::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::task_version::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::task_version::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::task_version::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::task_version::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<TaskVersion>> {
        let task_version = entity::task_version::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_by_semver(&self, task_id: Uuid, semver: String) -> Result<Option<TaskVersion>> {
        let task_version = entity::task_version::Entity::find()
            .filter(
                entity::task_version::Column::TaskId
                    .eq(task_id)
                    .and(entity::task_version::Column::Semver.eq(semver)),
            )
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_latest(&self, task_id: Uuid) -> Result<Option<TaskVersion>> {
        let task_version = entity::task_version::Entity::find()
            .filter(entity::task_version::Column::TaskId.eq(task_id))
            .order_by_desc(entity::task_version::Column::Id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .map(|task_version| task_version.into());

        Ok(task_version)
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<TaskVersion>> {
        let task_versions = entity::task_version::Entity::find()
            .filter(entity::task_version::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task_version| task_version.into())
            .collect::<Vec<_>>();

        Ok(task_versions)
    }

    async fn create(&self, input: TaskVersionCreateInput, owner_id: Uuid) -> Result<TaskVersion> {
        let messages = serde_json::to_value(input.messages)?;
        let parameters = serde_json::to_value(input.parameters)?;

        let created_task_version = entity::task_version::ActiveModel {
            id: Set(Uuid::new_v4()),
            task_id: Set(input.task_id.into()),
            owner_id: Set(owner_id.into()),
            semver: Set(input.semver),
            version: Set(input.version as i32),
            description: Set(input.description),
            release_note: Set(input.release_note),
            document: Set(input.document),
            status: Set(TaskVersionStatus::Draft),
            released_at: Set(None),
            messages: Set(messages),
            parameters: Set(parameters),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_task_version)
    }

    async fn update_by_id(&self, id: Uuid, input: TaskVersionUpdateInput) -> Result<TaskVersion> {
        let tx = self.db.begin().await?;

        let task_version = entity::task_version::Entity::find_by_id(id)
            .one(&tx)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                "TaskVersion not found"
            )))?;

        let mut active_model = task_version.clone().into_active_model();
        input.copy(&mut active_model);

        if let Some(message) = input.messages {
            let messages = serde_json::to_value(message)?;
            active_model.messages = Set(messages);
        }

        let mut parameters: Vec<Parameter> =
            serde_json::from_value(task_version.parameters.clone())?;
        let mut new_parameters = vec![];

        for parameter in parameters.iter() {
            let value = serde_json::to_value(parameter)?;
            new_parameters.push(value);
        }

        if let Some(mutations) = input.parameters {
            for mutation in mutations.into_iter() {
                match mutation {
                    ParameterMutationInput::Create(mut input) => {
                        self.model_cache.get(input.model_id).await.ok_or(
                            TaskVersionError::Unknown(anyhow::anyhow!("Model not found")),
                        )?;

                        info!("input: {:?}", input);
                        input.id = Uuid::new_v4();
                        let value = serde_json::to_value(input)?;
                        new_parameters.push(value);
                    }
                    ParameterMutationInput::Update(input) => {
                        let parameter = parameters
                            .iter_mut()
                            .find(|parameter| parameter.id == input.id)
                            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                                "Parameter not found"
                            )))?;

                        if let Some(model_id) = input.model_id {
                            self.model_cache.get(model_id).await.ok_or(
                                TaskVersionError::Unknown(anyhow::anyhow!("Model not found")),
                            )?;
                            parameter.model_id = model_id;
                        }

                        if let Some(ref name) = input.name {
                            parameter.name = name.clone();
                        }

                        if let Some(temperature) = input.temperature {
                            parameter.temperature = temperature;
                        }

                        if let Some(max_tokens) = input.max_tokens {
                            parameter.max_tokens = max_tokens;
                        }

                        if let Some(stop_sequences) = input.stop_sequences {
                            parameter.stop_sequences = stop_sequences.clone();
                        }

                        if let Some(top_p) = input.top_p {
                            parameter.top_p = top_p;
                        }

                        if let Some(frequency_penalty) = input.frequency_penalty {
                            parameter.frequency_penalty = frequency_penalty;
                        }

                        if let Some(presence_penalty) = input.presence_penalty {
                            parameter.presence_penalty = presence_penalty;
                        }

                        parameter.extra = input.extra.clone();

                        let pos = new_parameters
                            .iter()
                            .position(|p| p["id"] == parameter.id.to_string())
                            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                                "Parameter not found"
                            )))?;
                        let value = serde_json::to_value(parameter)?;
                        new_parameters[pos] = value;
                    }
                    ParameterMutationInput::Delete(id) => {
                        let parameter = parameters.iter().find(|parameter| parameter.id == id);

                        if parameter.is_none() {
                            return Err(TaskVersionError::Unknown(anyhow::anyhow!(
                                "Parameter not found"
                            )))?;
                        }
                    }
                };
            }

            let new_parameters = serde_json::to_value(new_parameters.clone())?;
            active_model.parameters = Set(new_parameters);
        }

        let updated_task_version = active_model
            .update(&tx)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .into();

        tx.commit().await?;

        Ok(updated_task_version)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<TaskVersion> {
        let deleted_task_version = entity::task_version::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                "TaskVersion not found"
            )))?;

        deleted_task_version
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(deleted_task_version.into())
    }

    async fn release(&self, _id: Uuid) -> Result<TaskVersion> {
        // TODO: copy parameters and save it to task_version
        todo!()
    }
}

impl From<TaskVersionService> for TaskVersionServiceDyn {
    fn from(value: TaskVersionService) -> Self {
        Arc::new(value) as Self
    }
}
