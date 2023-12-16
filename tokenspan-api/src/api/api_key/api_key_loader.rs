use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::{DataLoader, Loader};

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::models::{ApiKey, ApiKeyId};
use crate::api::services::ApiKeyServiceDyn;

pub struct ApiKeyLoader {
    pub api_key_service: ApiKeyServiceDyn,
}

impl ApiKeyLoader {
    pub fn new(api_key_service: ApiKeyServiceDyn) -> Self {
        Self { api_key_service }
    }
}

#[async_trait::async_trait]
impl Loader<ApiKeyId> for ApiKeyLoader {
    type Value = ApiKey;
    type Error = Arc<ApiKeyError>;

    async fn load(&self, keys: &[ApiKeyId]) -> Result<HashMap<ApiKeyId, Self::Value>, Self::Error> {
        let api_keys = self
            .api_key_service
            .find_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ApiKeyError::Unknown(anyhow::anyhow!(e))))?
            .into_iter()
            .map(|api_key| (api_key.id.clone(), api_key))
            .collect();

        Ok(api_keys)
    }
}

impl From<ApiKeyLoader> for DataLoader<ApiKeyLoader> {
    fn from(api_key_loader: ApiKeyLoader) -> Self {
        Self::new(api_key_loader, tokio::spawn)
    }
}
