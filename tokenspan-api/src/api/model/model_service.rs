use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::model::dto::{CreateModelInput, ModelArgs, UpdateModelInput};
use crate::api::model::model_error::ModelError;
use crate::api::model::model_model::Model;
use crate::api::models::ModelId;
use crate::prisma::{model, provider, PrismaClient};

#[async_trait::async_trait]
pub trait ModelServiceExt {
    async fn get_models(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>>;
    async fn get_model_by_id(&self, id: ModelId) -> Result<Option<Model>>;
    async fn get_models_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>>;
    async fn count_models(&self) -> Result<i64>;
    async fn create_model(&self, input: CreateModelInput) -> Result<Model>;
    async fn update_model(&self, id: ModelId, input: UpdateModelInput) -> Result<Model>;
    async fn delete_model(&self, id: ModelId) -> Result<Model>;
}

pub type ModelServiceDyn = Arc<dyn ModelServiceExt + Send + Sync>;

pub struct ModelService {
    prisma: Arc<PrismaClient>,
}

impl ModelService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl ModelServiceExt for ModelService {
    async fn get_models(&self, args: ModelArgs) -> Result<Pagination<Cursor, Model>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .model()
            .find_many(vec![])
            .take(take + 1)
            .order_by(model::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take((take + 2) * -1)
                .cursor(model::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(model::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| ModelError::UnableToGetModels)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_model_by_id(&self, id: ModelId) -> Result<Option<Model>> {
        let model = self
            .prisma
            .model()
            .find_unique(model::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ModelError::UnableToGetModel)?
            .map(|model| model.into());

        Ok(model)
    }

    async fn get_models_by_ids(&self, ids: Vec<ModelId>) -> Result<Vec<Model>> {
        let ids = ids
            .into_iter()
            .map(|id| model::id::equals(id.into()))
            .collect();
        let models = self
            .prisma
            .model()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| ModelError::UnableToGetModels)?
            .into_iter()
            .map(|model| model.into())
            .collect();

        Ok(models)
    }

    async fn count_models(&self) -> Result<i64> {
        let count = self
            .prisma
            .model()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| ModelError::UnableToCountModels)?;

        Ok(count)
    }

    async fn create_model(&self, input: CreateModelInput) -> Result<Model> {
        let created_model = self
            .prisma
            .model()
            .create(
                provider::id::equals(input.provider_id.into()),
                input.name,
                input.description,
                input.context,
                input.pricing,
                vec![],
            )
            .exec()
            .await
            .map_err(|_| ModelError::UnableToCreateModel)?;

        Ok(created_model.into())
    }

    async fn update_model(&self, id: ModelId, input: UpdateModelInput) -> Result<Model> {
        let updated_model = self
            .prisma
            .model()
            .update(model::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| ModelError::UnableToUpdateModel)?;

        Ok(updated_model.into())
    }

    async fn delete_model(&self, id: ModelId) -> Result<Model> {
        let deleted_model = self
            .prisma
            .model()
            .delete(model::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ModelError::UnableToDeleteModel)?;

        Ok(deleted_model.into())
    }
}

impl From<ModelService> for ModelServiceDyn {
    fn from(value: ModelService) -> Self {
        Arc::new(value) as Self
    }
}
