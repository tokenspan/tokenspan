use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::execution::execution_error::ExecutionError;
use crate::api::models::{Execution, ExecutionId};
use crate::api::services::ExecutionServiceDyn;

pub struct ExecutionLoader {
    pub execution_service: ExecutionServiceDyn,
}

impl ExecutionLoader {
    pub fn new(execution_service: ExecutionServiceDyn) -> Self {
        Self { execution_service }
    }
}

#[async_trait::async_trait]
impl Loader<ExecutionId> for ExecutionLoader {
    type Value = Execution;
    type Error = Arc<ExecutionError>;

    async fn load(
        &self,
        keys: &[ExecutionId],
    ) -> Result<HashMap<ExecutionId, Self::Value>, Self::Error> {
        let executions = self
            .execution_service
            .find_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ExecutionError::Unknown(anyhow::anyhow!(e))))?
            .into_iter()
            .map(|execution| (execution.id.clone(), execution))
            .collect();

        Ok(executions)
    }
}
