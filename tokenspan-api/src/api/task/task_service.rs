use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
};
use async_openai::Client;
use axum::extract::FromRef;
use chrono::Utc;
use rabbit_orm::pagination::{Cursor, Pagination};
use rabbit_orm::{Db, Order};
use regex::Regex;
use serde_json::json;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::dto::{ElapsedInput, ExecutionCreateInput, TaskExecuteInput, UsageInput};
use crate::api::models::{Execution, ExecutionStatus, Model, Parameter, Task};
use crate::api::services::{
    ApiKeyServiceDyn, ExecutionServiceDyn, ModelServiceDyn, ParameterServiceDyn,
    TaskVersionServiceDyn,
};
use crate::api::task::dto::{TaskArgs, TaskCreateInput, TaskUpdateInput};
use crate::api::task::task_error::TaskError;
use crate::prompt::{ChatMessage, PromptRole};
use crate::state::AppState;

#[async_trait::async_trait]
pub trait TaskServiceExt {
    async fn paginate(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>>;
    async fn find_by_owner(
        &self,
        user_id: Uuid,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Task>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Task>>;
    async fn create(&self, input: TaskCreateInput, owner_id: Uuid) -> Result<Task>;
    async fn update_by_id(&self, id: Uuid, input: TaskUpdateInput) -> Result<Option<Task>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Task>>;
    async fn execute(&self, input: TaskExecuteInput, execute_by_id: Uuid) -> Result<Execution>;
}

pub type TaskServiceDyn = Arc<dyn TaskServiceExt + Send + Sync>;

impl FromRef<AppState> for TaskServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.task_service.clone()
    }
}

#[derive(TypedBuilder)]
pub struct TaskService {
    db: Db,
    api_key_service: ApiKeyServiceDyn,
    model_service: ModelServiceDyn,
    parameter_service: ParameterServiceDyn,
    execution_service: ExecutionServiceDyn,
    task_version_service: TaskVersionServiceDyn,
}

impl TaskService {
    pub async fn chat_completion(
        &self,
        chat_messages: &[ChatMessage],
        parameter: Parameter,
        api_key: String,
        model: Model,
    ) -> Result<(
        CreateChatCompletionResponse,
        Vec<ChatCompletionRequestMessage>,
    )> {
        let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
        for message in chat_messages.iter().cloned() {
            messages.push(message.try_into()?);
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(model.name.clone())
            .max_tokens(parameter.max_tokens as u16)
            .temperature(parameter.temperature)
            .top_p(parameter.top_p)
            .frequency_penalty(parameter.frequency_penalty)
            .presence_penalty(parameter.presence_penalty)
            .stop(parameter.stop_sequences)
            .messages(messages.clone())
            .build()
            .map_err(|e| anyhow::anyhow!(e))?;

        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = Client::with_config(config);
        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok((response, messages))
    }
}

#[async_trait::async_trait]
impl TaskServiceExt for TaskService {
    async fn paginate(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>> {
        self.db
            .clone()
            .from::<Task>()
            .select_all()
            .cursor(args.before, args.after)
            .order_by("created_at", Order::Desc)
            .limit(args.take.unwrap_or(10))
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_owner(
        &self,
        user_id: Uuid,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>> {
        self.db
            .clone()
            .from::<Task>()
            .select_all()
            .and_where("owner_id", "=", user_id)
            .cursor(args.before, args.after)
            .order_by("created_at", Order::Desc)
            .limit(args.take.unwrap_or(10))
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        self.db
            .clone()
            .from::<Task>()
            .select_all()
            .find(id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Task>> {
        self.db
            .clone()
            .from::<Task>()
            .select_all()
            .and_where("id", "in", ids)
            .all()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Task>> {
        self.db
            .clone()
            .from::<Task>()
            .select_all()
            .and_where("slug", "=", slug)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn create(&self, input: TaskCreateInput, owner_id: Uuid) -> Result<Task> {
        let input = Task {
            id: Uuid::new_v4(),
            owner_id,
            name: input.name,
            slug: input.slug,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db
            .clone()
            .from::<Task>()
            .insert(input)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn update_by_id(&self, id: Uuid, input: TaskUpdateInput) -> Result<Option<Task>> {
        self.db
            .clone()
            .from::<Task>()
            .update(input)
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        self.db
            .clone()
            .from::<Task>()
            .delete()
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn execute(&self, input: TaskExecuteInput, execute_by_id: Uuid) -> Result<Execution> {
        let start = Instant::now();
        let api_key = self
            .api_key_service
            .find_by_id(input.api_key_id)
            .await?
            .ok_or(ApiKeyError::Unknown(anyhow::anyhow!("API key not found")))?;

        let task_version = self
            .task_version_service
            .find_by_id(input.task_version_id.clone())
            .await?
            .ok_or(TaskError::Unknown(anyhow::anyhow!(
                "Task version not found"
            )))?;

        let re = Regex::new(r#"<var\sname="([a-zA-Z]+)"/>"#).unwrap();
        let chat_messages: Vec<ChatMessage> = task_version
            .messages
            .clone()
            .into_iter()
            .map(|message| {
                let mut content = message.content.clone();
                for cap in re.captures_iter(&message.content) {
                    let variable = input
                        .variables
                        .get(&cap[1])
                        .ok_or(TaskError::Unknown(anyhow::anyhow!("Variable not found")))?;
                    content = content.replace(&cap[0], variable);
                }

                Ok(ChatMessage {
                    content,
                    role: PromptRole::from_str(message.role.as_str())?,
                })
            })
            .collect::<Result<Vec<ChatMessage>>>()?;

        let parameter = self
            .parameter_service
            .find_by_id(input.parameter_id)
            .await?
            .ok_or(TaskError::Unknown(anyhow::anyhow!("Parameter not found")))?;

        let model = self
            .model_service
            .find_by_id(parameter.model_id)
            .await?
            .ok_or(TaskError::Unknown(anyhow::anyhow!("Model not found")))?;
        let pre_elapsed = start.elapsed();

        let start = Instant::now();
        let response = self
            .chat_completion(&chat_messages, parameter.clone(), api_key.key, model)
            .await;
        let elapsed = start.elapsed();

        let start = Instant::now();
        let (status, output, usage, error) = match response {
            Err(e) => (
                ExecutionStatus::Failure,
                None,
                None,
                Some(json!(e.to_string())),
            ),
            Ok(response) => (
                ExecutionStatus::Success,
                Some(json!(response.0)),
                response.0.usage,
                None,
            ),
        };

        let usage = usage.map(|usage| UsageInput {
            input_tokens: usage.prompt_tokens as i32,
            output_tokens: usage.completion_tokens as i32,
            total_tokens: usage.total_tokens as i32,
        });
        let post_elapsed = start.elapsed();

        let elapsed = ElapsedInput {
            pre_elapsed: pre_elapsed.as_secs_f64(),
            elapsed: elapsed.as_secs_f64(),
            post_elapsed: post_elapsed.as_secs_f64(),
        };

        let messages = task_version
            .messages
            .clone()
            .into_iter()
            .map(|m| m.into())
            .collect();

        let execution = self
            .execution_service
            .create(
                ExecutionCreateInput {
                    task_id: task_version.task_id,
                    task_version_id: input.task_version_id,
                    parameter_id: parameter.id,
                    variables: input.variables,
                    messages,
                    elapsed,
                    status,
                    output,
                    error,
                    usage,
                },
                execute_by_id,
            )
            .await?;

        Ok(execution)
    }
}

impl From<TaskService> for TaskServiceDyn {
    fn from(value: TaskService) -> Self {
        Arc::new(value) as Self
    }
}
