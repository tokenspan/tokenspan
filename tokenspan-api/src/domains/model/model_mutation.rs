use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::domains::model::dto::{ModelCreateInput, ModelUpdateInput};
use crate::domains::model::model_model::Model;
use crate::domains::models::UserRole;
use crate::domains::services::ModelServiceDyn;
use crate::errors::AppError;
use crate::guards::RoleGuard;

#[derive(Default)]
pub struct ModelMutation;

#[Object]
impl ModelMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
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

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_model<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ModelUpdateInput,
    ) -> Result<Model> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.update_by_id(&id, input).await?;

        Ok(model)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_model<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Model> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.delete_by_id(&id).await?;

        Ok(model)
    }
}
