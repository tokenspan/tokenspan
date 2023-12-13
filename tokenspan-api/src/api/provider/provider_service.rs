use std::sync::Arc;

use async_graphql::Result;

use crate::api::models::ProviderId;
use crate::api::provider::dto::{ProviderArgs, ProviderCreateInput, ProviderUpdateInput};
use crate::api::provider::provider_error::ProviderError;
use crate::api::provider::provider_model::Provider;
use crate::api::repositories::{ProviderCreateEntity, ProviderUpdateEntity};
use crate::repository::RootRepository;
use tokenspan_extra::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ProviderServiceExt {
    async fn get_providers(&self, args: ProviderArgs) -> Result<Pagination<Cursor, Provider>>;
    async fn get_provider_by_id(&self, id: ProviderId) -> Result<Option<Provider>>;
    async fn get_provider_by_name(&self, name: String) -> Result<Option<Provider>>;
    async fn get_providers_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>>;
    async fn count_providers(&self) -> Result<u64>;
    async fn create_provider(&self, input: ProviderCreateInput) -> Result<Provider>;
    async fn update_provider(
        &self,
        id: ProviderId,
        input: ProviderUpdateInput,
    ) -> Result<Option<Provider>>;
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
            .provider
            .paginate::<Provider>(args.into())
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn get_provider_by_id(&self, id: ProviderId) -> Result<Option<Provider>> {
        let provider = self
            .repository
            .provider
            .find_by_id(id)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .map(|provider| provider.into());

        Ok(provider)
    }

    async fn get_provider_by_name(&self, name: String) -> Result<Option<Provider>> {
        let provider = self
            .repository
            .provider
            .find_by_name(name)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .map(|provider| provider.into());

        Ok(provider)
    }

    async fn get_providers_by_ids(&self, ids: Vec<ProviderId>) -> Result<Vec<Provider>> {
        let providers = self
            .repository
            .provider
            .find_many_by_ids(ids)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
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
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?;

        Ok(count)
    }

    async fn create_provider(&self, input: ProviderCreateInput) -> Result<Provider> {
        let created_provider = self
            .repository
            .provider
            .create(ProviderCreateEntity { name: input.name })
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_provider.into())
    }

    async fn update_provider(
        &self,
        id: ProviderId,
        input: ProviderUpdateInput,
    ) -> Result<Option<Provider>> {
        let updated_provider = self
            .repository
            .provider
            .update_by_id(id, ProviderUpdateEntity { name: input.name })
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .map(|provider| provider.into());

        Ok(updated_provider)
    }

    async fn delete_provider(&self, id: ProviderId) -> Result<Option<Provider>> {
        let deleted_provider = self
            .repository
            .provider
            .delete_by_id(id)
            .await
            .map_err(|e| ProviderError::Unknown(anyhow::anyhow!(e)))?
            .map(|provider| provider.into());

        Ok(deleted_provider)
    }
}

impl From<ProviderService> for ProviderServiceDyn {
    fn from(value: ProviderService) -> Self {
        Arc::new(value) as Self
    }
}
