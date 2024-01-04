use async_graphql::{Context, Object, Result};
use uuid::Uuid;

use crate::api::model::dto::{ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_model::Model;
use crate::api::models::UserRole;
use crate::api::services::ModelServiceDyn;
use crate::error::AppError;
use crate::guard::RoleGuard;

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
    ) -> Result<Option<Model>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.update_by_id(id, input).await?;

        Ok(model)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_model<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Option<Model>> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service.delete_by_id(id).await?;

        Ok(model)
    }
}
