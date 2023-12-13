use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::model::model_error::ModelError;
use crate::api::models::{Model, ModelId};
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ModelId> for AppLoader {
    type Value = Model;
    type Error = Arc<ModelError>;

    async fn load(&self, keys: &[ModelId]) -> Result<HashMap<ModelId, Self::Value>, Self::Error> {
        let models = self
            .model_service
            .get_models_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ModelError::Unknown(anyhow::anyhow!(e.message))))?
            .into_iter()
            .map(|model| (model.id.clone(), model))
            .collect();

        Ok(models)
    }
}
