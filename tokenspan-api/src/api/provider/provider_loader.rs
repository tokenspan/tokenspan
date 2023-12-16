use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::api::models::{Provider, ProviderId};
use crate::api::provider::provider_error::ProviderError;
use crate::api::services::ProviderServiceDyn;

pub struct ProviderLoader {
    pub provider_service: ProviderServiceDyn,
}

impl ProviderLoader {
    pub fn new(provider_service: ProviderServiceDyn) -> Self {
        Self { provider_service }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ProviderLoader {
    type Value = Provider;
    type Error = Arc<ProviderError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let providers = self
            .provider_service
            .find_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ProviderError::Unknown(e)))?
            .into_iter()
            .map(|provider| (provider.id.clone(), provider))
            .collect();

        Ok(providers)
    }
}
