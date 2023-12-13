use std::collections::HashMap;
use std::sync::Arc;

use async_graphql::dataloader::Loader;

use crate::api::models::{Parameter, ParameterId};
use crate::api::parameter::parameter_error::ParameterError;
use crate::loader::AppLoader;

#[async_trait::async_trait]
impl Loader<ParameterId> for AppLoader {
    type Value = Parameter;
    type Error = Arc<ParameterError>;

    async fn load(
        &self,
        keys: &[ParameterId],
    ) -> Result<HashMap<ParameterId, Self::Value>, Self::Error> {
        let parameters = self
            .parameter_service
            .get_parameters_by_ids(keys.to_vec())
            .await
            .map_err(|e| Arc::new(ParameterError::Unknown(anyhow::anyhow!(e.message))))?
            .into_iter()
            .map(|parameter| (parameter.id.clone(), parameter))
            .collect();

        Ok(parameters)
    }
}
