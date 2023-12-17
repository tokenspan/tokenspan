use std::sync::Arc;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use tokenspan_extra::pagination::{Cursor, Pagination};

use crate::api::api_key::api_key_error::ApiKeyError;
use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyArgs, ApiKeyCreateInput, ApiKeyUpdateInput};
use crate::configs::EncryptionConfig;

#[async_trait::async_trait]
pub trait ApiKeyServiceExt {
    fn decrypt(&self, key: String) -> String;
    async fn paginate(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ApiKey>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ApiKey>>;
    async fn create(&self, input: ApiKeyCreateInput, owner_id: Uuid) -> Result<ApiKey>;
    async fn update_by_id(&self, id: Uuid, input: ApiKeyUpdateInput) -> Result<ApiKey>;
    async fn delete_by_id(&self, id: Uuid) -> Result<ApiKey>;
}

pub type ApiKeyServiceDyn = Arc<dyn ApiKeyServiceExt + Send + Sync>;

pub struct ApiKeyService {
    db: DatabaseConnection,
    mc: MagicCrypt256,
}

impl ApiKeyService {
    pub fn new(db: DatabaseConnection, encryption_config: EncryptionConfig) -> Self {
        let mc = new_magic_crypt!(encryption_config.secret.clone(), 256);

        Self { mc, db }
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
        let take = args.take.unwrap_or(10);
        let limit = take
            + if args.after.is_some() || args.before.is_some() {
                2
            } else {
                1
            };
        let mut select = entity::api_key::Entity::find()
            .limit(Some(limit))
            .order_by_desc(entity::api_key::Column::CreatedAt);

        if let Some(after) = args.after.clone() {
            let after: NaiveDateTime = after.try_into()?;
            select = select.filter(entity::api_key::Column::CreatedAt.lte(after));
        }

        if let Some(before) = args.before.clone() {
            let before: NaiveDateTime = before.try_into()?;
            select = select.filter(entity::api_key::Column::CreatedAt.gte(before));
        }

        let count_fut = entity::api_key::Entity::find().count(&self.db);
        let select_fut = select.all(&self.db);

        let (count, items) = tokio::join!(count_fut, select_fut);

        let count = count.map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?;
        let items = items
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|api_key| api_key.into())
            .collect::<Vec<_>>();

        Ok(Pagination::new(items, args.before, args.after, take, count))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ApiKey>> {
        let api_key = entity::api_key::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .map(|api_key| api_key.into());

        Ok(api_key)
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ApiKey>> {
        let api_keys = entity::api_key::Entity::find()
            .filter(entity::api_key::Column::Id.is_in(ids))
            .all(&self.db)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .into_iter()
            .map(|api_key| api_key.into())
            .collect();

        Ok(api_keys)
    }

    async fn create(&self, input: ApiKeyCreateInput, owner_id: Uuid) -> Result<ApiKey> {
        let created_api_key = entity::api_key::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(input.name),
            key: Set(self.encrypt(input.key)),
            owner_id: Set(owner_id.into()),
            provider_id: Set(input.provider_id.into()),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        }
        .insert(&self.db)
        .await
        .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
        .into();

        Ok(created_api_key)
    }

    async fn update_by_id(&self, id: Uuid, input: ApiKeyUpdateInput) -> Result<ApiKey> {
        let mut api_key = entity::api_key::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ApiKeyError::Unknown(anyhow::anyhow!("ApiKey not found")))?
            .into_active_model();

        api_key.updated_at = Set(Utc::now().naive_utc());

        if let Some(name) = input.name {
            api_key.name = Set(name);
        }

        let updated_api_key = api_key
            .update(&self.db)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .into();

        Ok(updated_api_key)
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<ApiKey> {
        let deleted_api_key = entity::api_key::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?
            .ok_or(ApiKeyError::Unknown(anyhow::anyhow!("ApiKey not found")))?;

        deleted_api_key
            .clone()
            .delete(&self.db)
            .await
            .map_err(|e| ApiKeyError::Unknown(anyhow::anyhow!(e)))?;

        Ok(deleted_api_key.into())
    }
}

impl From<ApiKeyService> for ApiKeyServiceDyn {
    fn from(value: ApiKeyService) -> Self {
        Arc::new(value) as Self
    }
}
