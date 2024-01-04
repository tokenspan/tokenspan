use std::sync::Arc;

use anyhow::Result;
use axum::extract::FromRef;
use chrono::Utc;
use dojo_orm::ops::{and, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_model::Model;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>>;
    async fn find_first(&self) -> Result<Option<Model>>;
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
    db: Database,
}

#[async_trait::async_trait]
impl ModelServiceExt for ModelService {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        self.db
            .bind::<Model>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Model>> {
        self.db
            .bind::<Model>()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn find_first(&self) -> Result<Option<Model>> {
        self.db.bind::<Model>().first().await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Model>> {
        self.db
            .bind::<Model>()
            .where_by(and(&[in_list("id", &ids)]))
            .all()
            .await
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Model>> {
        self.db
            .bind::<Model>()
            .where_by(and(&[eq("slug", &slug)]))
            .first()
            .await
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

        self.db.insert(&input).await
    }

    async fn update_by_id(&self, id: Uuid, input: ModelUpdateInput) -> Result<Option<Model>> {
        self.db
            .update(&input)
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Model>> {
        self.db
            .delete()
            .where_by(and(&[eq("id", &id)]))
            .first()
            .await
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
