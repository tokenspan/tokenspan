use std::sync::Arc;

use async_graphql::Result;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_error::ModelError;
use crate::api::model::model_model::Model;
use crate::api::models::ModelId;
use crate::api::repositories::{ModelCreateEntity, ModelUpdateEntity};
use crate::repository::RootRepository;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn get_models(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn get_model_by_id(&self, id: ModelId) -> Result<Option<Model>>;
    async fn get_models_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>>;
    async fn get_model_by_name(&self, name: String) -> Result<Option<Model>>;
    async fn count_models(&self) -> Result<u64>;
    async fn create_model(&self, input: ModelCreateInput) -> Result<Model>;
    async fn update_model(&self, id: ModelId, input: ModelUpdateInput) -> Result<Option<Model>>;
    async fn delete_model(&self, id: ModelId) -> Result<Option<Model>>;
}

pub type ModelServiceDyn = Arc<dyn ModelServiceExt + Send + Sync>;

pub struct ModelService {
    repository: Arc<RootRepository>,
}

impl ModelService {
    pub fn new(repository: Arc<RootRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait::async_trait]
impl ModelServiceExt for ModelService {
    async fn get_models(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        let paginated = self
            .repository
            .model
            .paginate::<Model>(args.into())
            .await
            .map_err(|_| ModelError::UnableToGetModels)?;

        Ok(paginated)
    }

    async fn get_model_by_id(&self, id: ModelId) -> Result<Option<Model>> {
        let model = self
            .repository
            .model
            .find_by_id(id)
            .await
            .map_err(|_| ModelError::UnableToGetModel)?
            .map(|model| model.into());

        Ok(model)
    }

    async fn get_models_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>> {
        let models = self
            .repository
            .model
            .find_many_by_ids(ids)
            .await
            .map_err(|_| ModelError::UnableToGetModels)?
            .into_iter()
            .map(|model| model.into())
            .collect();

        Ok(models)
    }

    async fn get_model_by_name(&self, name: String) -> Result<Option<Model>> {
        let model = self
            .repository
            .model
            .find_by_name(name)
            .await
            .map_err(|_| ModelError::UnableToGetModel)?
            .map(|model| model.into());

        Ok(model)
    }

    async fn count_models(&self) -> Result<u64> {
        let count = self
            .repository
            .model
            .count()
            .await
            .map_err(|_| ModelError::UnableToCountModels)?;

        Ok(count)
    }

    async fn create_model(&self, input: ModelCreateInput) -> Result<Model> {
        let created_model = self
            .repository
            .model
            .create(ModelCreateEntity {
                provider_id: input.provider_id,
                name: input.name,
                description: input.description,
                context: input.context,
                input_pricing: input.input_pricing.into(),
                output_pricing: input.output_pricing.into(),
                training_at: input.training_at,
            })
            .await
            .map_err(|_| ModelError::UnableToCreateModel)?;

        Ok(created_model.into())
    }

    async fn update_model(&self, id: ModelId, input: ModelUpdateInput) -> Result<Option<Model>> {
        let updated_model = self
            .repository
            .model
            .update_by_id(
                id,
                ModelUpdateEntity {
                    name: input.name,
                    description: input.description,
                    context: input.context,
                    input_pricing: input.input_pricing.map(|pricing| pricing.into()),
                    output_pricing: input.output_pricing.map(|pricing| pricing.into()),
                    training_at: input.training_at,
                },
            )
            .await
            .map_err(|_| ModelError::UnableToUpdateModel)?
            .map(|model| model.into());

        Ok(updated_model)
    }

    async fn delete_model(&self, id: ModelId) -> Result<Option<Model>> {
        let deleted_model = self
            .repository
            .model
            .delete_by_id(id)
            .await
            .map_err(|_| ModelError::UnableToDeleteModel)?
            .map(|model| model.into());

        Ok(deleted_model)
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
