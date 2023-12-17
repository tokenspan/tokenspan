use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;
use uuid::Uuid;

use crate::api::models::Parameter;
use crate::api::parameter::parameter_error::ParameterError;
use crate::api::services::ParameterServiceDyn;

pub struct ParameterLoader {
    pub parameter_service: ParameterServiceDyn,
}

impl ParameterLoader {
    pub fn new(parameter_service: ParameterServiceDyn) -> Self {
        Self { parameter_service }
    }
}

#[async_trait::async_trait]
impl Loader<Uuid> for ParameterLoader {
    type Value = Parameter;
    type Error = Arc<ParameterError>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let parameters = self
            .parameter_service
            .find_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ParameterError::Unknown(e)))?
            .into_iter()
            .map(|provider| (provider.id.clone(), provider))
            .collect();

        Ok(parameters)
    }
}
