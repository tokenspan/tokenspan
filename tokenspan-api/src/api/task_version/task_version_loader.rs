use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::api::task_version::task_version_error::TaskVersionError;

pub struct TaskVersionLoader {
    pub task_version_service: TaskVersionServiceDyn,
}

impl TaskVersionLoader {
    pub fn new(task_version_service: TaskVersionServiceDyn) -> Self {
        Self {
            task_version_service,
        }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for TaskVersionLoader {
    type Value = TaskVersion;
    type Error = Arc<TaskVersionError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let task_versions = self
            .task_version_service
            .find_by_ids(keys)
            .await
            .map_err(|e| Arc::new(TaskVersionError::Unknown(e)))?
            .into_iter()
            .map(|task_version| (task_version.id.clone(), task_version))
            .collect();

        Ok(task_versions)
    }
}
