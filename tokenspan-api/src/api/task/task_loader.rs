use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::models::{Task, TaskId};
use crate::api::task::task_error::TaskError;
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<TaskId> for AppLoader {
    type Value = Task;
    type Error = Arc<TaskError>;

    async fn load(&self, keys: &[TaskId]) -> Result<HashMap<TaskId, Self::Value>, Self::Error> {
        let tasks = self
            .task_service
            .get_tasks_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(TaskError::Unknown(anyhow::anyhow!(e.message))))?
            .into_iter()
            .map(|task| (task.id.clone(), task))
            .collect();

        Ok(tasks)
    }
}
