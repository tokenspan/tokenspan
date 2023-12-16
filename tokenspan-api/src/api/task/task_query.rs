use crate::api::models::{ParsedToken, TaskId};
use async_graphql::connection::Connection;
use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskArgs;
use crate::api::task::task_model::Task;
use crate::error::AppError;
use tokenspan_extra::pagination::{AdditionalFields, Cursor};

#[derive(Default)]
pub struct TaskQuery;

#[Object]
impl TaskQuery {
    pub async fn tasks<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(default)] args: TaskArgs,
    ) -> Result<Connection<Cursor, Task, AdditionalFields>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_task = task_service.paginate(args).await?;

        Ok(paginated_task.into())
    }

    pub async fn tasks_by_owner<'a>(
        &self,
        ctx: &Context<'a>,
        args: TaskArgs,
    ) -> Result<Connection<Cursor, Task, AdditionalFields>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let paginated_task = task_service
            .find_by_owner(parsed_token.user_id.clone(), args)
            .await?;

        Ok(paginated_task.into())
    }

    pub async fn task<'a>(&self, ctx: &Context<'a>, id: TaskId) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.find_by_id(id).await?;

        Ok(task)
    }

    pub async fn count_tasks<'a>(&self, ctx: &Context<'a>, id: TaskId) -> Result<Option<Task>> {
        let task_service = ctx
            .data::<TaskServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task = task_service.find_by_id(id).await?;

        Ok(task)
    }
}
