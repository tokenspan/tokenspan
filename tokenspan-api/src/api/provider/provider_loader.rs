use std::collections::HashMap;

use async_graphql::dataloader::Loader;

use crate::api::models::{Provider, ProviderId};
use crate::api::provider::provider_error::ProviderError;
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ProviderId> for AppLoader {
    type Value = Provider;
    type Error = ProviderError;

    async fn load(
        &self,
        keys: &[ProviderId],
    ) -> Result<HashMap<ProviderId, Self::Value>, Self::Error> {
        let providers = self
            .provider_service
            .get_providers_by_ids(keys.to_vec())
            .await
            .map_err(|_| ProviderError::UnableToGetProviders)?
            .into_iter()
            .map(|provider| (provider.id.clone(), provider))
            .collect();

        Ok(providers)
    }
}
