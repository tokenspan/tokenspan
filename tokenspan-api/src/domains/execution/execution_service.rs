use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::predicates::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::execution::dto::{ExecutionArgs, ExecutionCreateInput};
use crate::domains::execution::execution_model::Execution;

#[async_trait::async_trait]
pub trait ExecutionServiceExt {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Execution>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Execution>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Execution>>;
    async fn create(&self, input: ExecutionCreateInput, executor_id: Uuid) -> Result<Execution>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<Execution>;
}

pub type ExecutionServiceDyn = Arc<dyn ExecutionServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ExecutionService {
    db: Database,
}

#[async_trait::async_trait]
impl ExecutionServiceExt for ExecutionService {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Execution>> {
        let mut predicates = vec![];
        if let Some(r#where) = &args.r#where {
            if let Some(thread_id_args) = &r#where.thread_id {
                if let Some(id) = &thread_id_args.equals {
                    predicates.push(equals("thread_id", id));
                }
            }
        }

        self.db
            .bind::<Execution>()
            .where_by(and(&predicates))
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Execution>> {
        self.db
            .bind::<Execution>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Execution>> {
        self.db
            .bind::<Execution>()
            .where_by(equals("id", &ids))
            .all()
            .await
    }

    async fn create(&self, input: ExecutionCreateInput, executor_id: Uuid) -> Result<Execution> {
        let input = Execution {
            id: Uuid::new_v4(),
            thread_version_id: input.thread_version_id,
            executed_by_id: executor_id,
            parameter: input.parameter,
            input_messages: input.input_messages,
            output_messages: input.output_messages,
            elapsed: input.elapsed,
            usage: input.usage,
            response: input.response,
            error: input.error,
            status: input.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).exec().await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<Execution> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<ExecutionService> for ExecutionServiceDyn {
    fn from(value: ExecutionService) -> Self {
        Arc::new(value) as Self
    }
}
