use std::collections::HashMap;

use async_graphql::dataloader::Loader;

use crate::api::execution::execution_error::ExecutionError;
use crate::api::models::{Execution, ExecutionId};
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ExecutionId> for AppLoader {
    type Value = Execution;
    type Error = ExecutionError;

    async fn load(
        &self,
        keys: &[ExecutionId],
    ) -> Result<HashMap<ExecutionId, Self::Value>, Self::Error> {
        let executions = self
            .execution_service
            .get_executions_by_ids(keys.to_vec())
            .await
            .map_err(|_| ExecutionError::UnableToGetExecutions)?
            .into_iter()
            .map(|execution| (execution.id.clone(), execution))
            .collect();

        Ok(executions)
    }
}
