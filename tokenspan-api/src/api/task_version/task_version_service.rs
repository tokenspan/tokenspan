use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, TransactionTrait,
};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use entity::sea_orm_active_enums::TaskVersionStatus;
use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::models::TaskVersion;
use crate::api::services::{MessageServiceDyn, ParameterServiceDyn};
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

    parameter_service: ParameterServiceDyn,
    message_service: MessageServiceDyn,
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

        // process messages
        if let Some(mut messages) = input.messages.clone() {
            for message in messages.iter_mut() {
                message.task_version_id = id;
            }

            self.message_service.upsert_many(messages).await?;
        }

        // process parameters
        if let Some(mut parameters) = input.parameters.clone() {
            for parameter in parameters.iter_mut() {
                parameter.task_version_id = id;
            }

            self.parameter_service.upsert_many(parameters).await?;
        }

        let mut task_version = entity::task_version::Entity::find_by_id(id)
            .one(&tx)
            .await
            .map_err(|e| TaskVersionError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskVersionError::Unknown(anyhow::anyhow!(
                "TaskVersion not found"
            )))?
            .into_active_model();
        input.copy(&mut task_version);
        let updated_task_version = task_version
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
