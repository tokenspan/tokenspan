use std::sync::Arc;

use async_graphql::Result;
use prisma_client_rust::Direction;

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyArgs, CreateApiKeyInput, UpdateApiKeyInput};
use crate::api::models::{ApiKeyId, UserId};
use crate::prisma::{api_key, provider, user, PrismaClient};
use tokenspan_utils::pagination::{Cursor, Pagination};

#[async_trait::async_trait]
pub trait ApiKeyServiceExt {
    async fn get_api_keys(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>>;
    async fn get_api_key_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>>;
    async fn get_api_keys_by_ids(&self, ids: Vec<ApiKeyId>) -> Result<Vec<ApiKey>>;
    async fn count_api_keys(&self) -> Result<i64>;
    async fn create_api_key(&self, input: CreateApiKeyInput, owner_id: UserId) -> Result<ApiKey>;
    async fn update_api_key(&self, id: ApiKeyId, input: UpdateApiKeyInput) -> Result<ApiKey>;
    async fn delete_api_key(&self, id: ApiKeyId) -> Result<ApiKey>;
}

pub type ApiKeyServiceDyn = Arc<dyn ApiKeyServiceExt + Send + Sync>;

pub struct ApiKeyService {
    prisma: Arc<PrismaClient>,
}

impl ApiKeyService {
    pub fn new(prisma: Arc<PrismaClient>) -> Self {
        Self { prisma }
    }
}

#[async_trait::async_trait]
impl ApiKeyServiceExt for ApiKeyService {
    async fn get_api_keys(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>> {
        let take = args.take.unwrap_or(1);

        let builder = self
            .prisma
            .api_key()
            .find_many(vec![])
            .take(take + 1)
            .order_by(api_key::id::order(Direction::Desc));

        let builder = match (&args.before, &args.after) {
            (Some(cursor), None) => builder
                .take((take + 2) * -1)
                .cursor(api_key::id::equals(cursor.id.clone())),
            (None, Some(cursor)) => builder
                .take(take + 2)
                .cursor(api_key::id::equals(cursor.id.clone())),
            _ => builder,
        };

        let items = builder
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKeys)?
            .into_iter()
            .map(|data| data.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take))
    }

    async fn get_api_key_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>> {
        let api_key = self
            .prisma
            .api_key()
            .find_unique(api_key::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKey)?
            .map(|api_key| api_key.into());

        Ok(api_key)
    }

    async fn get_api_keys_by_ids(&self, ids: Vec<ApiKeyId>) -> Result<Vec<ApiKey>> {
        let ids = ids
            .into_iter()
            .map(|id| api_key::id::equals(id.into()))
            .collect();
        let api_keys = self
            .prisma
            .api_key()
            .find_many(ids)
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKeys)?
            .into_iter()
            .map(|api_key| api_key.into())
            .collect();

        Ok(api_keys)
    }

    async fn count_api_keys(&self) -> Result<i64> {
        let count = self
            .prisma
            .api_key()
            .count(vec![])
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToCountApiKeys)?;

        Ok(count)
    }

    async fn create_api_key(&self, input: CreateApiKeyInput, owner_id: UserId) -> Result<ApiKey> {
        let created_api_key = self
            .prisma
            .api_key()
            .create(
                provider::id::equals(input.provider_id.into()),
                input.name,
                input.key,
                user::id::equals(owner_id.into()),
                vec![],
            )
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToCreateApiKey)?;

        Ok(created_api_key.into())
    }

    async fn update_api_key(&self, id: ApiKeyId, input: UpdateApiKeyInput) -> Result<ApiKey> {
        let updated_api_key = self
            .prisma
            .api_key()
            .update(api_key::id::equals(id.into()), input.into())
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToUpdateApiKey)?;

        Ok(updated_api_key.into())
    }

    async fn delete_api_key(&self, id: ApiKeyId) -> Result<ApiKey> {
        let deleted_api_key = self
            .prisma
            .api_key()
            .delete(api_key::id::equals(id.into()))
            .exec()
            .await
            .map_err(|_| ApiKeyError::UnableToDeleteApiKey)?;

        Ok(deleted_api_key.into())
    }
}

impl From<ApiKeyService> for ApiKeyServiceDyn {
    fn from(value: ApiKeyService) -> Self {
        Arc::new(value) as Self
    }
}
