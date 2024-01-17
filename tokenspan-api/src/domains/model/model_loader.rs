use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::domains::model::model_error::ModelError;
use crate::domains::models::Model;
use crate::domains::services::ModelServiceDyn;

pub struct ModelLoader {
    pub model_service: ModelServiceDyn,
}

impl ModelLoader {
    pub fn new(model_service: ModelServiceDyn) -> Self {
        Self { model_service }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ModelLoader {
    type Value = Model;
    type Error = Arc<ModelError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let models = self
            .model_service
            .find_by_ids(keys)
            .await
            .map_err(|e| Arc::new(ModelError::Unknown(e)))?
            .into_iter()
            .map(|model| (model.id.clone(), model))
            .collect();

        Ok(models)
    }
}
