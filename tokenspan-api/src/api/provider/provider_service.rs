use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use crate::api::models::ProviderId;
use crate::api::provider::dto::{CreateProviderInput, ProviderArgs, UpdateProviderInput};
use crate::api::provider::provider_error::ProviderError;
use crate::api::provider::provider_model::Provider;
use crate::prisma::{provider, PrismaClient};
use tokenspan_utils::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn get_providers(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>>;
    async fn get_provider_by_id(&self, id: ProviderId) -> Result<Option<Provider>>;
    async fn get_providers_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>>;
    async fn count_providers(&self) -> Result<i64>;
    async fn create_provider(&self, input: CreateProviderInput) -> Result<Provider>;
    async fn update_provider(&self, id: ProviderId, input: UpdateProviderInput)
        -> Result<Provider>;
    async fn delete_provider(&self, id: ProviderId) -> Result<Provider>;
}

pub type ProviderServiceDyn = Arc<dyn ProviderServiceExt + Send + Sync>;

pub struct ProviderService {
    prisma: Arc<PrismaClient>,
}

impl ProviderService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn get_providers(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .provider()
            .find_many(vec![])
            .take(take + 1)
            .order_by(provider::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take((take + 2) * -1)
                .cursor(provider::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(provider::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToGetProviders)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_provider_by_id(&self, id: ProviderId) -> Result<Option<Provider>> {
        let provider = self
            .prisma
            .provider()
            .find_unique(provider::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToGetProvider)?
            .map(|provider| provider.into());

        Ok(provider)
    }

    async fn get_providers_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>> {
        let ids = ids
            .into_iter()
            .map(|id| provider::id::equals(id.into()))
            .collect();
        let providers = self
            .prisma
            .provider()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToGetProviders)?
            .into_iter()
            .map(|provider| provider.into())
            .collect();

        Ok(providers)
    }

    async fn count_providers(&self) -> Result<i64> {
        let count = self
            .prisma
            .provider()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToCountProviders)?;

        Ok(count)
    }

    async fn create_provider(&self, input: CreateProviderInput) -> Result<Provider> {
        let created_provider = self
            .prisma
            .provider()
            .create(input.name, vec![])
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToCreateProvider)?;

        Ok(created_provider.into())
    }

    async fn update_provider(
        &self,
        id: ProviderId,
        input: UpdateProviderInput,
    ) -> Result<Provider> {
        let updated_provider = self
            .prisma
            .provider()
            .update(provider::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToUpdateProvider)?;

        Ok(updated_provider.into())
    }

    async fn delete_provider(&self, id: ProviderId) -> Result<Provider> {
        let deleted_provider = self
            .prisma
            .provider()
            .delete(provider::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ProviderError::UnableToDeleteProvider)?;

        Ok(deleted_provider.into())
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
