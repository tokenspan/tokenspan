use std::sync::Arc;

use async_graphql::Result;
use openai_api_rust::chat::{ChatApi, ChatBody};
use openai_api_rust::{Auth, Message, OpenAI, Role};
use prisma_client_rust::Direction;
use tracing::info;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::execution_history::dto::CreateExecutionHistoryInput;
use crate::api::model::model_error::ModelError;
use crate::api::models::{ExecutionHistory, TaskId, UserId};
use crate::api::parameter::parameter_error::ParameterError;
use crate::api::services::{
    ApiKeyServiceDyn, ExecutionHistoryServiceDyn, ModelServiceDyn, ParameterServiceDyn,
};
use crate::api::task::dto::{CreateTaskInput, ExecuteTaskInput, TaskArgs, UpdateTaskInput};
use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;
use crate::prisma::{task, user, Endpoint, ExecutionStatus, PrismaClient};

#[async_trait::async_trait]
pub trait TaskServiceExt {
    async fn get_tasks(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>>;
    async fn get_task_by_id(&self, id: TaskId) -> Result<Option<Task>>;
    async fn get_tasks_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>>;
    async fn count_tasks(&self) -> Result<i64>;
    async fn create_task(&self, input: CreateTaskInput, owner: UserId) -> Result<Task>;
    async fn update_task(&self, id: TaskId, input: UpdateTaskInput) -> Result<Task>;
    async fn delete_task(&self, id: TaskId) -> Result<Task>;
    async fn execute_task(
        &self,
        input: ExecuteTaskInput,
        execution_by_id: UserId,
    ) -> Result<ExecutionHistory>;
}

pub type TaskServiceDyn = Arc<dyn TaskServiceExt + Send + Sync>;

pub struct TaskService {
    prisma: Arc<PrismaClient>,
    parameter_service: ParameterServiceDyn,
    model_service: ModelServiceDyn,
    api_key_service: ApiKeyServiceDyn,
    execution_history_service: ExecutionHistoryServiceDyn,
}

impl TaskService {
    pub fn new(
        prisma: Arc<PrismaClient>,
        parameter_service: ParameterServiceDyn,
        model_service: ModelServiceDyn,
        api_key_service: ApiKeyServiceDyn,
        execution_history_service: ExecutionHistoryServiceDyn,
    ) -> Self {
        Self {
            prisma,
            parameter_service,
            model_service,
            api_key_service,
            execution_history_service,
        }
    }
}

#[async_trait::async_trait]
impl TaskServiceExt for TaskService {
    async fn get_tasks(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .task()
            .find_many(vec![])
            .take(take + 1)
            .order_by(task::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take(-(take + 2))
                .cursor(task::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(task::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| TaskError::UnableToGetTasks)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_task_by_id(&self, id: TaskId) -> Result<Option<Task>> {
        let task = self
            .prisma
            .task()
            .find_unique(task::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| TaskError::UnableToGetTask)?
            .map(|task| task.into());

        Ok(task)
    }

    async fn get_tasks_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>> {
        let ids = ids
            .into_iter()
            .map(|id| task::id::equals(id.into()))
            .collect();
        let tasks = self
            .prisma
            .task()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| TaskError::UnableToGetTasks)?
            .into_iter()
            .map(|task| task.into())
            .collect();

        Ok(tasks)
    }

    async fn count_tasks(&self) -> Result<i64> {
        let count = self
            .prisma
            .task()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| TaskError::UnableToCountTasks)?;

        Ok(count)
    }

    async fn create_task(&self, input: CreateTaskInput, owner: UserId) -> Result<Task> {
        let created_task = self
            .prisma
            .task()
            .create(
                user::id::equals(owner.to_string()),
                input.name,
                input.slug,
                vec![],
            )
            .exec()
            .await
            .map_err(|_| TaskError::UnableToCreateTask)?;

        Ok(created_task.into())
    }

    async fn update_task(&self, id: TaskId, input: UpdateTaskInput) -> Result<Task> {
        let updated_task = self
            .prisma
            .task()
            .update(task::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| TaskError::UnableToUpdateTask)?;

        Ok(updated_task.into())
    }

    async fn delete_task(&self, id: TaskId) -> Result<Task> {
        let deleted_task = self
            .prisma
            .task()
            .delete(task::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| TaskError::UnableToDeleteTask)?;

        Ok(deleted_task.into())
    }

    async fn execute_task(
        &self,
        input: ExecuteTaskInput,
        execution_by_id: UserId,
    ) -> Result<ExecutionHistory> {
        let parameter = self
            .parameter_service
            .get_parameter_by_id(input.parameter_id)
            .await?
            .ok_or(ParameterError::UnableToGetParameter)?;

        let _model = self
            .model_service
            .get_model_by_id(parameter.model_id.clone())
            .await?
            .ok_or(ModelError::UnableToGetModel)?;

        let api_key = self
            .api_key_service
            .get_api_key_by_id(input.api_key_id)
            .await?
            .ok_or(ApiKeyError::UnableToCreateApiKey)?;

        let auth = Auth::new(api_key.key.as_str());
        let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
        let body = ChatBody {
            model: "gpt-3.5-turbo".to_string(),
            max_tokens: Some(7),
            temperature: Some(0_f32),
            top_p: Some(0_f32),
            n: Some(2),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: vec![Message {
                role: Role::User,
                content: "Hello!".to_string(),
            }],
        };
        let rs = openai.chat_completion_create(&body);
        let choices = rs.unwrap().choices;
        info!("choices: {:?}", choices);
        let _message = &choices[0].message.as_ref().unwrap();

        let parameter = serde_json::to_value(&parameter).unwrap();

        let execution_history_input = CreateExecutionHistoryInput {
            task_version_id: input.task_version_id,
            endpoint: Endpoint::Studio,
            elapsed_ms: 0,
            status: ExecutionStatus::Pending,
            messages: vec![],
            parameter,
            output: None,
            error: None,
            usage: Default::default(),
        };

        self.execution_history_service
            .create_execution_history(execution_history_input, execution_by_id)
            .await
    }
}

impl From<TaskService> for TaskServiceDyn {
    fn from(value: TaskService) -> Self {
        Arc::new(value) as Self
    }
}
