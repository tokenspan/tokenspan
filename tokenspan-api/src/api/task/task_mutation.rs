use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::api::dto::{ParameterCreateInput, ParameterInput};
use crate::api::models::{ModelId, ParsedToken, TaskId};
use crate::api::services::{ModelServiceDyn, TaskServiceDyn, TaskVersionServiceDyn};
use crate::api::task::dto::{TaskCreateInput, TaskUpdateInput};
use crate::api::task::task_model::Task;
use crate::api::task_version::dto::TaskVersionCreateInput;
use crate::api::types::Role;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    // #[graphql(guard = "RoleGuard::new(Role::User)")]
    pub async fn create_task<'a>(
        &self,
        ctx: &Context<'a>,
        input: TaskCreateInput,
        model_id: ModelId,
    ) -> Result<Task> {
        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model_service = ctx
            .data::<ModelServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_service
            .find_by_id(model_id)
            .await?
            .ok_or(AppError::NotFound("model not found".to_string()))?;

        let created_task = task_service
            .create(input, parsed_token.user_id.clone())
            .await?;

        let parameter = ParameterCreateInput {
            data: ParameterInput {
                name: "default".to_string(),
                temperature: 1.0,
                max_tokens: 100,
                stop_sequences: Vec::new(),
                top_p: 1.0,
                frequency_penalty: 1.0,
                presence_penalty: 1.0,
                extra: None,
                model_id: model.id,
            },
        };
        task_version_service
            .create(
                TaskVersionCreateInput {
                    task_id: created_task.id.clone(),
                    version: "0.0.0".to_string(),
                    release_note: None,
                    description: None,
                    document: None,
                    messages: Vec::new(),
                    parameters: vec![parameter],
                },
                &parsed_token.user_id,
            )
            .await?;

        Ok(created_task)
    }

    #[graphql(guard = "RoleGuard::new(Role::User)")]
    pub async fn update_task<'a>(
        &self,
        ctx: &Context<'a>,
        id: TaskId,
        input: TaskUpdateInput,
    ) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.update_by_id(id, input).await?;

        Ok(task)
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_task<'a>(&self, ctx: &Context<'a>, id: TaskId) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.delete_by_id(id).await?;

        Ok(task)
    }
}
