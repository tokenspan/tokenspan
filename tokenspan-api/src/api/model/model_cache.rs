use std::collections::HashMap;
use std::sync::Arc;

use crate::api::dto::ModelArgs;
use crate::api::models::{Model, ModelId};
use crate::api::services::ModelServiceDyn;

#[derive(Clone)]
pub struct ModelCache {
    cache: HashMap<ModelId, Model>,
}

pub type ModelCacheDyn = Arc<ModelCache>;

impl ModelCache {
    pub async fn new(model_service: ModelServiceDyn) -> anyhow::Result<Self> {
        let models = model_service
            .paginate(ModelArgs {
                take: Some(100),
                ..Default::default()
            })
            .await
            .map_err(|e| anyhow::anyhow!(e.message))?;

        let mut cache = HashMap::new();
        for model in models.items.into_iter() {
            cache.insert(model.id.clone(), model);
        }

        Ok(Self { cache })
    }

    pub fn get(&self, id: ModelId) -> Option<Model> {
        self.cache.get(&id).cloned()
    }
}
