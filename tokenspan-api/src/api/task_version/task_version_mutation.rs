use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::api::models::{ParsedToken, TaskVersion, UserRole};
use crate::api::services::TaskVersionServiceDyn;
use crate::api::task_version::dto::{TaskVersionCreateInput, TaskVersionUpdateInput};
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct TaskVersionMutation;

#[Object]
impl TaskVersionMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::Admin)")]
    pub async fn create_task_version<'a>(
        &self,
        ctx: &Context<'a>,
        input: TaskVersionCreateInput,
    ) -> Result<TaskVersion> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let task_version = task_version_service
            .create(input, parsed_token.user_id.clone())
            .await?;

        Ok(task_version)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::Admin)")]
    pub async fn update_task_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: TaskVersionUpdateInput,
    ) -> Result<TaskVersion> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version = task_version_service.update_by_id(id, input).await?;

        Ok(task_version)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::Admin)")]
    pub async fn delete_task_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
    ) -> Result<TaskVersion> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version = task_version_service.delete_by_id(id).await?;

        Ok(task_version)
    }
}
