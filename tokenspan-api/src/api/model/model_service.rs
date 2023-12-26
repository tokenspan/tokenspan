use std::sync::Arc;

use anyhow::Result;
use axum::extract::FromRef;
use chrono::Utc;
use futures_util::TryFutureExt;
use rabbit_orm::pagination::{Cursor, Pagination};
use rabbit_orm::{Db, Order};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_model::Model;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Model>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Model>>;
    async fn create(&self, input: ModelCreateInput) -> Result<Model>;
    async fn update_by_id(&self, id: Uuid, input: ModelUpdateInput) -> Result<Option<Model>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Model>>;
}

pub type ModelServiceDyn = Arc<dyn ModelServiceExt + Send + Sync>;

impl FromRef<AppState> for ModelServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.model_service.clone()
    }
}

#[derive(TypedBuilder)]
pub struct ModelService {
    db: Db,
}

#[async_trait::async_trait]
impl ModelServiceExt for ModelService {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        self.db
            .clone()
            .from::<Model>()
            .select_all()
            .cursor(args.before, args.after)
            .order_by("created_at", Order::Desc)
            .limit(args.take.unwrap_or(10))
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>> {
        self.db
            .clone()
            .from::<Model>()
            .select_all()
            .find(id)
            .map_err(|e| anyhow::anyhow!(e))
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Model>> {
        self.db
            .clone()
            .from::<Model>()
            .select_all()
            .and_where("id", "in", ids)
            .all()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Model>> {
        self.db
            .clone()
            .from::<Model>()
            .select_all()
            .and_where("slug", "=", slug)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn create(&self, input: ModelCreateInput) -> Result<Model> {
        let input = Model {
            id: Uuid::new_v4(),
            name: input.name,
            description: input.description,
            slug: input.slug,
            context: input.context,
            input_pricing: input.input_pricing.into(),
            output_pricing: input.output_pricing.into(),
            training_at: input.training_at,
            provider_id: input.provider_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db
            .clone()
            .from::<Model>()
            .insert(input)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn update_by_id(&self, id: Uuid, input: ModelUpdateInput) -> Result<Option<Model>> {
        self.db
            .clone()
            .from::<Model>()
            .update(input)
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Model>> {
        self.db
            .clone()
            .from::<Model>()
            .delete()
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
