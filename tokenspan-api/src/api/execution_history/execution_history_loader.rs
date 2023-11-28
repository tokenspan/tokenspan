use std::collections::HashMap;

use async_graphql::dataloader::Loader;

use crate::api::execution_history::execution_history_error::ExecutionHistoryError;
use crate::api::models::{ExecutionHistory, ExecutionHistoryId};
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ExecutionHistoryId> for AppLoader {
    type Value = ExecutionHistory;
    type Error = ExecutionHistoryError;

    async fn load(
        &self,
        keys: &[ExecutionHistoryId],
    ) -> Result<HashMap<ExecutionHistoryId, Self::Value>, Self::Error> {
        let execution_histories = self
            .execution_history_service
            .get_execution_histories_by_ids(keys.to_vec())
            .await
            .map_err(|_| ExecutionHistoryError::UnableToGetExecutionHistories)?
            .into_iter()
            .map(|execution_history| (execution_history.id.clone(), execution_history))
            .collect();

        Ok(execution_histories)
    }
}
