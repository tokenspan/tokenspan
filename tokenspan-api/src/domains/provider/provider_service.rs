use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::prelude::*;
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::domains::models::Provider;
use crate::domains::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Provider>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Provider>>;
    async fn find_by_slug(&self, slug: &String) -> Result<Option<Provider>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Provider>>;
    async fn create(&self, input: ProviderCreateInput) -> Result<Provider>;
    async fn update_by_id(&self, id: &Uuid, input: ProviderUpdateInput) -> Result<Provider>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<Provider>;
}

pub type ProviderServiceDyn = Arc<dyn ProviderServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ProviderService {
    db: Database,
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Provider>> {
        self.db
            .bind::<Provider>()
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Provider>> {
        self.db
            .bind::<Provider>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_slug(&self, slug: &String) -> Result<Option<Provider>> {
        self.db
            .bind::<Provider>()
            .where_by(equals("slug", &slug))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<Provider>> {
        self.db
            .bind::<Provider>()
            .where_by(in_list("id", &ids))
            .all()
            .await
    }

    async fn create(&self, input: ProviderCreateInput) -> Result<Provider> {
        let input = Provider {
            id: Uuid::new_v4(),
            name: input.name,
            slug: input.slug,
            base_url: input.base_url,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).exec().await
    }

    async fn update_by_id(&self, id: &Uuid, input: ProviderUpdateInput) -> Result<Provider> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<Provider> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
