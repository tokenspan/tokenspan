use std::sync::Arc;

use async_graphql::Result;
use bson::doc;
use bson::oid::ObjectId;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::execution::dto::{ExecutionArgs, ExecutionCreateInput};
use crate::api::execution::execution_error::ExecutionError;
use crate::api::execution::execution_model::Execution;
use crate::api::models::{ExecutionId, UserId};
use crate::api::repositories::ExecutionCreateEntity;
use crate::repository::RootRepository;

#[async_trait::async_trait]
pub trait ExecutionServiceExt {
    async fn get_executions(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>>;
    async fn get_execution_by_id(&self, id: ExecutionId) -> Result<Option<Execution>>;
    async fn get_executions_by_ids(&self, ids: Vec<ExecutionId>) -> Result<Vec<Execution>>;
    async fn create_execution(
        &self,
        input: ExecutionCreateInput,
        executed_by_id: UserId,
    ) -> Result<Execution>;
    async fn delete_execution(&self, id: ExecutionId) -> Result<Option<Execution>>;
}

pub type ExecutionServiceDyn = Arc<dyn ExecutionServiceExt + Send + Sync>;

pub struct ExecutionService {
    repository: Arc<RootRepository>,
}

impl ExecutionService {
    pub fn new(repository: Arc<RootRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ExecutionServiceExt for ExecutionService {
    async fn get_executions(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>> {
        let task_id = ObjectId::from(args.task_id.clone());
        let paginated = self
            .repository
            .execution
            .paginate_with_filter::<Execution>(
                doc! {
                    "task_id": task_id,
                },
                args.into(),
            )
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn get_execution_by_id(&self, id: ExecutionId) -> Result<Option<Execution>> {
        let execution = self
            .repository
            .execution
            .find_by_id(id)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .map(|execution| execution.into());

        Ok(execution)
    }

    async fn get_executions_by_ids(&self, ids: Vec<ExecutionId>) -> Result<Vec<Execution>> {
        let executions = self
            .repository
            .execution
            .find_many_by_ids(ids)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect();

        Ok(executions)
    }

    async fn create_execution(
        &self,
        input: ExecutionCreateInput,
        executed_by_id: UserId,
    ) -> Result<Execution> {
        let created_execution = self
            .repository
            .execution
            .create(ExecutionCreateEntity {
                endpoint: input.endpoint,
                elapsed: input.elapsed.into(),
                status: input.status,
                messages: input.messages,
                parameter: input.parameter,
                output: input.output,
                error: input.error,
                usage: input.usage,
                task_id: input.task_id,
                task_version_id: input.task_version_id,
                executed_by_id,
            })
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_execution.into())
    }

    async fn delete_execution(&self, id: ExecutionId) -> Result<Option<Execution>> {
        let deleted_execution = self
            .repository
            .execution
            .delete_by_id(id)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .map(|execution| execution.into());

        Ok(deleted_execution)
    }
}

impl From<ExecutionService> for ExecutionServiceDyn {
    fn from(value: ExecutionService) -> Self {
        Arc::new(value) as Self
    }
}
