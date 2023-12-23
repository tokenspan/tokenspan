use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use rabbit_orm::pagination::{Cursor, Pagination};
use rabbit_orm::{Db, Order};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};
use crate::api::provider::provider_model::Provider;

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Provider>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Provider>>;
    async fn create(&self, input: ProviderCreateInput) -> Result<Provider>;
    async fn update_by_id(&self, id: Uuid, input: ProviderUpdateInput) -> Result<Provider>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Provider>;
}

pub type ProviderServiceDyn = Arc<dyn ProviderServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ProviderService {
    db: Db,
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>> {
        self.db
            .table::<Provider>()
            .limit(args.take.unwrap_or(10))
            .order_by("created_at", Order::Desc)
            .cursor_paginate(args.before, args.after)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>> {
        self.db
            .table::<Provider>()
            .find(id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Provider>> {
        self.db
            .table::<Provider>()
            .where_("slug", "=", slug)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Provider>> {
        self.db
            .table::<Provider>()
            .where_("id", "in", ids)
            .get()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn create(&self, input: ProviderCreateInput) -> Result<Provider> {
        let input = Provider {
            id: Uuid::new_v4(),
            name: input.name,
            slug: input.slug,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db
            .table::<Provider>()
            .insert(input)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn update_by_id(&self, id: Uuid, input: ProviderUpdateInput) -> Result<Provider> {
        self.db
            .table::<Provider>()
            .where_("id", "=", id)
            .update(input)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Provider> {
        self.db
            .table::<Provider>()
            .where_("id", "=", id)
            .delete()
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .first()
            .cloned()
            .ok_or(anyhow::anyhow!("Provider not found"))
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
