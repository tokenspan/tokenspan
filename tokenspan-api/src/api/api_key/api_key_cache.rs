use std::collections::HashMap;
use std::sync::Arc;

use crate::api::cache::CacheExt;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::api::dto::ApiKeyArgs;
use crate::api::models::ApiKeyId;
use crate::api::services::ApiKeyServiceDyn;

#[derive(Clone)]
pub struct ApiKeyCache {
    cache: Arc<Mutex<HashMap<ApiKeyId, String>>>,
}

pub type ApiKeyCacheDyn = Arc<dyn CacheExt<ApiKeyId, String> + Send + Sync>;

impl ApiKeyCache {
    pub async fn new(api_key_service: ApiKeyServiceDyn) -> Result<Self> {
        let keys = api_key_service
            .paginate(ApiKeyArgs {
                take: Some(100),
                ..Default::default()
            })
            .await?;

        let mut cache = HashMap::new();
        for key in keys.items {
            cache.insert(key.id, api_key_service.decrypt(key.key));
        }

        Ok(Self {
            cache: Arc::new(Mutex::new(cache)),
        })
    }
}

#[async_trait]
impl CacheExt<ApiKeyId, String> for ApiKeyCache {
    async fn set(&self, key: ApiKeyId, value: String) {
        let mut cache = self.cache.lock().await;
        cache.insert(key, value);
    }

    async fn get(&self, key: ApiKeyId) -> Option<String> {
        let cache = self.cache.lock().await;
        cache.get(&key).cloned()
    }
}

impl From<ApiKeyCache> for ApiKeyCacheDyn {
    fn from(value: ApiKeyCache) -> Self {
        Arc::new(value) as Self
    }
}
