use std::collections::HashMap;

use async_graphql::dataloader::Loader;

use crate::api::models::{TaskVersion, TaskVersionId};
use crate::api::task_version::task_version_error::TaskVersionError;
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<TaskVersionId> for AppLoader {
    type Value = TaskVersion;
    type Error = TaskVersionError;

    async fn load(
        &self,
        keys: &[TaskVersionId],
    ) -> Result<HashMap<TaskVersionId, Self::Value>, Self::Error> {
        let task_versions = self
            .task_version_service
            .get_task_versions_by_ids(keys.to_vec())
            .await
            .map_err(|_| TaskVersionError::UnableToGetTaskVersions)?
            .into_iter()
            .map(|task_version| (task_version.id.clone(), task_version))
            .collect();

        Ok(task_versions)
    }
}
