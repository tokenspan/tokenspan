use crate::api::models::ExecutionId;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use crate::api::execution::dto::ExecutionArgs;
use crate::api::execution::execution_model::Execution;
use crate::api::services::ExecutionServiceDyn;
use crate::error::AppError;
use tokenspan_utils::pagination::Cursor;

#[derive(Default)]
pub struct ExecutionQuery;

#[Object]
impl ExecutionQuery {
    pub async fn executions<'a>(
        &self,
        ctx: &Context<'a>,
        args: ExecutionArgs,
    ) -> Result<Connection<Cursor, Execution>> {
        let execution_service = ctx
            .data::<ExecutionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_execution = execution_service.get_executions(args).await?;

        Ok(paginated_execution.into())
    }

    pub async fn execution<'a>(
        &self,
        ctx: &Context<'a>,
        id: ExecutionId,
    ) -> Result<Option<Execution>> {
        let execution_service = ctx
            .data::<ExecutionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let execution = execution_service.get_execution_by_id(id).await?;

        Ok(execution)
    }
}
