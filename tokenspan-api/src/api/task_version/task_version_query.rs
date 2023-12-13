use crate::api::models::TaskVersionId;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use crate::api::services::TaskVersionServiceDyn;
use crate::api::task_version::dto::TaskVersionArgs;
use crate::api::task_version::task_version_model::TaskVersion;
use crate::error::AppError;
use tokenspan_extra::pagination::{AdditionalFields, Cursor};

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

        let paginated_task_version = task_version_service.get_task_versions(args).await?;

        Ok(paginated_task_version.into())
    }

    pub async fn task_version<'a>(
        &self,
        ctx: &Context<'a>,
        id: TaskVersionId,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_version = task_version_service.get_task_version_by_id(id).await?;

        Ok(task_version)
    }
}
