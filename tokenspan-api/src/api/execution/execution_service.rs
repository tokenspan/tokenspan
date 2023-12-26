use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use rabbit_orm::pagination::{Cursor, Pagination};
use rabbit_orm::{Db, Order};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::execution::dto::{ExecutionArgs, ExecutionCreateInput};
use crate::api::execution::execution_model::Execution;

#[async_trait::async_trait]
pub trait ExecutionServiceExt {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Execution>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Execution>>;
    async fn create(&self, input: ExecutionCreateInput, executor_id: Uuid) -> Result<Execution>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Execution>>;
}

pub type ExecutionServiceDyn = Arc<dyn ExecutionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ExecutionService {
    db: Db,
}

#[async_trait::async_trait]
impl ExecutionServiceExt for ExecutionService {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>> {
        self.db
            .clone()
            .from::<Execution>()
            .select_all()
            .cursor(args.before, args.after)
            .order_by("created_at", Order::Desc)
            .limit(args.take.unwrap_or(10))
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Execution>> {
        self.db
            .clone()
            .from::<Execution>()
            .select_all()
            .find(id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Execution>> {
        self.db
            .clone()
            .from::<Execution>()
            .select_all()
            .and_where("id", "in", ids)
            .order_by("created_at", Order::Desc)
            .all()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn create(&self, input: ExecutionCreateInput, executor_id: Uuid) -> Result<Execution> {
        let messages = input.messages.into_iter().map(|m| m.into()).collect();
        let elapsed = input.elapsed.into();
        let usage = input.usage.map(|u| u.into());

        let input = Execution {
            id: Uuid::new_v4(),
            task_version_id: input.task_version_id,
            executed_by_id: executor_id,
            parameter_id: input.parameter_id,
            messages,
            elapsed,
            usage,
            output: input.output,
            error: input.error,
            status: input.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db
            .clone()
            .from::<Execution>()
            .insert(input)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Execution>> {
        self.db
            .clone()
            .from()
            .delete()
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}

impl From<ExecutionService> for ExecutionServiceDyn {
    fn from(value: ExecutionService) -> Self {
        Arc::new(value) as Self
    }
}
