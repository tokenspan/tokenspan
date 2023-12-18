use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::dto::{ParameterArgs, ParameterCreateInput, ParameterUpdateInput};
use crate::api::models::Parameter;
use crate::api::parameter::parameter_error::ParameterError;

#[async_trait::async_trait]
pub trait ParameterServiceExt {
    async fn paginate(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Parameter>>;
    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Parameter>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Parameter>>;
    async fn create(&self, inputs: ParameterCreateInput) -> Result<Parameter>;
    async fn update_by_id(&self, id: Uuid, input: ParameterUpdateInput) -> Result<Parameter>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Parameter>;
}

pub type ParameterServiceDyn = Arc<dyn ParameterServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ParameterService {
    db: DatabaseConnection,
}

#[async_trait::async_trait]
impl ParameterServiceExt for ParameterService {
    async fn paginate(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>> {
        let take = args.take.unwrap_or(10);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::parameter::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::parameter::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::parameter::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::parameter::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::parameter::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|parameter| parameter.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Parameter>> {
        let parameter = entity::parameter::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .map(|parameter| parameter.into());

        Ok(parameter)
    }

    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Parameter>> {
        let parameter = entity::parameter::Entity::find()
            .filter(entity::parameter::Column::TaskVersionId.eq(id))
            .all(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|parameter| parameter.into())
            .collect();

        Ok(parameter)
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Parameter>> {
        let parameters = entity::parameter::Entity::find()
            .filter(entity::parameter::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|parameter| parameter.into())
            .collect();

        Ok(parameters)
    }

    async fn create(&self, input: ParameterCreateInput) -> Result<Parameter> {
        let created_parameter = entity::parameter::ActiveModel {
            id: Set(Uuid::new_v4()),
            task_version_id: Set(input.task_version_id),
            model_id: Set(input.model_id),
            name: Set(input.name),
            max_tokens: Set(input.max_tokens as i32),
            temperature: Set(input.temperature),
            stop_sequences: Set(input.stop_sequences),
            top_p: Set(input.top_p),
            frequency_penalty: Set(input.frequency_penalty),
            presence_penalty: Set(input.presence_penalty),
            extra: Set(input.extra),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_parameter)
    }

    async fn update_by_id(&self, id: Uuid, input: ParameterUpdateInput) -> Result<Parameter> {
        let mut updated_parameter = entity::parameter::Entity::find()
            .filter(entity::parameter::Column::Id.eq(id))
            .one(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ParameterError::Unknown(anyhow::anyhow!(
                "Parameter not found"
            )))?
            .into_active_model();

        input.copy(&mut updated_parameter);

        let updated_parameter = updated_parameter
            .update(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(updated_parameter)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Parameter> {
        let deleted_parameter = entity::parameter::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ParameterError::Unknown(anyhow::anyhow!(
                "Parameter not found"
            )))?;

        deleted_parameter
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?;

        Ok(deleted_parameter.into())
    }
}

impl From<ParameterService> for ParameterServiceDyn {
    fn from(value: ParameterService) -> Self {
        Arc::new(value) as Self
    }
}
