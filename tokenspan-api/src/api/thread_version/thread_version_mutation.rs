use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::api::models::{ParsedToken, ThreadVersion, UserRole};
use crate::api::services::ThreadVersionServiceDyn;
use crate::api::thread_version::dto::{ThreadVersionCreateInput, ThreadVersionUpdateInput};
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ThreadVersionMutation;

#[Object]
impl ThreadVersionMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        input: ThreadVersionCreateInput,
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
            .create(input, parsed_token.user_id.clone())
            .await?;

        Ok(thread_version)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ThreadVersionUpdateInput,
    ) -> Result<Option<ThreadVersion>> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread_version = thread_version_service.update_by_id(id, input).await?;

        Ok(thread_version)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_thread_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> Result<Option<ThreadVersion>> {
        let thread_version_service = ctx
            .data::<ThreadVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread_version = thread_version_service.delete_by_id(id).await?;

        Ok(thread_version)
    }
}
