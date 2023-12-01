use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::api::models::{ParsedToken, Role, TaskVersionId};
use crate::api::services::TaskVersionServiceDyn;
use crate::api::task_version::dto::{TaskVersionCreateInput, TaskVersionUpdateInput};
use crate::api::task_version::task_version_model::TaskVersion;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct TaskVersionMutation;

#[Object]
impl TaskVersionMutation {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
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

        task_version_service
            .create_task_version(input, &parsed_token.user_id)
            .await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_task_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: TaskVersionId,
        input: TaskVersionUpdateInput,
    ) -> Result<TaskVersion> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        task_version_service.update_task_version(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_task_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: TaskVersionId,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        task_version_service.delete_task_version(id).await
    }
}
