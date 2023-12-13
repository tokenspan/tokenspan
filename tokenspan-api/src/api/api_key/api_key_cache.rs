use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::FromRef;

use crate::api::dto::ApiKeyArgs;
use crate::api::models::ApiKeyId;
use crate::api::services::ApiKeyServiceDyn;
use crate::state::AppState;

#[derive(Clone)]
pub struct ApiKeyCache {
    cache: HashMap<ApiKeyId, String>,
}

pub type ApiKeyCacheDyn = Arc<ApiKeyCache>;

impl FromRef<AppState> for ApiKeyCacheDyn {
    fn from_ref(input: &AppState) -> Self {
        input.api_key_cache.clone()
    }
}

impl ApiKeyCache {
    pub async fn new(api_key_service: ApiKeyServiceDyn) -> anyhow::Result<Self> {
        let keys = api_key_service
            .get_api_keys(ApiKeyArgs {
                take: Some(100),
                ..Default::default()
            })
            .await
            .map_err(|e| anyhow::anyhow!(e.message))?;

        let mut cache = HashMap::new();
        for key in keys.items {
            cache.insert(key.id, api_key_service.decrypt(key.key));
        }

        Ok(Self { cache })
    }

    pub fn get(&self, id: ApiKeyId) -> Option<String> {
        self.cache.get(&id).cloned()
    }
}
