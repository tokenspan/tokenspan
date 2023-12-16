use async_graphql::{Context, Object, Result};

use crate::api::model::dto::{ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_model::Model;
use crate::api::models::ModelId;
use crate::api::services::ModelServiceDyn;
use crate::api::types::Role;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ModelMutation;

#[Object]
impl ModelMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn create_model<'a>(
        &self,
        ctx: &Context<'a>,
        input: ModelCreateInput,
    ) -> Result<Model> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.create(input).await?;

        Ok(model)
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_model<'a>(
        &self,
        ctx: &Context<'a>,
        id: ModelId,
        input: ModelUpdateInput,
    ) -> Result<Option<Model>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.update_by_id(id, input).await?;

        Ok(model)
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_model<'a>(&self, ctx: &Context<'a>, id: ModelId) -> Result<Option<Model>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.delete_by_id(id).await?;

        Ok(model)
    }
}
