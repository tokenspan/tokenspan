use std::sync::Arc;

use anyhow::Result;
use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateChatCompletionResponse,
};
use async_openai::Client;
use axum::extract::FromRef;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::caches::api_key_cache::ApiKeyCacheDyn;
use crate::api::caches::model_cache::ModelCacheDyn;
use crate::api::models::{Model, Parameter, TaskId, UserId};
use crate::api::services::{ExecutionServiceDyn, TaskVersionServiceDyn};
use crate::api::task::dto::{TaskArgs, TaskCreateInput, TaskUpdateInput};
use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;
use crate::prompt::ChatMessage;
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
    async fn create(&self, input: TaskCreateInput, owner_id: UserId) -> Result<Task>;
    async fn update_by_id(&self, id: TaskId, input: TaskUpdateInput) -> Result<Task>;
    async fn delete_by_id(&self, id: TaskId) -> Result<Task>;
}

pub type TaskServiceDyn = Arc<dyn TaskServiceExt + Send + Sync>;

impl FromRef<AppState> for TaskServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.task_service.clone()
    }
}

pub struct TaskService {
    db: DatabaseConnection,
    api_key_cache: ApiKeyCacheDyn,
    model_cache: ModelCacheDyn,

    execution_service: ExecutionServiceDyn,
    task_version_service: TaskVersionServiceDyn,
}

impl TaskService {
    pub fn new(
        db: DatabaseConnection,
        api_key_cache: ApiKeyCacheDyn,
        model_cache: ModelCacheDyn,

        execution_service: ExecutionServiceDyn,
        task_version_service: TaskVersionServiceDyn,
    ) -> Self {
        Self {
            db,
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
        let take = args.take.unwrap_or(10) as u64;
        let mut cursor = entity::task::Entity::find()
            .cursor_by(entity::task::Column::Id)
            .order_by_desc(entity::task::Column::Id)
            .limit(Some(take));

        if let Some(after) = args.after.clone() {
            cursor.after(after.id);
        }

        if let Some(before) = args.before.clone() {
            cursor.before(before.id);
        }

        let count = entity::task::Entity::find().count(&self.db).await?;
        let items = cursor
            .all(&self.db)
            .await
            .map_err(|e| crate::api::task::task_error::TaskError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task| task.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(
            items,
            args.before,
            args.after,
            take as i64,
            count,
        ))
    }

    async fn find_by_owner(
        &self,
        user_id: UserId,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>> {
        let take = args.take.unwrap_or(10) as u64;
        let mut cursor = entity::task::Entity::find()
            .filter(entity::task::Column::OwnerId.eq(user_id))
            .cursor_by(entity::task::Column::Id)
            .order_by_desc(entity::task::Column::Id)
            .limit(Some(take));

        if let Some(after) = args.after.clone() {
            cursor.after(after.id);
        }

        if let Some(before) = args.before.clone() {
            cursor.before(before.id);
        }

        let count = entity::task::Entity::find().count(&self.db).await?;
        let items = cursor
            .all(&self.db)
            .await
            .map_err(|e| crate::api::task::task_error::TaskError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task| task.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(
            items,
            args.before,
            args.after,
            take as i64,
            count,
        ))
    }

    async fn find_by_id(&self, id: TaskId) -> Result<Option<Task>> {
        let task = entity::task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .map(|task| task.into());

        Ok(task)
    }

    async fn find_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>> {
        let ids = ids.into_iter().map(|id| id.to_string()).collect::<Vec<_>>();
        let tasks = entity::task::Entity::find()
            .filter(entity::task::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|task| task.into())
            .collect::<Vec<_>>();

        Ok(tasks)
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Task>> {
        let task = entity::task::Entity::find()
            .filter(entity::task::Column::Slug.eq(slug))
            .one(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .map(|task| task.into());

        Ok(task)
    }

    async fn create(&self, input: TaskCreateInput, owner_id: UserId) -> Result<Task> {
        let task = entity::task::ActiveModel {
            id: Set(TaskId::new_v4()),
            name: Set(input.name.clone()),
            slug: Set(input.name),
            private: Set(input.private),
            owner_id: Set(owner_id),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(task)
    }

    async fn update_by_id(&self, id: TaskId, input: TaskUpdateInput) -> Result<Task> {
        let mut task = entity::task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskError::Unknown(anyhow::anyhow!("Task not found")))?
            .into_active_model();

        input.merge(&mut task);

        let task = task
            .update(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(task)
    }

    async fn delete_by_id(&self, id: TaskId) -> Result<Task> {
        let task = entity::task::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(TaskError::Unknown(anyhow::anyhow!("Task not found")))?;

        task.clone()
            .delete(&self.db)
            .await
            .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

        Ok(task.into())
    }

    // async fn execute(&self, input: TaskExecuteInput, execute_by_id: UserId) -> Result<Execution> {
    //     let start = Instant::now();
    //     let api_key = self
    //         .api_key_cache
    //         .get(input.api_key_id)
    //         .await
    //         .ok_or(ApiKeyError::Unknown(anyhow::anyhow!("API key not found")))?;
    //
    //     let task_version = self
    //         .task_version_service
    //         .find_by_id(input.task_version_id.clone())
    //         .await?
    //         .ok_or(TaskError::Unknown(anyhow::anyhow!(
    //             "Task version not found"
    //         )))?;
    //
    //     let re = Regex::new(r#"<var\sname="([a-zA-Z]+)"/>"#).unwrap();
    //     let messages: Vec<ChatMessage> = task_version
    //         .messages
    //         .into_iter()
    //         .map(|message| {
    //             let mut content = message.content.clone();
    //             for cap in re.captures_iter(&message.content) {
    //                 let variable = input
    //                     .variables
    //                     .get(&cap[1])
    //                     .ok_or(TaskError::Unknown(anyhow::anyhow!("Variable not found")))?;
    //                 info!("{} {} {}", &cap[0], &cap[1], variable);
    //                 content = content.replace(&cap[0], variable);
    //             }
    //
    //             Ok(ChatMessage {
    //                 content,
    //                 raw: message.raw,
    //                 role: message.role,
    //             })
    //         })
    //         .collect::<anyhow::Result<Vec<ChatMessage>>>()?;
    //
    //     let parameter = task_version
    //         .parameters
    //         .into_iter()
    //         .find(|parameter| parameter.id == input.parameter_id)
    //         .ok_or(TaskError::Unknown(anyhow::anyhow!("Parameter not found")))?;
    //
    //     let model = self
    //         .model_cache
    //         .get(parameter.model_id.clone())
    //         .await
    //         .ok_or(TaskError::Unknown(anyhow::anyhow!("Model not found")))?;
    //     let pre_elapsed = start.elapsed();
    //
    //     let start = Instant::now();
    //     let response = self
    //         .chat_completion(&messages, parameter.clone(), api_key, model)
    //         .await;
    //     let elapsed = start.elapsed();
    //
    //     let start = Instant::now();
    //     let (status, output, usage, messages, error) = match response {
    //         Err(e) => (
    //             ExecutionStatus::Failure,
    //             None,
    //             None,
    //             vec![],
    //             Some(json!(e.to_string())),
    //         ),
    //         Ok(response) => (
    //             ExecutionStatus::Success,
    //             Some(json!(response.0)),
    //             response.0.usage,
    //             response.1.iter().map(|message| json!(message)).collect(),
    //             None,
    //         ),
    //     };
    //
    //     let usage = usage.map(|usage| Usage {
    //         input_tokens: usage.prompt_tokens,
    //         output_tokens: usage.completion_tokens,
    //         total_tokens: usage.total_tokens,
    //     });
    //     let parameter = json!(parameter);
    //     let post_elapsed = start.elapsed();
    //
    //     let execution = self
    //         .execution_service
    //         .create(
    //             ExecutionCreateInput {
    //                 task_id: task_version.task_id,
    //                 task_version_id: input.task_version_id,
    //                 endpoint: Endpoint::Http,
    //                 elapsed: ElapsedInput {
    //                     pre_elapsed: pre_elapsed.as_secs_f64(),
    //                     elapsed: elapsed.as_secs_f64(),
    //                     post_elapsed: post_elapsed.as_secs_f64(),
    //                 },
    //                 variables: input.variables,
    //                 parameter,
    //                 status,
    //                 output,
    //                 error,
    //                 messages,
    //                 usage,
    //             },
    //             execute_by_id,
    //         )
    //         .await?;
    //
    //     Ok(execution)
    // }
}

impl From<TaskService> for TaskServiceDyn {
    fn from(value: TaskService) -> Self {
        Arc::new(value) as Self
    }
}
