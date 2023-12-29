use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::ops::{and, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::dto::{ParameterArgs, ParameterCreateInput, ParameterUpdateInput};
use crate::api::models::Parameter;

#[async_trait::async_trait]
pub trait ParameterServiceExt {
    async fn paginate(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Parameter>>;
    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Parameter>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Parameter>>;
    async fn create(&self, inputs: ParameterCreateInput) -> Result<Parameter>;
    async fn update_by_id(
        &self,
        id: Uuid,
        input: ParameterUpdateInput,
    ) -> Result<Option<Parameter>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Parameter>>;
}

pub type ParameterServiceDyn = Arc<dyn ParameterServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ParameterService {
    db: Database,
}

#[async_trait::async_trait]
impl ParameterServiceExt for ParameterService {
    async fn paginate(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>> {
        self.db
            .bind::<Parameter>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Parameter>> {
        self.db
            .bind::<Parameter>()
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_task_version_id(&self, id: Uuid) -> Result<Vec<Parameter>> {
        self.db
            .bind::<Parameter>()
            .where_by(and(vec![eq("task_version_id", &id)]))
            .all()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Parameter>> {
        self.db
            .bind::<Parameter>()
            .where_by(and(vec![in_list("id", &ids)]))
            .all()
            .await
    }

    async fn create(&self, input: ParameterCreateInput) -> Result<Parameter> {
        let input = Parameter {
            id: Uuid::new_v4(),
            name: input.name,
            stop_sequences: input.stop_sequences,
            model_id: input.model_id,
            temperature: input.temperature,
            max_tokens: input.max_tokens,
            top_p: input.top_p,
            frequency_penalty: input.frequency_penalty,
            presence_penalty: input.presence_penalty,
            extra: input.extra,
            task_version_id: input.task_version_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn update_by_id(
        &self,
        id: Uuid,
        input: ParameterUpdateInput,
    ) -> Result<Option<Parameter>> {
        self.db
            .update(&input)
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Parameter>> {
        self.db
            .delete()
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }
}

impl From<ParameterService> for ParameterServiceDyn {
    fn from(value: ParameterService) -> Self {
        Arc::new(value) as Self
    }
}
