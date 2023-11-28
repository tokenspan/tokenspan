use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use crate::api::execution_history::dto::{CreateExecutionHistoryInput, ExecutionHistoryArgs};
use crate::api::execution_history::execution_history_error::ExecutionHistoryError;
use crate::api::execution_history::execution_history_model::ExecutionHistory;
use crate::api::models::{ExecutionHistoryId, UserId};
use crate::prisma::{execution_history, task_version, user, PrismaClient};
use tokenspan_utils::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ExecutionHistoryServiceExt {
    async fn get_execution_histories(
        &self,
        args: ExecutionHistoryArgs,
    ) -> Result<Pagination<Cursor, ExecutionHistory>>;
    async fn get_execution_history_by_id(
        &self,
        id: ExecutionHistoryId,
    ) -> Result<Option<ExecutionHistory>>;
    async fn get_execution_histories_by_ids(
        &self,
        ids: Vec<ExecutionHistoryId>,
    ) -> Result<Vec<ExecutionHistory>>;
    async fn count_execution_histories(&self) -> Result<i64>;
    async fn create_execution_history(
        &self,
        input: CreateExecutionHistoryInput,
        executed_by_id: UserId,
    ) -> Result<ExecutionHistory>;
    async fn delete_execution_history(&self, id: ExecutionHistoryId) -> Result<ExecutionHistory>;
}

pub type ExecutionHistoryServiceDyn = Arc<dyn ExecutionHistoryServiceExt + Send + Sync>;

pub struct ExecutionHistoryService {
    prisma: Arc<PrismaClient>,
}

impl ExecutionHistoryService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl ExecutionHistoryServiceExt for ExecutionHistoryService {
    async fn get_execution_histories(
        &self,
        args: ExecutionHistoryArgs,
    ) -> Result<Pagination<Cursor, ExecutionHistory>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .execution_history()
            .find_many(vec![])
            .take(take + 1)
            .order_by(execution_history::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take((take + 2) * -1)
                .cursor(execution_history::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(execution_history::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| ExecutionHistoryError::UnableToGetExecutionHistories)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_execution_history_by_id(
        &self,
        id: ExecutionHistoryId,
    ) -> Result<Option<ExecutionHistory>> {
        let execution_history = self
            .prisma
            .execution_history()
            .find_unique(execution_history::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ExecutionHistoryError::UnableToGetExecutionHistory)?
            .map(|execution_history| execution_history.into());

        Ok(execution_history)
    }

    async fn get_execution_histories_by_ids(
        &self,
        ids: Vec<ExecutionHistoryId>,
    ) -> Result<Vec<ExecutionHistory>> {
        let ids = ids
            .into_iter()
            .map(|id| execution_history::id::equals(id.into()))
            .collect();
        let execution_histories = self
            .prisma
            .execution_history()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| ExecutionHistoryError::UnableToGetExecutionHistories)?
            .into_iter()
            .map(|execution_history| execution_history.into())
            .collect();

        Ok(execution_histories)
    }

    async fn count_execution_histories(&self) -> Result<i64> {
        let count = self
            .prisma
            .execution_history()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| ExecutionHistoryError::UnableToCountExecutionHistories)?;

        Ok(count)
    }

    async fn create_execution_history(
        &self,
        input: CreateExecutionHistoryInput,
        executed_by_id: UserId,
    ) -> Result<ExecutionHistory> {
        let created_execution_history = self
            .prisma
            .execution_history()
            .create(
                task_version::id::equals(input.task_version_id.into()),
                user::id::equals(executed_by_id.into()),
                input.endpoint,
                input.elapsed_ms,
                input.status,
                input.parameter,
                vec![],
            )
            .exec()
            .await
            .map_err(|_| ExecutionHistoryError::UnableToCreateExecutionHistory)?;

        Ok(created_execution_history.into())
    }

    async fn delete_execution_history(&self, id: ExecutionHistoryId) -> Result<ExecutionHistory> {
        let deleted_execution_history = self
            .prisma
            .execution_history()
            .delete(execution_history::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ExecutionHistoryError::UnableToDeleteExecutionHistory)?;

        Ok(deleted_execution_history.into())
    }
}

impl From<ExecutionHistoryService> for ExecutionHistoryServiceDyn {
    fn from(value: ExecutionHistoryService) -> Self {
        Arc::new(value) as Self
    }
}
