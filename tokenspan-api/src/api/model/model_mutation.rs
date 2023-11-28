use async_graphql::{Context, Object, Result};

use crate::api::model::dto::{CreateModelInput, UpdateModelInput};
use crate::api::model::model_model::Model;
use crate::api::models::{ModelId, Role};
use crate::api::services::ModelServiceDyn;
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
        input: CreateModelInput,
    ) -> Result<Model> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        model_service.create_model(input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_model<'a>(
        &self,
        ctx: &Context<'a>,
        id: ModelId,
        input: UpdateModelInput,
    ) -> Result<Model> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        model_service.update_model(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_model<'a>(&self, ctx: &Context<'a>, id: ModelId) -> Result<Model> {
        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        model_service.delete_model(id).await
    }
}
