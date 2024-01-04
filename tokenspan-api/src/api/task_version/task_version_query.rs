use async_graphql::{Context, Object, Result};

use crate::api::dto::{TaskVersionBy, TaskVersionBySemver};
use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::error::AppError;

#[derive(Default)]
pub struct TaskVersionQuery;

#[Object]
impl TaskVersionQuery {
    pub async fn task_version<'a>(
        &self,
        ctx: &Context<'a>,
        by: TaskVersionBy,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version = match by {
            TaskVersionBy::Id(id) => task_version_service.find_by_id(id).await,
            TaskVersionBy::Semver(TaskVersionBySemver { task_id, semver })
                if semver == "latest".to_string() =>
            {
                task_version_service.find_latest(task_id).await
            }
            TaskVersionBy::Semver(version) => {
                task_version_service
                    .find_by_semver(version.task_id, version.semver)
                    .await
            }
            TaskVersionBy::Latest(latest) => task_version_service.find_latest(latest.task_id).await,
        }?;

        Ok(task_version)
    }
}
