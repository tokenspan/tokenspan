use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domains::cache::CacheExt;
use crate::domains::dto::ModelArgs;
use crate::domains::models::Model;
use crate::domains::services::ModelServiceDyn;

#[derive(Clone)]
pub struct ModelCache {
    cache: Arc<Mutex<HashMap<Uuid, Model>>>,
}

pub type ModelCacheDyn = Arc<dyn CacheExt<Uuid, Model> + Send + Sync>;

impl ModelCache {
    pub async fn new(model_service: ModelServiceDyn) -> Result<Self> {
        let models = model_service
            .paginate(ModelArgs {
                last: Some(100),
                ..Default::default()
            })
            .await?;

        let mut cache = HashMap::new();
        for model in models.items.into_iter() {
            cache.insert(model.id.clone(), model);
        }

        Ok(Self {
            cache: Arc::new(Mutex::new(cache)),
        })
    }
}

#[async_trait]
impl CacheExt<Uuid, Model> for ModelCache {
    async fn set(&self, key: Uuid, value: Model) {
        let mut cache = self.cache.lock().await;
        cache.insert(key, value);
    }

    async fn get(&self, key: Uuid) -> Option<Model> {
        let cache = self.cache.lock().await;
        cache.get(&key).cloned()
    }
}

impl From<ModelCache> for ModelCacheDyn {
    fn from(value: ModelCache) -> Self {
        Arc::new(value) as Self
    }
}
