use std::sync::Arc;

use async_graphql::Result;
use axum::extract::FromRef;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::model::dto::{ModelArgs, ModelCreateInput, ModelUpdateInput};
use crate::api::model::model_error::ModelError;
use crate::api::model::model_model::Model;
use crate::api::models::ModelId;
use crate::api::repositories::{ModelCreateEntity, ModelUpdateEntity};
use crate::repository::RootRepository;
use crate::state::AppState;

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn find_by_id(&self, id: ModelId) -> Result<Option<Model>>;
    async fn find_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>>;
    async fn find_by_name(&self, name: String) -> Result<Option<Model>>;
    async fn count(&self) -> Result<u64>;
    async fn create(&self, input: ModelCreateInput) -> Result<Model>;
    async fn update_by_id(&self, id: ModelId, input: ModelUpdateInput) -> Result<Option<Model>>;
    async fn delete_by_id(&self, id: ModelId) -> Result<Option<Model>>;
}

pub type ModelServiceDyn = Arc<dyn ModelServiceExt + Send + Sync>;

impl FromRef<AppState> for ModelServiceDyn {
    fn from_ref(input: &AppState) -> Self {
        input.model_service.clone()
    }
}

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
    async fn paginate(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        let paginated = self
            .repository
            .model
            .paginate::<Model>(args.into())
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?;

        Ok(paginated)
    }

    async fn find_by_id(&self, id: ModelId) -> Result<Option<Model>> {
        let model = self
            .repository
            .model
            .find_by_id(id)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(model)
    }

    async fn find_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>> {
        let models = self
            .repository
            .model
            .find_many_by_ids(ids)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|model| model.into())
            .collect();

        Ok(models)
    }

    async fn find_by_name(&self, name: String) -> Result<Option<Model>> {
        let model = self
            .repository
            .model
            .find_by_name(name)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(model)
    }

    async fn count(&self) -> Result<u64> {
        let count = self
            .repository
            .model
            .count()
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?;

        Ok(count)
    }

    async fn create(&self, input: ModelCreateInput) -> Result<Model> {
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
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_model.into())
    }

    async fn update_by_id(&self, id: ModelId, input: ModelUpdateInput) -> Result<Option<Model>> {
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
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(updated_model)
    }

    async fn delete_by_id(&self, id: ModelId) -> Result<Option<Model>> {
        let deleted_model = self
            .repository
            .model
            .delete_by_id(id)
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?
            .map(|model| model.into());

        Ok(deleted_model)
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
