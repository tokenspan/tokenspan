use std::sync::Arc;

use async_graphql::Result;
use axum::extract::FromRef;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::models::{TaskId, UserId};
use crate::api::repositories::{TaskCreateEntity, TaskUpdateEntity};
use crate::api::services::{
    ApiKeyServiceDyn, ExecutionServiceDyn, ModelServiceDyn, ParameterServiceDyn,
};
use crate::api::task::dto::{TaskArgs, TaskCreateInput, TaskUpdateInput};
use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;
use crate::repository::RootRepository;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait TaskServiceExt {
    async fn get_tasks(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>>;
    async fn get_tasks_by_owner(
        &self,
        user_id: UserId,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>>;
    async fn get_task_by_id(&self, id: TaskId) -> Result<Option<Task>>;
    async fn get_tasks_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>>;
    async fn count_tasks(&self) -> Result<u64>;
    async fn create_task(&self, input: TaskCreateInput, owner: UserId) -> Result<Task>;
    async fn update_task(&self, id: TaskId, input: TaskUpdateInput) -> Result<Option<Task>>;
    async fn delete_task(&self, id: TaskId) -> Result<Option<Task>>;
    // async fn execute_task(
    //     &self,
    //     input: TaskExecuteInput,
    //     execution_by_id: UserId,
    // ) -> Result<Execution>;
}

pub type TaskServiceDyn = Arc<dyn TaskServiceExt + Send + Sync>;

impl FromRef<AppState> for TaskServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.task_service.clone()
    }
}

pub struct TaskService {
    repository: Arc<RootRepository>,
    parameter_service: ParameterServiceDyn,
    model_service: ModelServiceDyn,
    api_key_service: ApiKeyServiceDyn,
    execution_service: ExecutionServiceDyn,
}

impl TaskService {
    pub fn new(
        repository: Arc<RootRepository>,
        parameter_service: ParameterServiceDyn,
        model_service: ModelServiceDyn,
        api_key_service: ApiKeyServiceDyn,
        execution_service: ExecutionServiceDyn,
    ) -> Self {
        Self {
            repository,
            parameter_service,
            model_service,
            api_key_service,
            execution_service,
        }
    }
}

#[async_trait::async_trait]
impl TaskServiceExt for TaskService {
    async fn get_tasks(&self, args: TaskArgs) -> Result<Pagination<Cursor, Task>> {
        let paginated = self
            .repository
            .task
            .paginate::<Task>(args.into())
            .await
            .map_err(|_| TaskError::UnableToCreateTask)?;

        Ok(paginated)
    }

    async fn get_tasks_by_owner(
        &self,
        user_id: UserId,
        args: TaskArgs,
    ) -> Result<Pagination<Cursor, Task>> {
        let paginated = self
            .repository
            .task
            .paginate_by_owner::<Task>(user_id, args.into())
            .await
            .map_err(|_| TaskError::UnableToCreateTask)?;

        Ok(paginated)
    }

    async fn get_task_by_id(&self, id: TaskId) -> Result<Option<Task>> {
        let task = self
            .repository
            .task
            .find_by_id(id)
            .await
            .map_err(|_| TaskError::UnableToGetTask)?
            .map(|task| task.into());

        Ok(task)
    }

    async fn get_tasks_by_ids(&self, ids: Vec<TaskId>) -> Result<Vec<Task>> {
        let tasks = self
            .repository
            .task
            .find_many_by_ids(ids)
            .await
            .map_err(|_| TaskError::UnableToGetTasks)?
            .into_iter()
            .map(|task| task.into())
            .collect();

        Ok(tasks)
    }

    async fn count_tasks(&self) -> Result<u64> {
        let count = self
            .repository
            .task
            .count()
            .await
            .map_err(|_| TaskError::UnableToCountTasks)?;

        Ok(count)
    }

    async fn create_task(&self, input: TaskCreateInput, owner: UserId) -> Result<Task> {
        let created_task = self
            .repository
            .task
            .create(TaskCreateEntity {
                owner_id: owner,
                name: input.name.clone(),
                slug: input.name,
                private: input.private,
            })
            .await
            .map_err(|_| TaskError::UnableToCreateTask)?;

        Ok(created_task.into())
    }

    async fn update_task(&self, id: TaskId, input: TaskUpdateInput) -> Result<Option<Task>> {
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
            .map_err(|_| TaskError::UnableToUpdateTask)?
            .map(|task| task.into());

        Ok(updated_task)
    }

    async fn delete_task(&self, id: TaskId) -> Result<Option<Task>> {
        let deleted_task = self
            .repository
            .task
            .delete_by_id(id)
            .await
            .map_err(|_| TaskError::UnableToDeleteTask)?
            .map(|task| task.into());

        Ok(deleted_task)
    }

    // async fn execute_task(
    //     &self,
    //     input: TaskExecuteInput,
    //     execution_by_id: UserId,
    // ) -> Result<Execution> {
    //     let parameter = self
    //         .parameter_service
    //         .get_parameter_by_id(input.parameter_id)
    //         .await?
    //         .ok_or(ParameterError::UnableToGetParameter)?;
    //
    //     let model = self
    //         .model_service
    //         .get_model_by_id(parameter.model_id.clone())
    //         .await?
    //         .ok_or(ModelError::UnableToGetModel)?;
    //
    //     let api_key = self
    //         .api_key_service
    //         .get_api_key_by_id(input.api_key_id)
    //         .await?
    //         .ok_or(ApiKeyError::UnableToCreateApiKey)?;
    //
    //     let parameter = serde_json::to_value(&parameter).unwrap();
    //
    //     let execution_input = ExecutionCreateInput {
    //         task_id: TaskId::new(),
    //         task_version_id: input.task_version_id,
    //         endpoint: Endpoint::Studio,
    //         elapsed_ms: 0,
    //         status: ExecutionStatus::Success,
    //         messages: vec![],
    //         parameter,
    //         output: None,
    //         error: None,
    //         usage: Default::default(),
    //     };
    //
    //     self.execution_service
    //         .create_execution(execution_input, execution_by_id)
    //         .await
    // }
}

impl From<TaskService> for TaskServiceDyn {
    fn from(value: TaskService) -> Self {
        Arc::new(value) as Self
    }
}
