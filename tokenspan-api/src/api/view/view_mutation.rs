use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::api::models::{ParsedToken, Role, ViewId};
use crate::api::services::ViewServiceDyn;
use crate::api::view::dto::{CreateViewInput, UpdateViewInput};
use crate::api::view::view_model::View;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ViewMutation;

#[Object]
impl ViewMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn create_view<'a>(&self, ctx: &Context<'a>, input: CreateViewInput) -> Result<View> {
        let view_service = ctx
            .data::<ViewServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        view_service
            .create_view(input, parsed_token.user_id.clone())
            .await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_view<'a>(
        &self,
        ctx: &Context<'a>,
        id: ViewId,
        input: UpdateViewInput,
    ) -> Result<View> {
        let view_service = ctx
            .data::<ViewServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        view_service.update_view(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_view<'a>(&self, ctx: &Context<'a>, id: ViewId) -> Result<View> {
        let view_service = ctx
            .data::<ViewServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        view_service.delete_view(id).await
    }
}
