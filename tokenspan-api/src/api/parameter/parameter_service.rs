use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    QueryFilter,
};
use uuid::Uuid;

use crate::api::dto::parameter_input::{ParameterCreateInput, ParameterUpdateInput};
use crate::api::models::Parameter;
use crate::api::parameter::parameter_error::ParameterError;

#[async_trait::async_trait]
pub trait ParameterServiceExt {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Parameter>>;
    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Parameter>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Parameter>>;
    async fn create(&self, input: ParameterCreateInput) -> Result<Parameter>;
    async fn update_by_id(&self, id: Uuid, input: ParameterUpdateInput) -> Result<Parameter>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Parameter>;
}

pub type ParameterServiceDyn = Arc<dyn ParameterServiceExt + Send + Sync>;

pub struct ParameterService {
    db: DatabaseConnection,
}

impl ParameterService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ParameterServiceExt for ParameterService {
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

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Parameter>> {
        let parameters = entity::parameter::Entity::find()
            .filter(entity::parameter::Column::Id.is_in(ids.to_vec()))
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
        let mut updated_parameter = entity::parameter::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ParameterError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ParameterError::Unknown(anyhow::anyhow!(
                "Parameter not found"
            )))?
            .into_active_model();

        input.merge(&mut updated_parameter);

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
