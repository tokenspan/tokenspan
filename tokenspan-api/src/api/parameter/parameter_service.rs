use std::sync::Arc;

use async_graphql::Result;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::models::ParameterId;
use crate::api::parameter::dto::{ParameterArgs, ParameterCreateInput, ParameterUpdateInput};
use crate::api::parameter::parameter_error::ParameterError;
use crate::api::parameter::parameter_model::Parameter;
use crate::api::repositories::{ParameterCreateEntity, ParameterUpdateEntity};
use crate::repository::RootRepository;

#[async_trait::async_trait]
pub trait ParameterServiceExt {
    async fn get_parameters(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>>;
    async fn get_parameter_by_id(&self, id: ParameterId) -> Result<Option<Parameter>>;
    async fn get_parameters_by_ids(&self, ids: Vec<ParameterId>) -> Result<Vec<Parameter>>;
    async fn count_parameters(&self) -> Result<u64>;
    async fn create_parameter(&self, input: ParameterCreateInput) -> Result<Parameter>;
    async fn update_parameter(
        &self,
        id: ParameterId,
        input: ParameterUpdateInput,
    ) -> Result<Option<Parameter>>;
    async fn delete_parameter(&self, id: ParameterId) -> Result<Option<Parameter>>;
}

pub type ParameterServiceDyn = Arc<dyn ParameterServiceExt + Send + Sync>;

pub struct ParameterService {
    repository: Arc<RootRepository>,
}

impl ParameterService {
    pub fn new(repository: Arc<RootRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ParameterServiceExt for ParameterService {
    async fn get_parameters(&self, args: ParameterArgs) -> Result<Pagination<Cursor, Parameter>> {
        let paginated = self
            .repository
            .parameter
            .paginate::<Parameter>(args.into())
            .await
            .map_err(|_| ParameterError::UnableToGetParameters)?;

        Ok(paginated)
    }

    async fn get_parameter_by_id(&self, id: ParameterId) -> Result<Option<Parameter>> {
        let parameter = self
            .repository
            .parameter
            .find_by_id(id)
            .await
            .map_err(|_| ParameterError::UnableToGetParameter)?
            .map(|parameter| parameter.into());

        Ok(parameter)
    }

    async fn get_parameters_by_ids(&self, ids: Vec<ParameterId>) -> Result<Vec<Parameter>> {
        let parameters = self
            .repository
            .parameter
            .find_many_by_ids(ids)
            .await
            .map_err(|_| ParameterError::UnableToGetParameters)?
            .into_iter()
            .map(|parameter| parameter.into())
            .collect();

        Ok(parameters)
    }

    async fn count_parameters(&self) -> Result<u64> {
        let count = self
            .repository
            .parameter
            .count()
            .await
            .map_err(|_| ParameterError::UnableToCountParameters)?;

        Ok(count)
    }

    async fn create_parameter(&self, input: ParameterCreateInput) -> Result<Parameter> {
        let created_parameter = self
            .repository
            .parameter
            .create(ParameterCreateEntity {
                model_id: input.model_id,
                name: input.name,
                temperature: 3f32,
                max_tokens: input.max_tokens,
                stop_sequences: input.stop_sequences,
                top_p: 3f32,
                frequency_penalty: 3f32,
                presence_penalty: 3f32,
                extra: input.extra,
            })
            .await
            .map_err(|_| ParameterError::UnableToCreateParameter)?;

        Ok(created_parameter.into())
    }

    async fn update_parameter(
        &self,
        id: ParameterId,
        input: ParameterUpdateInput,
    ) -> Result<Option<Parameter>> {
        let updated_parameter = self
            .repository
            .parameter
            .update_by_id(
                id,
                ParameterUpdateEntity {
                    name: input.name,
                    temperature: input.temperature,
                    max_tokens: input.max_tokens,
                    stop_sequences: input.stop_sequences,
                    top_p: input.top_p,
                    frequency_penalty: input.frequency_penalty,
                    presence_penalty: input.presence_penalty,
                    extra: input.extra,
                },
            )
            .await
            .map_err(|_| ParameterError::UnableToUpdateParameter)?
            .map(|parameter| parameter.into());

        Ok(updated_parameter)
    }

    async fn delete_parameter(&self, id: ParameterId) -> Result<Option<Parameter>> {
        let deleted_parameter = self
            .repository
            .parameter
            .delete_by_id(id)
            .await
            .map_err(|_| ParameterError::UnableToDeleteParameter)?
            .map(|parameter| parameter.into());

        Ok(deleted_parameter)
    }
}

impl From<ParameterService> for ParameterServiceDyn {
    fn from(value: ParameterService) -> Self {
        Arc::new(value) as Self
    }
}
