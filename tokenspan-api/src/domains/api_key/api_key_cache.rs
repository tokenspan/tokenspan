use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domains::cache::CacheExt;
use crate::domains::dto::ApiKeyArgs;
use crate::domains::services::ApiKeyServiceDyn;

#[derive(Clone)]
pub struct ApiKeyCache {
    cache: Arc<Mutex<HashMap<Uuid, String>>>,
}

pub type ApiKeyCacheDyn = Arc<dyn CacheExt<Uuid, String> + Send + Sync>;

impl ApiKeyCache {
    pub async fn new(api_key_service: ApiKeyServiceDyn) -> Result<Self> {
        let keys = api_key_service
            .paginate(ApiKeyArgs {
                last: Some(100),
                ..Default::default()
            })
            .await?;

        let mut cache = HashMap::new();
        for key in keys.items {
            cache.insert(key.id, api_key_service.decrypt(&key.key)?);
        }

        Ok(Self {
            cache: Arc::new(Mutex::new(cache)),
        })
    }
}

#[async_trait]
impl CacheExt<Uuid, String> for ApiKeyCache {
    async fn set(&self, key: Uuid, value: String) {
        let mut cache = self.cache.lock().await;
        cache.insert(key, value);
    }

    async fn get(&self, key: Uuid) -> Option<String> {
        let cache = self.cache.lock().await;
        cache.get(&key).cloned()
    }
}

impl From<ApiKeyCache> for ApiKeyCacheDyn {
    fn from(value: ApiKeyCache) -> Self {
        Arc::new(value) as Self
    }
}
