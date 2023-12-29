use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::ops::{and, eq, in_list};
use dojo_orm::pagination::{Cursor, Pagination};
use dojo_orm::Database;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::models::Provider;
use crate::api::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Provider>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Provider>>;
    async fn create(&self, input: ProviderCreateInput) -> Result<Provider>;
    async fn update_by_id(&self, id: Uuid, input: ProviderUpdateInput) -> Result<Option<Provider>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Provider>>;
}

pub type ProviderServiceDyn = Arc<dyn ProviderServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ProviderService {
    db: Database,
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>> {
        self.db
            .bind::<Provider>()
            .cursor(&args.before, &args.after)
            .limit(args.take.unwrap_or(10))
            .all()
            .await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>> {
        self.db
            .bind::<Provider>()
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Provider>> {
        self.db
            .bind::<Provider>()
            .where_by(and(vec![eq("slug", &slug)]))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Provider>> {
        self.db
            .bind::<Provider>()
            .where_by(and(vec![in_list("id", &ids)]))
            .all()
            .await
    }

    async fn create(&self, input: ProviderCreateInput) -> Result<Provider> {
        let input = Provider {
            id: Uuid::new_v4(),
            name: input.name,
            slug: input.slug,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db.insert(&input).await
    }

    async fn update_by_id(&self, id: Uuid, input: ProviderUpdateInput) -> Result<Option<Provider>> {
        self.db
            .update(&input)
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<Provider>> {
        self.db
            .delete()
            .where_by(and(vec![eq("id", &id)]))
            .first()
            .await
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
