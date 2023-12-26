use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use rabbit_orm::pagination::{AdditionalFields, Cursor};

use crate::api::dto::TaskVersionBy;
use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::api::task_version::dto::TaskVersionArgs;
use crate::error::AppError;

#[derive(Default)]
pub struct TaskVersionQuery;

#[Object]
impl TaskVersionQuery {
    pub async fn task_versions<'a>(
        &self,
        ctx: &Context<'a>,
        args: TaskVersionArgs,
    ) -> Result<Connection<Cursor, TaskVersion, AdditionalFields>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_task_version = task_version_service.paginate(args).await?;

        Ok(paginated_task_version.into())
    }

    pub async fn task_version<'a>(
        &self,
        ctx: &Context<'a>,
        by: TaskVersionBy,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        match by {
            TaskVersionBy::Id(id) => {
                let task_version = task_version_service.find_by_id(id).await?;

                Ok(task_version)
            }
            TaskVersionBy::Version(version) => {
                let task_version = task_version_service
                    .find_by_semver(version.task_id, version.version)
                    .await?;

                Ok(task_version)
            }
            TaskVersionBy::Latest(latest) => {
                let task_version = task_version_service.find_latest(latest.task_id).await?;

                Ok(task_version)
            }
        }
    }
}
