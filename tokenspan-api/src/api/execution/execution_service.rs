use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::predicates::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::execution::dto::{ExecutionArgs, ExecutionCreateInput};
use crate::api::execution::execution_model::Execution;
use crate::api::services::MessageServiceDyn;

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
    message_service: MessageServiceDyn,
}

#[async_trait::async_trait]
impl ExecutionServiceExt for ExecutionService {
    async fn paginate(&self, args: ExecutionArgs) -> Result<Pagination<Execution>> {
        self.db
            .bind::<Execution>()
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
        let elapsed = input.elapsed.into();
        let usage = input.usage.map(|u| u.into());

        let messages = self
            .message_service
            .find_by_thread_version_id(&input.thread_version_id)
            .await?;

        let input = Execution {
            id: Uuid::new_v4(),
            thread_version_id: input.thread_version_id,
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

    async fn delete_by_id(&self, id: &Uuid) -> Result<Execution> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<ExecutionService> for ExecutionServiceDyn {
    fn from(value: ExecutionService) -> Self {
        Arc::new(value) as Self
    }
}
