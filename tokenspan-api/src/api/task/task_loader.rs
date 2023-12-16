use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::models::{Task, TaskId};
use crate::api::services::TaskServiceDyn;
use crate::api::task::task_error::TaskError;

pub struct TaskLoader {
    pub task_service: TaskServiceDyn,
}

impl TaskLoader {
    pub fn new(task_service: TaskServiceDyn) -> Self {
        Self { task_service }
    }
}

#[async_trait::async_trait]
impl Loader<TaskId> for TaskLoader {
    type Value = Task;
    type Error = Arc<TaskError>;

    async fn load(&self, keys: &[TaskId]) -> Result<HashMap<TaskId, Self::Value>, Self::Error> {
        let tasks = self
            .task_service
            .find_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(TaskError::Unknown(e)))?
            .into_iter()
            .map(|task| (task.id.clone(), task))
            .collect();

        Ok(tasks)
    }
}
