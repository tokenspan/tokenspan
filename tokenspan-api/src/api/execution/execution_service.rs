use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::execution::dto::{ExecutionArgs, ExecutionCreateInput};
use crate::api::execution::execution_error::ExecutionError;
use crate::api::execution::execution_model::Execution;

#[async_trait::async_trait]
pub trait ExecutionServiceExt {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Execution>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Execution>>;
    async fn create(&self, input: ExecutionCreateInput, executor_id: Uuid) -> Result<Execution>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Execution>;
}

pub type ExecutionServiceDyn = Arc<dyn ExecutionServiceExt + Send + Sync>;

pub struct ExecutionService {
    db: DatabaseConnection,
}

impl ExecutionService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ExecutionServiceExt for ExecutionService {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>> {
        let take = args.take.unwrap_or(10);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::execution::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::execution::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::execution::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::execution::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::execution::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Execution>> {
        let execution = entity::execution::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .map(|execution| execution.into());

        Ok(execution)
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Execution>> {
        let executions = entity::execution::Entity::find()
            .filter(entity::execution::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect();

        Ok(executions)
    }

    async fn create(&self, input: ExecutionCreateInput, executor_id: Uuid) -> Result<Execution> {
        let usage = if let Some(usage) = input.usage {
            let value = serde_json::to_value(usage).map_err(|e| {
                ExecutionError::Unknown(anyhow::anyhow!("Failed to serialize usage: {}", e))
            })?;

            Some(value)
        } else {
            None
        };

        let elapsed = serde_json::to_value(input.elapsed)?;
        let messages = serde_json::to_value(input.messages.clone())?;
        let parameter = serde_json::to_value(input.parameter.clone())?;
        let created_execution = entity::execution::ActiveModel {
            id: Set(Uuid::new_v4()),
            task_id: Set(input.task_id.into()),
            executor_id: Set(executor_id.into()),
            task_version_id: Set(input.task_version_id.into()),
            usage: Set(usage),
            error: Set(input.error),
            elapsed: Set(elapsed),
            messages: Set(messages),
            parameter: Set(parameter),
            output: Set(input.output),
            status: Set(input.status.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_execution)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Execution> {
        let execution = entity::execution::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ExecutionError::Unknown(anyhow::anyhow!(
                "Execution not found"
            )))?;

        execution
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?;

        Ok(execution.into())
    }
}

impl From<ExecutionService> for ExecutionServiceDyn {
    fn from(value: ExecutionService) -> Self {
        Arc::new(value) as Self
    }
}
