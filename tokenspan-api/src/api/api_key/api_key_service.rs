use std::sync::Arc;

use async_graphql::Result;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

use tokenspan_utils::pagination::{Cursor, Pagination};

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyArgs, ApiKeyCreateInput, ApiKeyUpdateInput};
use crate::api::models::{ApiKeyId, UserId};
use crate::api::repositories::{ApiKeyCreateEntity, ApiKeyUpdateEntity};
use crate::configs::EncryptionConfig;
use crate::repository::RootRepository;

#[async_trait::async_trait]
pub trait ApiKeyServiceExt {
    async fn get_api_keys(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>>;
    async fn get_api_key_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>>;
    async fn get_api_keys_by_ids(&self, ids: Vec<ApiKeyId>) -> Result<Vec<ApiKey>>;
    async fn count_api_keys(&self) -> Result<u64>;
    async fn create_api_key(&self, input: ApiKeyCreateInput, owner_id: UserId) -> Result<ApiKey>;
    async fn update_api_key(
        &self,
        id: ApiKeyId,
        input: ApiKeyUpdateInput,
    ) -> Result<Option<ApiKey>>;
    async fn delete_api_key(&self, id: ApiKeyId) -> Result<Option<ApiKey>>;
}

pub type ApiKeyServiceDyn = Arc<dyn ApiKeyServiceExt + Send + Sync>;

pub struct ApiKeyService {
    repository: Arc<RootRepository>,
    encryption_config: EncryptionConfig,
}

impl ApiKeyService {
    const HINT_SIZE: usize = 4;
    
    pub fn new(repository: Arc<RootRepository>, encryption_config: EncryptionConfig) -> Self {
        Self {
            repository,
            encryption_config,
        }
    }

    fn create_hint(&self, key: String) -> String {
        let mut hint = String::new();
        let key_len = key.len();
        let key_first = &key[0..Self::HINT_SIZE];
        let key_last = &key[key_len - Self::HINT_SIZE..key_len];
        hint.push_str(key_first);
        hint.push_str("...");
        hint.push_str(key_last);
        hint
    }
}

#[async_trait::async_trait]
impl ApiKeyServiceExt for ApiKeyService {
    async fn get_api_keys(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>> {
        let paginated = self
            .repository
            .api_key
            .paginate::<ApiKey>(args.into())
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKeys)?;

        Ok(paginated)
    }

    async fn get_api_key_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>> {
        let api_key = self
            .repository
            .api_key
            .find_by_id(id)
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKey)?
            .map(|api_key| api_key.into());

        Ok(api_key)
    }

    async fn get_api_keys_by_ids(&self, ids: Vec<ApiKeyId>) -> Result<Vec<ApiKey>> {
        let api_keys = self
            .repository
            .api_key
            .find_many_by_ids(ids)
            .await
            .map_err(|_| ApiKeyError::UnableToGetApiKeys)?
            .into_iter()
            .map(|api_key| api_key.into())
            .collect();

        Ok(api_keys)
    }

    async fn count_api_keys(&self) -> Result<u64> {
        let count = self
            .repository
            .api_key
            .count()
            .await
            .map_err(|_| ApiKeyError::UnableToCountApiKeys)?;

        Ok(count)
    }

    async fn create_api_key(&self, input: ApiKeyCreateInput, owner_id: UserId) -> Result<ApiKey> {
        let mc = new_magic_crypt!(self.encryption_config.secret.clone(), 256);
        let encrypted_key = mc.encrypt_str_to_base64(input.key.as_str());
        let hint = self.create_hint(input.key);

        let created_api_key = self
            .repository
            .api_key
            .create(ApiKeyCreateEntity {
                owner_id,
                provider_id: input.provider_id,
                name: input.name,
                key: encrypted_key,
                hint,
            })
            .await
            .map_err(|_| ApiKeyError::UnableToCreateApiKey)?;

        Ok(created_api_key.into())
    }

    async fn update_api_key(
        &self,
        id: ApiKeyId,
        input: ApiKeyUpdateInput,
    ) -> Result<Option<ApiKey>> {
        let updated_api_key = self
            .repository
            .api_key
            .update_by_id(id, ApiKeyUpdateEntity { name: input.name })
            .await
            .map_err(|_| ApiKeyError::UnableToUpdateApiKey)?
            .map(|api_key| api_key.into());

        Ok(updated_api_key)
    }

    async fn delete_api_key(&self, id: ApiKeyId) -> Result<Option<ApiKey>> {
        let deleted_api_key = self
            .repository
            .api_key
            .delete_by_id(id)
            .await
            .map_err(|_| ApiKeyError::UnableToDeleteApiKey)?
            .map(|api_key| api_key.into());

        Ok(deleted_api_key)
    }
}

impl From<ApiKeyService> for ApiKeyServiceDyn {
    fn from(value: ApiKeyService) -> Self {
        Arc::new(value) as Self
    }
}
