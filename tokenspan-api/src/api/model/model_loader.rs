use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::model::model_error::ModelError;
use crate::api::models::{Model, ModelId};
use crate::api::services::ModelServiceDyn;

pub struct ModelLoader {
    pub model_service: ModelServiceDyn,
}

impl ModelLoader {
    pub fn new(model_service: ModelServiceDyn) -> Self {
        Self { model_service }
    }
}

#[async_trait::async_trait]
impl Loader<ModelId> for ModelLoader {
    type Value = Model;
    type Error = Arc<ModelError>;

    async fn load(&self, keys: &[ModelId]) -> Result<HashMap<ModelId, Self::Value>, Self::Error> {
        let models = self
            .model_service
            .find_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ModelError::Unknown(e)))?
            .into_iter()
            .map(|model| (model.id.clone(), model))
            .collect();

        Ok(models)
    }
}
