use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::domains::dto::ThreadVersionPublishInput;
use crate::domains::models::{ParsedToken, ThreadVersion, UserRole};
use crate::domains::services::ThreadVersionServiceDyn;
use crate::domains::thread_version::dto::ThreadVersionUpdateInput;
use crate::errors::AppError;
use crate::guards::RoleGuard;

#[derive(Default)]
pub struct ThreadVersionMutation;

#[Object]
impl ThreadVersionMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn publish_thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ThreadVersionPublishInput,
    ) -> Result<ThreadVersion> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let thread_version = thread_version_service
            .publish(&id, input, parsed_token.user_id)
            .await?;

        Ok(thread_version)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ThreadVersionUpdateInput,
    ) -> Result<ThreadVersion> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread_version = thread_version_service.update_by_id(&id, input).await?;

        Ok(thread_version)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> Result<ThreadVersion> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread_version = thread_version_service.delete_by_id(&id).await?;

        Ok(thread_version)
    }
}
