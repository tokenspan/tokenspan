use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};
use crate::api::provider::provider_error::ProviderError;
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
    db: DatabaseConnection,
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn paginate(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>> {
        let take = args.take.unwrap_or(10);
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
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::provider::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::provider::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::provider::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|provider| provider.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Provider>> {
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

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<Provider>> {
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
            id: Set(Uuid::new_v4()),
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

    async fn update_by_id(&self, id: Uuid, input: ProviderUpdateInput) -> Result<Provider> {
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

    async fn delete_by_id(&self, id: Uuid) -> Result<Provider> {
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
