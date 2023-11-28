use std::collections::HashMap;

use async_graphql::dataloader::Loader;

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::models::{ApiKey, ApiKeyId};
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ApiKeyId> for AppLoader {
    type Value = ApiKey;
    type Error = ApiKeyError;

    async fn load(&self, keys: &[ApiKeyId]) -> Result<HashMap<ApiKeyId, Self::Value>, Self::Error> {
        let api_keys = self
            .api_key_service
            .get_api_keys_by_ids(keys.to_vec())
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKeys)?
            .into_iter()
            .map(|api_key| (api_key.id.clone(), api_key))
            .collect();

        Ok(api_keys)
    }
}
