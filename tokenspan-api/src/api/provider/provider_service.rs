use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use chrono::{DateTime, NaiveDateTime};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, IntoActiveModel,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::models::ProviderId;
use crate::api::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};
use crate::api::provider::provider_error::ProviderError;
use crate::api::provider::provider_model::Provider;

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>>;
    async fn find_by_id(&self, id: ProviderId) -> Result<Option<Provider>>;
    async fn find_by_slug(&self, slug: String) -> Result<Option<Provider>>;
    async fn find_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>>;
    async fn create(&self, input: ProviderCreateInput) -> Result<Provider>;
    async fn update_by_id(&self, id: ProviderId, input: ProviderUpdateInput) -> Result<Provider>;
    async fn delete_by_id(&self, id: ProviderId) -> Result<Provider>;
}

pub type ProviderServiceDyn = Arc<dyn ProviderServiceExt + Send + Sync>;

pub struct ProviderService {
    db: DatabaseConnection,
}

impl ProviderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>> {
        let take = args.take.unwrap_or(10) as u64;
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::provider::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::provider::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after = NaiveDateTime::from_timestamp_micros(after.id).unwrap();
            select = select.filter(entity::provider::Column::CreatedAt.lte(after));
            println!("{}", select.build(DbBackend::Postgres).to_string());
        }

        if let Some(before) = args.before.clone() {
            let before = NaiveDateTime::from_timestamp_micros(before.id).unwrap();
            select = select.filter(entity::provider::Column::CreatedAt.gte(before));
            println!("{}", select.build(DbBackend::Postgres).to_string());
        }

        let count = entity::provider::Entity::find().count(&self.db).await?;
        let items = select
            .all(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|provider| provider.into())
            .collect::<Vec<_>>();
        println!("items: {:?}", items);

        Ok(Pagination::new(
            items,
            args.before,
            args.after,
            take as i64,
            count,
        ))
    }

    async fn find_by_id(&self, id: ProviderId) -> Result<Option<Provider>> {
        let provider = entity::provider::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .map(|provider| provider.into());

        Ok(provider)
    }

    async fn find_by_slug(&self, slug: String) -> Result<Option<Provider>> {
        let provider = entity::provider::Entity::find()
            .filter(entity::provider::Column::Slug.eq(slug))
            .one(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .map(|provider| provider.into());

        Ok(provider)
    }

    async fn find_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>> {
        let ids = ids.into_iter().map(|id| id.to_string()).collect::<Vec<_>>();
        let providers = entity::provider::Entity::find()
            .filter(entity::provider::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|provider| provider.into())
            .collect();

        Ok(providers)
    }

    async fn create(&self, input: ProviderCreateInput) -> Result<Provider> {
        let created_provider = entity::provider::ActiveModel {
            id: Set(ProviderId::new_v4()),
            name: Set(input.name),
            slug: Set(input.slug),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_provider)
    }

    async fn update_by_id(&self, id: ProviderId, input: ProviderUpdateInput) -> Result<Provider> {
        let mut provider = entity::provider::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ProviderError::Unknown(anyhow::anyhow!(
                "Provider not found"
            )))?
            .into_active_model();

        provider.updated_at = Set(Utc::now().naive_utc());

        if let Some(name) = input.name {
            provider.name = Set(name);
        }

        if let Some(slug) = input.slug {
            provider.slug = Set(slug);
        }

        let updated_provider = provider
            .update(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(updated_provider)
    }

    async fn delete_by_id(&self, id: ProviderId) -> Result<Provider> {
        let provider = entity::provider::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ProviderError::Unknown(anyhow::anyhow!(
                "Provider not found"
            )))?;

        provider
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?;

        Ok(provider.into())
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
