use std::sync::Arc;

use async_graphql::Result;

use crate::api::models::ProviderId;
use crate::api::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};
use crate::api::provider::provider_error::ProviderError;
use crate::api::provider::provider_model::Provider;
use crate::api::repositories::{ProviderCreateEntity, ProviderUpdateEntity};
use crate::repository::RootRepository;
use tokenspan_utils::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn get_providers(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>>;
    async fn get_provider_by_id(&self, id: ProviderId) -> Result<Option<Provider>>;
    async fn get_providers_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>>;
    async fn count_providers(&self) -> Result<i64>;
    async fn create_provider(&self, input: ProviderCreateInput) -> Result<Provider>;
    async fn update_provider(&self, id: ProviderId, input: ProviderUpdateInput)
        -> Result<Provider>;
    async fn delete_provider(&self, id: ProviderId) -> Result<Option<Provider>>;
}

pub type ProviderServiceDyn = Arc<dyn ProviderServiceExt + Send + Sync>;

pub struct ProviderService {
    repository: Arc<RootRepository>,
}

impl ProviderService {
    pub fn new(repository: Arc<RootRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ProviderServiceExt for ProviderService {
    async fn get_providers(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>> {
        let paginated = self
            .repository
            .view
            .paginate::<Provider>(args.take, args.before, args.after)
            .await
            .map_err(|_| ProviderError::UnableToGetProviders)?;

        Ok(paginated)
    }

    async fn get_provider_by_id(&self, id: ProviderId) -> Result<Option<Provider>> {
        let provider = self
            .repository
            .provider
            .find_by_id(id)
            .await
            .map_err(|_| ProviderError::UnableToGetProvider)?
            .map(|provider| provider.into());

        Ok(provider)
    }

    async fn get_providers_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>> {
        let providers = self
            .repository
            .provider
            .find_many_by_ids(ids)
            .await
            .map_err(|_| ProviderError::UnableToGetProviders)?
            .into_iter()
            .map(|provider| provider.into())
            .collect();

        Ok(providers)
    }

    async fn count_providers(&self) -> Result<u64> {
        let count = self
            .repository
            .provider
            .count()
            .await
            .map_err(|_| ProviderError::UnableToCountProviders)?;

        Ok(count)
    }

    async fn create_provider(&self, input: ProviderCreateInput) -> Result<Provider> {
        let created_provider = self
            .repository
            .provider
            .create(ProviderCreateEntity { name: input.name })
            .await
            .map_err(|_| ProviderError::UnableToCreateProvider)?;

        Ok(created_provider.into())
    }

    async fn update_provider(
        &self,
        id: ProviderId,
        input: ProviderUpdateInput,
    ) -> Result<Provider> {
        let updated_provider = self
            .repository
            .provider
            .update_by_id(id, ProviderUpdateEntity { name: input.name })
            .await
            .map_err(|_| ProviderError::UnableToUpdateProvider)?;

        Ok(updated_provider.into())
    }

    async fn delete_provider(&self, id: ProviderId) -> Result<Option<Provider>> {
        let deleted_provider = self
            .repository
            .provider
            .delete_by_id(id)
            .await
            .map_err(|_| ProviderError::UnableToDeleteProvider)?
            .map(|provider| provider.into());

        Ok(deleted_provider)
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
