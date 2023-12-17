use async_graphql::{Context, ErrorExtensions, Object, Result};
use uuid::Uuid;

use crate::api::models::{ParsedToken, UserRole};
use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::{TaskCreateInput, TaskUpdateInput};
use crate::api::task::task_model::Task;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn create_task<'a>(&self, ctx: &Context<'a>, input: TaskCreateInput) -> Result<Task> {
        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let created_task = task_service.create(input, parsed_token.user_id).await?;

        Ok(created_task)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::User)")]
    pub async fn update_task<'a>(
        &self,
        ctx: &Context<'a>,
        id: Uuid,
        input: TaskUpdateInput,
    ) -> Result<Task> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.update_by_id(id, input).await?;

        Ok(task)
    }

    #[graphql(guard = "RoleGuard::new(UserRole::Admin)")]
    pub async fn delete_task<'a>(&self, ctx: &Context<'a>, id: Uuid) -> Result<Task> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.delete_by_id(id).await?;

        Ok(task)
    }
}
