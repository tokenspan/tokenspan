use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::api::cache::CacheExt;
use crate::api::dto::ModelArgs;
use crate::api::models::{Model, ModelId};
use crate::api::services::ModelServiceDyn;

#[derive(Clone)]
pub struct ModelCache {
    cache: Arc<Mutex<HashMap<ModelId, Model>>>,
}

pub type ModelCacheDyn = Arc<dyn CacheExt<ModelId, Model> + Send + Sync>;

impl ModelCache {
    pub async fn new(model_service: ModelServiceDyn) -> Result<Self> {
        let models = model_service
            .paginate(ModelArgs {
                take: Some(100),
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
impl CacheExt<ModelId, Model> for ModelCache {
    async fn set(&self, key: ModelId, value: Model) {
        let mut cache = self.cache.lock().await;
        cache.insert(key, value);
    }

    async fn get(&self, key: ModelId) -> Option<Model> {
        let cache = self.cache.lock().await;
        cache.get(&key).cloned()
    }
}

impl From<ModelCache> for ModelCacheDyn {
    fn from(value: ModelCache) -> Self {
        Arc::new(value) as Self
    }
}
