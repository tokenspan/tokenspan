use std::sync::Arc;

use anyhow::Result;
use axum::extract::FromRef;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::prelude::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::domains::model::model_model::Model;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Model>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>>;
    async fn find_first(&self) -> Result<Option<Model>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Model>>;
    async fn find_by_slug(&self, slug: &String) -> Result<Option<Model>>;
    async fn create(&self, input: ModelCreateInput) -> Result<Model>;
    async fn update_by_id(&self, id: &Uuid, input: ModelUpdateInput) -> Result<Model>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<Model>;
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
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Model>> {
        let mut predicates: Vec<Predicate> = vec![];
        if let Some(where_args) = &args.r#where {
            if let Some(provider_id) = &where_args.provider_id {
                if let Some(id) = &provider_id.equals {
                    predicates.push(equals("provider_id", id));
                }
            }
        }

        self.db
            .bind::<Model>()
            .where_by(and(&predicates))
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Model>> {
        self.db
            .bind::<Model>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_first(&self) -> Result<Option<Model>> {
        self.db.bind::<Model>().first().await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Model>> {
        self.db
            .bind::<Model>()
            .where_by(in_list("id", &ids))
            .all()
            .await
    }

    async fn find_by_slug(&self, slug: &String) -> Result<Option<Model>> {
        self.db
            .bind::<Model>()
            .where_by(equals("slug", slug))
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

        self.db.insert(&input).exec().await
    }

    async fn update_by_id(&self, id: &Uuid, input: ModelUpdateInput) -> Result<Model> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<Model> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
