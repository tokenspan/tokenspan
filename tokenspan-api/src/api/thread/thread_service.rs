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
use dojo_orm::pagination::Pagination;
use dojo_orm::prelude::equals;
use dojo_orm::prelude::*;
use dojo_orm::Database;
use regex::Regex;
use serde_json::json;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::dto::{
    ExecutionCreateInput, ParameterCreateInput, ThreadExecuteInput, ThreadVersionCreateInput,
};
use crate::api::models::{Elapsed, Execution, ExecutionStatus, Model, Parameter, Thread, Usage};
use crate::api::services::{
    ApiKeyServiceDyn, ExecutionServiceDyn, MessageServiceDyn, ModelServiceDyn, ParameterServiceDyn,
    ThreadVersionServiceDyn,
};
use crate::api::thread::dto::{ThreadArgs, ThreadCreateInput, ThreadUpdateInput};
use crate::api::thread::thread_error::ThreadError;
use crate::prompt::{ChatMessage, PromptRole};
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ThreadServiceExt {
    async fn paginate(&self, args: ThreadArgs) -> Result<Pagination<Thread>>;
    async fn find_by_owner(&self, user_id: &Uuid, args: ThreadArgs) -> Result<Pagination<Thread>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Thread>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Thread>>;
    async fn find_by_slug(&self, slug: &String) -> Result<Option<Thread>>;
    async fn create(&self, input: ThreadCreateInput, owner_id: Uuid) -> Result<Thread>;
    async fn new(&self, input: ThreadCreateInput, owner_id: Uuid) -> Result<Thread>;
    async fn update_by_id(&self, id: &Uuid, input: ThreadUpdateInput) -> Result<Thread>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<Thread>;
    async fn execute(&self, input: ThreadExecuteInput, execute_by_id: Uuid) -> Result<Execution>;
}

pub type ThreadServiceDyn = Arc<dyn ThreadServiceExt + Send + Sync>;

impl FromRef<AppState> for ThreadServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.thread_service.clone()
    }
}

#[derive(TypedBuilder)]
pub struct ThreadService {
    db: Database,
    api_key_service: ApiKeyServiceDyn,
    model_service: ModelServiceDyn,
    parameter_service: ParameterServiceDyn,
    execution_service: ExecutionServiceDyn,
    thread_version_service: ThreadVersionServiceDyn,
    message_service: MessageServiceDyn,
}

impl ThreadService {
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
impl ThreadServiceExt for ThreadService {
    async fn paginate(&self, args: ThreadArgs) -> Result<Pagination<Thread>> {
        let mut predicates = vec![];
        if let Some(r#where) = &args.r#where {
            if let Some(thread_owner_args) = &r#where.owner_id {
                if let Some(id) = &thread_owner_args.equals {
                    predicates.push(equals("owner_id", id));
                }
            }
        }

        self.db
            .bind::<Thread>()
            .where_by(and(&predicates))
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_owner(&self, user_id: &Uuid, args: ThreadArgs) -> Result<Pagination<Thread>> {
        self.db
            .bind::<Thread>()
            .where_by(equals("owner_id", user_id))
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Thread>> {
        self.db
            .bind::<Thread>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Thread>> {
        self.db
            .bind::<Thread>()
            .where_by(in_list("id", &ids))
            .all()
            .await
    }

    async fn find_by_slug(&self, slug: &String) -> Result<Option<Thread>> {
        self.db
            .bind::<Thread>()
            .where_by(equals("slug", slug))
            .first()
            .await
    }

    async fn create(&self, input: ThreadCreateInput, owner_id: Uuid) -> Result<Thread> {
        let input = Thread {
            id: Uuid::new_v4(),
            owner_id,
            name: input.name,
            slug: input.slug,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn new(&self, input: ThreadCreateInput, owner_id: Uuid) -> Result<Thread> {
        // TODO: tx
        let created_thread = self.create(input, owner_id).await?;
        let created_thread_version = self
            .thread_version_service
            .create(
                ThreadVersionCreateInput::builder()
                    .thread_id(created_thread.id)
                    .build(),
                owner_id,
            )
            .await?;

        let model = self
            .model_service
            .find_first()
            .await?
            .ok_or(anyhow::anyhow!("model not found"))?;

        self.parameter_service
            .create(
                ParameterCreateInput::builder()
                    .model_id(model.id)
                    .thread_version_id(created_thread_version.id)
                    .build(),
            )
            .await?;

        Ok(created_thread)
    }

    async fn update_by_id(&self, id: &Uuid, input: ThreadUpdateInput) -> Result<Thread> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<Thread> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }

    async fn execute(&self, input: ThreadExecuteInput, execute_by_id: Uuid) -> Result<Execution> {
        let start = Instant::now();
        let api_key = self
            .api_key_service
            .find_by_id(&input.api_key_id)
            .await?
            .ok_or(ApiKeyError::Unknown(anyhow::anyhow!("API key not found")))?;

        let thread_version = self
            .thread_version_service
            .find_by_id(&input.thread_version_id)
            .await?
            .ok_or(ThreadError::Unknown(anyhow::anyhow!(
                "Thread version not found"
            )))?;

        let messages = self
            .message_service
            .find_by_thread_version_id(&input.thread_version_id)
            .await?;

        let re = Regex::new(r#"<var\sname="([a-zA-Z]+)"/>"#).unwrap();
        let chat_messages: Vec<ChatMessage> = messages
            .clone()
            .into_iter()
            .map(|message| {
                let mut content = message.content.clone();
                for cap in re.captures_iter(&message.content) {
                    let variable = input
                        .variables
                        .get(&cap[1])
                        .ok_or(ThreadError::Unknown(anyhow::anyhow!("Variable not found")))?;
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
            .find_by_id(&input.parameter_id)
            .await?
            .ok_or(ThreadError::Unknown(anyhow::anyhow!("Parameter not found")))?;

        let model = self
            .model_service
            .find_by_id(&parameter.model_id)
            .await?
            .ok_or(ThreadError::Unknown(anyhow::anyhow!("Model not found")))?;
        let pre_elapsed = start.elapsed();

        let start = Instant::now();
        let response = self
            .chat_completion(&chat_messages, parameter.clone(), api_key.key, model)
            .await;
        let elapsed = start.elapsed();

        let start = Instant::now();
        let (status, output, usage, error) = match response {
            Err(e) => (
                ExecutionStatus::Failed,
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

        let usage = usage.map(|usage| Usage {
            input_tokens: usage.prompt_tokens as i32,
            output_tokens: usage.completion_tokens as i32,
            total_tokens: usage.total_tokens as i32,
        });
        let post_elapsed = start.elapsed();

        let elapsed = Elapsed {
            pre_elapsed: pre_elapsed.as_secs_f64(),
            elapsed: elapsed.as_secs_f64(),
            post_elapsed: post_elapsed.as_secs_f64(),
        };

        let execution = self
            .execution_service
            .create(
                ExecutionCreateInput {
                    thread_id: thread_version.thread_id,
                    thread_version_id: input.thread_version_id,
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

impl From<ThreadService> for ThreadServiceDyn {
    fn from(value: ThreadService) -> Self {
        Arc::new(value) as Self
    }
}
