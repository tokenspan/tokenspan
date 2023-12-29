use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::ops::{and, eq};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
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
    db: Database,
}

#[async_trait::async_trait]
impl ExecutionServiceExt for ExecutionService {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Cursor, Execution>> {
        self.db
            .bind::<Execution>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Execution>> {
        self.db
            .bind::<Execution>()
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Execution>> {
        self.db
            .bind::<Execution>()
            .where_by(and(vec![eq("id", &ids)]))
            .all()
            .await
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

        self.db.insert(&input).await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Execution>> {
        self.db
            .delete()
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }
}

impl From<ExecutionService> for ExecutionServiceDyn {
    fn from(value: ExecutionService) -> Self {
        Arc::new(value) as Self
    }
}
