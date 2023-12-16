use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
};
use async_openai::Client;
use axum::extract::FromRef;
use regex::Regex;
use serde_json::json;
use tracing::info;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::caches::api_key_cache::ApiKeyCacheDyn;
use crate::api::caches::model_cache::ModelCacheDyn;
use crate::api::dto::{ElapsedInput, ExecutionCreateInput, TaskExecuteInput};
use crate::api::models::{Execution, Model, Parameter, TaskId, UserId};
use crate::api::repositories::{TaskCreateEntity, TaskUpdateEntity};
use crate::api::services::{ExecutionServiceDyn, TaskVersionServiceDyn};
use crate::api::task::dto::{TaskArgs, TaskCreateInput, TaskUpdateInput};
use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;
use crate::api::types::{Endpoint, ExecutionStatus, Usage};
use crate::prompt::ChatMessage;
use crate::repository::RootRepository;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait TaskServiceExt {
    async fn paginate(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>>;
    async fn find_by_owner(
        &self,
        user_id: UserId,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>>;
    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>>;
    async fn find_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Task>>;
    async fn count(&self) -> Result<u64>;
    async fn create(&self, input: TaskCreateInput, owner: UserId) -> Result<Task>;
    async fn update_by_id(&self, id: TaskId, input: TaskUpdateInput) -> Result<Option<Task>>;
    async fn delete_by_id(&self, id: TaskId) -> Result<Option<Task>>;
    async fn execute(&self, input: TaskExecuteInput, execution_by_id: UserId) -> Result<Execution>;
}

pub type TaskServiceDyn = Arc<dyn TaskServiceExt + Send + Sync>;

impl FromRef<AppState> for TaskServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.task_service.clone()
    }
}

pub struct TaskService {
    repository: RootRepository,
    api_key_cache: ApiKeyCacheDyn,
    model_cache: ModelCacheDyn,

    execution_service: ExecutionServiceDyn,
    task_version_service: TaskVersionServiceDyn,
}

impl TaskService {
    pub fn new(
        repository: RootRepository,
        api_key_cache: ApiKeyCacheDyn,
        model_cache: ModelCacheDyn,

        execution_service: ExecutionServiceDyn,
        task_version_service: TaskVersionServiceDyn,
    ) -> Self {
        Self {
            repository,
            api_key_cache,
            model_cache,
            execution_service,
            task_version_service,
        }
    }

    pub async fn chat_completion(
        &self,
        chat_messages: &[ChatMessage],
        parameter: Parameter,
        api_key: String,
        model: Model,
    ) -> anyhow::Result<(
        CreateChatCompletionResponse,
        Vec<ChatCompletionRequestMessage>,
    )> {
        let mut messages: Vec<ChatCompletionRequestMessage> = vec![];
        for message in chat_messages.iter().cloned() {
            messages.push(message.try_into()?);
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(model.name.clone())
            .max_tokens(parameter.max_tokens)
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
        let paginated = self
            .repository
            .task
            .paginate::<Task>(args.into())
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn find_by_owner(
        &self,
        user_id: UserId,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>> {
        let paginated = self
            .repository
            .task
            .paginate_by_owner::<Task>(user_id, args.into())
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>> {
        let task = self
            .repository
            .task
            .find_by_id(id)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .map(|task| task.into());

        Ok(task)
    }

    async fn find_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>> {
        let tasks = self
            .repository
            .task
            .find_many_by_ids(ids)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task| task.into())
            .collect();

        Ok(tasks)
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Task>> {
        let task = self
            .repository
            .task
            .find_by_slug(slug)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .map(|task| task.into());

        Ok(task)
    }

    async fn count(&self) -> Result<u64> {
        let count = self
            .repository
            .task
            .count()
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

        Ok(count)
    }

    async fn create(&self, input: TaskCreateInput, owner: UserId) -> Result<Task> {
        let created_task = self
            .repository
            .task
            .create(TaskCreateEntity {
                owner_id: owner,
                name: input.name,
                slug: input.slug,
                private: input.private,
            })
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_task.into())
    }

    async fn update_by_id(&self, id: TaskId, input: TaskUpdateInput) -> Result<Option<Task>> {
        let updated_task = self
            .repository
            .task
            .update_by_id(
                id,
                TaskUpdateEntity {
                    name: input.name.clone(),
                    slug: input.name,
                    private: input.private,
                },
            )
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .map(|task| task.into());

        Ok(updated_task)
    }

    async fn delete_by_id(&self, id: TaskId) -> Result<Option<Task>> {
        let deleted_task = self
            .repository
            .task
            .delete_by_id(id)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .map(|task| task.into());

        Ok(deleted_task)
    }

    async fn execute(&self, input: TaskExecuteInput, execute_by_id: UserId) -> Result<Execution> {
        let start = Instant::now();
        let api_key = self
            .api_key_cache
            .get(input.api_key_id)
            .await
            .ok_or(ApiKeyError::Unknown(anyhow::anyhow!("API key not found")))?;

        let task_version = self
            .task_version_service
            .find_by_id(input.task_version_id.clone())
            .await?
            .ok_or(TaskError::Unknown(anyhow::anyhow!(
                "Task version not found"
            )))?;

        let re = Regex::new(r#"<var\sname="([a-zA-Z]+)"/>"#).unwrap();
        let messages: Vec<ChatMessage> = task_version
            .messages
            .into_iter()
            .map(|message| {
                let mut content = message.content.clone();
                for cap in re.captures_iter(&message.content) {
                    let variable = input
                        .variables
                        .get(&cap[1])
                        .ok_or(TaskError::Unknown(anyhow::anyhow!("Variable not found")))?;
                    info!("{} {} {}", &cap[0], &cap[1], variable);
                    content = content.replace(&cap[0], variable);
                }

                Ok(ChatMessage {
                    content,
                    raw: message.raw,
                    role: message.role,
                })
            })
            .collect::<anyhow::Result<Vec<ChatMessage>>>()?;

        let parameter = task_version
            .parameters
            .into_iter()
            .find(|parameter| parameter.id == input.parameter_id)
            .ok_or(TaskError::Unknown(anyhow::anyhow!("Parameter not found")))?;

        let model = self
            .model_cache
            .get(parameter.model_id.clone())
            .await
            .ok_or(TaskError::Unknown(anyhow::anyhow!("Model not found")))?;
        let pre_elapsed = start.elapsed();

        let start = Instant::now();
        let response = self
            .chat_completion(&messages, parameter.clone(), api_key, model)
            .await;
        let elapsed = start.elapsed();

        let start = Instant::now();
        let (status, output, usage, messages, error) = match response {
            Err(e) => (
                ExecutionStatus::Failure,
                None,
                None,
                vec![],
                Some(json!(e.to_string())),
            ),
            Ok(response) => (
                ExecutionStatus::Success,
                Some(json!(response.0)),
                response.0.usage,
                response.1.iter().map(|message| json!(message)).collect(),
                None,
            ),
        };

        let usage = usage.map(|usage| Usage {
            input_tokens: usage.prompt_tokens,
            output_tokens: usage.completion_tokens,
            total_tokens: usage.total_tokens,
        });
        let parameter = json!(parameter);
        let post_elapsed = start.elapsed();

        let execution = self
            .execution_service
            .create(
                ExecutionCreateInput {
                    task_id: task_version.task_id,
                    task_version_id: input.task_version_id,
                    endpoint: Endpoint::Http,
                    elapsed: ElapsedInput {
                        pre_elapsed: pre_elapsed.as_secs_f64(),
                        elapsed: elapsed.as_secs_f64(),
                        post_elapsed: post_elapsed.as_secs_f64(),
                    },
                    variables: input.variables,
                    parameter,
                    status,
                    output,
                    error,
                    messages,
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
