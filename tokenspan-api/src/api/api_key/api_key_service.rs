use std::sync::Arc;

use anyhow::Result;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyArgs, ApiKeyCreateInput, ApiKeyUpdateInput};
use crate::api::models::{ApiKeyId, UserId};
use crate::api::repositories::{ApiKeyCreateEntity, ApiKeyUpdateEntity};
use crate::configs::EncryptionConfig;
use crate::repository::RootRepository;

#[async_trait::async_trait]
pub trait ApiKeyServiceExt {
    fn decrypt(&self, key: String) -> String;
    async fn paginate(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>>;
    async fn find_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>>;
    async fn find_by_ids(&self, ids: Vec<ApiKeyId>) -> Result<Vec<ApiKey>>;
    async fn count(&self) -> Result<u64>;
    async fn create(&self, input: ApiKeyCreateInput, owner_id: UserId) -> Result<ApiKey>;
    async fn update_by_id(&self, id: ApiKeyId, input: ApiKeyUpdateInput) -> Result<Option<ApiKey>>;
    async fn delete_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>>;
}

pub type ApiKeyServiceDyn = Arc<dyn ApiKeyServiceExt + Send + Sync>;

pub struct ApiKeyService {
    repository: RootRepository,
    mc: MagicCrypt256,
}

impl ApiKeyService {
    const HINT_SIZE: usize = 3;

    pub fn new(repository: RootRepository, encryption_config: EncryptionConfig) -> Self {
        let mc = new_magic_crypt!(encryption_config.secret.clone(), 256);

        Self { repository, mc }
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

    pub fn encrypt(&self, key: String) -> String {
        self.mc.encrypt_str_to_base64(key.as_str())
    }
}

#[async_trait::async_trait]
impl ApiKeyServiceExt for ApiKeyService {
    fn decrypt(&self, key: String) -> String {
        self.mc.decrypt_base64_to_string(key.as_str()).unwrap()
    }

    async fn paginate(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>> {
        let paginated = self
            .repository
            .api_key
            .paginate::<ApiKey>(args.into())
            .await
            .map_err(|e| {
                println!("error: {:?}", e);
                ApiKeyError::Unknown(anyhow::anyhow!(e))
            })?;

        Ok(paginated)
    }

    async fn find_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>> {
        let api_key = self
            .repository
            .api_key
            .find_by_id(id)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .map(|api_key| api_key.into());

        Ok(api_key)
    }

    async fn find_by_ids(&self, ids: Vec<ApiKeyId>) -> Result<Vec<ApiKey>> {
        let api_keys = self
            .repository
            .api_key
            .find_many_by_ids(ids)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|api_key| api_key.into())
            .collect();

        Ok(api_keys)
    }

    async fn count(&self) -> Result<u64> {
        let count = self
            .repository
            .api_key
            .count()
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?;

        Ok(count)
    }

    async fn create(&self, input: ApiKeyCreateInput, owner_id: UserId) -> Result<ApiKey> {
        let encrypted_key = self.encrypt(input.key.clone());
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
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?;

        Ok(created_api_key.into())
    }

    async fn update_by_id(&self, id: ApiKeyId, input: ApiKeyUpdateInput) -> Result<Option<ApiKey>> {
        let updated_api_key = self
            .repository
            .api_key
            .update_by_id(id, ApiKeyUpdateEntity { name: input.name })
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .map(|api_key| api_key.into());

        Ok(updated_api_key)
    }

    async fn delete_by_id(&self, id: ApiKeyId) -> Result<Option<ApiKey>> {
        let deleted_api_key = self
            .repository
            .api_key
            .delete_by_id(id)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .map(|api_key| api_key.into());

        Ok(deleted_api_key)
    }
}

impl From<ApiKeyService> for ApiKeyServiceDyn {
    fn from(value: ApiKeyService) -> Self {
        Arc::new(value) as Self
    }
}
