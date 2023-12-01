use async_graphql::{Context, ErrorExtensions, Object, Result};
use bson::oid::ObjectId;

use crate::api::models::{Execution, ModelId, ParsedToken, TaskId};
use crate::api::parameter::dto::ParameterCreateInput;
use crate::api::repositories::TaskVersionStatus;
use crate::api::services::{ParameterServiceDyn, TaskServiceDyn, TaskVersionServiceDyn};
use crate::api::task::dto::{TaskCreateInput, TaskExecuteInput, TaskUpdateInput};
use crate::api::task::task_model::Task;
use crate::api::task_version::dto::TaskVersionCreateInput;
use crate::error::AppError;
use crate::guard::RoleGuard;

#[derive(Default)]
pub struct TaskMutation;

#[Object]
impl TaskMutation {
    #[graphql(guard = "RoleGuard::new(Role::User)")]
    pub async fn create_task<'a>(&self, ctx: &Context<'a>, input: TaskCreateInput) -> Result<Task> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parameter_service = ctx
            .data::<ParameterServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let created_task = task_service
            .create_task(input, parsed_token.user_id.clone())
            .await?;

        let create_task_version_input = TaskVersionCreateInput {
            task_id: created_task.id.clone(),
            version: "0.0.0".to_string(),
            release_note: None,
            description: None,
            document: None,
            messages: Vec::new(),
            status: TaskVersionStatus::Draft,
        };
        let created_task_version = task_version_service
            .create_task_version(create_task_version_input, &parsed_token.user_id)
            .await?;

        let gpt3_5_turbo_model_id =
            ModelId::from(ObjectId::parse_str("65617fc7b35c48147687a83c").unwrap());
        let create_parameter_input = ParameterCreateInput {
            task_version_id: created_task_version.id.clone(),
            model_id: gpt3_5_turbo_model_id,
            name: "untitled".to_string(),
            temperature: 1f64,
            presence_penalty: 0f64,
            frequency_penalty: 0f64,
            max_tokens: 64,
            top_p: 1f64,
            stop_sequences: Vec::new(),
            extra: None,
        };
        parameter_service
            .create_parameter(create_parameter_input)
            .await?;

        Ok(created_task)
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn update_task<'a>(
        &self,
        ctx: &Context<'a>,
        id: TaskId,
        input: TaskUpdateInput,
    ) -> Result<Task> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        task_service.update_task(id, input).await
    }

    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    pub async fn delete_task<'a>(&self, ctx: &Context<'a>, id: TaskId) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        task_service.delete_task(id).await
    }

    #[graphql(guard = "RoleGuard::new(Role::User)")]
    pub async fn execute<'a>(
        &self,
        ctx: &Context<'a>,
        _id: TaskId,
        input: TaskExecuteInput,
    ) -> Result<Execution> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        task_service
            .execute_task(input, parsed_token.user_id.clone())
            .await
    }
}
