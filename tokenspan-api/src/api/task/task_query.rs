use crate::api::models::TaskId;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskArgs;
use crate::api::task::task_model::Task;
use crate::error::AppError;
use tokenspan_utils::pagination::Cursor;

#[derive(Default)]
pub struct TaskQuery;

#[Object]
impl TaskQuery {
    pub async fn tasks<'a>(
        &self,
        ctx: &Context<'a>,
        args: TaskArgs,
    ) -> Result<Connection<Cursor, Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_task = task_service.get_tasks(args).await?;

        Ok(paginated_task.into())
    }

    pub async fn task<'a>(&self, ctx: &Context<'a>, id: TaskId) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.get_task_by_id(id).await?;

        Ok(task)
    }
}
