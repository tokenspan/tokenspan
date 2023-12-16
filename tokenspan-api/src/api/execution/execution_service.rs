use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::execution::dto::{ExecutionArgs, ExecutionCreateInput};
use crate::api::execution::execution_error::ExecutionError;
use crate::api::execution::execution_model::Execution;
use crate::api::models::{ExecutionId, UserId};

#[async_trait::async_trait]
pub trait ExecutionServiceExt {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>>;
    async fn find_by_id(&self, id: ExecutionId) -> Result<Option<Execution>>;
    async fn find_by_ids(&self, ids: Vec<ExecutionId>) -> Result<Vec<Execution>>;
    async fn create(&self, input: ExecutionCreateInput, executor_id: UserId) -> Result<Execution>;
    async fn delete_by_id(&self, id: ExecutionId) -> Result<Execution>;
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
        let take = args.take.unwrap_or(10) as u64;
        let mut cursor = entity::execution::Entity::find()
            .cursor_by(entity::execution::Column::Id)
            .order_by_desc(entity::execution::Column::Id)
            .limit(Some(take));

        if let Some(after) = args.after.clone() {
            cursor.after(after.id);
        }

        if let Some(before) = args.before.clone() {
            cursor.before(before.id);
        }

        let count = entity::execution::Entity::find().count(&self.db).await?;
        let items = cursor
            .all(&self.db)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|execution| execution.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(
            items,
            args.before,
            args.after,
            take as i64,
            count,
        ))
    }

    async fn find_by_id(&self, id: ExecutionId) -> Result<Option<Execution>> {
        let execution = entity::execution::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
            .map(|execution| execution.into());

        Ok(execution)
    }

    async fn find_by_ids(&self, ids: Vec<ExecutionId>) -> Result<Vec<Execution>> {
        let ids = ids.into_iter().map(|id| id.to_string()).collect::<Vec<_>>();
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

    async fn create(&self, input: ExecutionCreateInput, executor_id: UserId) -> Result<Execution> {
        let usage = if let Some(usage) = input.usage {
            let value = serde_json::to_value(usage).map_err(|e| {
                ExecutionError::Unknown(anyhow::anyhow!("Failed to serialize usage: {}", e))
            })?;

            Some(value)
        } else {
            None
        };

        let elapsed = serde_json::to_value(input.elapsed).map_err(|e| {
            ExecutionError::Unknown(anyhow::anyhow!("Failed to serialize elapsed: {}", e))
        })?;

        let created_execution = entity::execution::ActiveModel {
            id: Set(ExecutionId::new_v4()),
            task_id: Set(input.task_id.into()),
            executor_id: Set(executor_id.into()),
            task_version_id: Set(input.task_version_id.into()),
            usage: Set(usage),
            error: Set(input.error),
            elapsed: Set(elapsed),
            messages: Set(input.messages),
            parameter: Set(input.parameter),
            output: Set(input.output),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| ExecutionError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_execution)
    }

    async fn delete_by_id(&self, id: ExecutionId) -> Result<Execution> {
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
