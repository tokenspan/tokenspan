use crate::api::models::ExecutionHistoryId;
use async_graphql::connection::Connection;
use async_graphql::{Context, Object, Result};

use crate::api::execution_history::dto::ExecutionHistoryArgs;
use crate::api::execution_history::execution_history_model::ExecutionHistory;
use crate::api::services::ExecutionHistoryServiceDyn;
use crate::error::AppError;
use tokenspan_utils::pagination::Cursor;

#[derive(Default)]
pub struct ExecutionHistoryQuery;

#[Object]
impl ExecutionHistoryQuery {
    pub async fn execution_histories<'a>(
        &self,
        ctx: &Context<'a>,
        args: ExecutionHistoryArgs,
    ) -> Result<Connection<Cursor, ExecutionHistory>> {
        let execution_history_service = ctx
            .data::<ExecutionHistoryServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let paginated_execution_history = execution_history_service
            .get_execution_histories(args)
            .await?;

        Ok(paginated_execution_history.into())
    }

    pub async fn execution_history<'a>(
        &self,
        ctx: &Context<'a>,
        id: ExecutionHistoryId,
    ) -> Result<Option<ExecutionHistory>> {
        let execution_history_service = ctx
            .data::<ExecutionHistoryServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let execution_history = execution_history_service
            .get_execution_history_by_id(id)
            .await?;

        Ok(execution_history)
    }
}
