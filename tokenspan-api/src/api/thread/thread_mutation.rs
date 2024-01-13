use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::api::models::{ParsedToken, UserRole};
use crate::api::services::ThreadServiceDyn;
use crate::api::thread::dto::{ThreadCreateInput, ThreadUpdateInput};
use crate::api::thread::thread_model::Thread;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct ThreadMutation;

#[Object]
impl ThreadMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_thread<'a>(
        &self,
        ctx: &Context<'a>,
        input: ThreadCreateInput,
    ) -> Result<Thread> {
        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread = thread_service
            .new(input, parsed_token.user_id.clone())
            .await?;

        Ok(thread)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_thread<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: ThreadUpdateInput,
    ) -> Result<Thread> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread = thread_service.update_by_id(&id, input).await?;

        Ok(thread)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn delete_thread<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Thread> {
        let thread_service = ctx
            .data::<ThreadServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let thread = thread_service.delete_by_id(&id).await?;

        Ok(thread)
    }
}
