use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use rabbit_orm::pagination::{Cursor, Pagination};
use rabbit_orm::{Db, Order};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyArgs, ApiKeyCreateInput, ApiKeyUpdateInput};

#[async_trait::async_trait]
pub trait ApiKeyServiceExt {
    fn decrypt(&self, key: String) -> String;
    async fn paginate(&self, args: ApiKeyArgs) -> Result<Pagination<Cursor, ApiKey>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ApiKey>>;
    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ApiKey>>;
    async fn create(&self, input: ApiKeyCreateInput, owner_id: Uuid) -> Result<ApiKey>;
    async fn update_by_id(&self, id: Uuid, input: ApiKeyUpdateInput) -> Result<Option<ApiKey>>;
    async fn delete_by_id(&self, id: Uuid) -> Result<Option<ApiKey>>;
}

pub type ApiKeyServiceDyn = Arc<dyn ApiKeyServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ApiKeyService {
    db: Db,
    mc: MagicCrypt256,
}

impl ApiKeyService {
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
        self.db
            .clone()
            .from::<ApiKey>()
            .select_all()
            .cursor(args.before, args.after)
            .order_by("created_at", Order::Desc)
            .limit(args.take.unwrap_or(10))
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<ApiKey>> {
        self.db
            .clone()
            .from::<ApiKey>()
            .select_all()
            .find(id)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn find_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ApiKey>> {
        self.db
            .clone()
            .from::<ApiKey>()
            .select_all()
            .and_where("id", "in", ids)
            .all()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn create(&self, input: ApiKeyCreateInput, owner_id: Uuid) -> Result<ApiKey> {
        let input = ApiKey {
            id: Uuid::new_v4(),
            owner_id,
            name: input.name,
            key: self.encrypt(input.key),
            provider_id: input.provider_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.db
            .clone()
            .from::<ApiKey>()
            .insert(input)
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn update_by_id(&self, id: Uuid, input: ApiKeyUpdateInput) -> Result<Option<ApiKey>> {
        self.db
            .clone()
            .from::<ApiKey>()
            .update(input)
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<Option<ApiKey>> {
        self.db
            .clone()
            .from::<ApiKey>()
            .delete()
            .and_where("id", "=", id)
            .first()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }
}

impl From<ApiKeyService> for ApiKeyServiceDyn {
    fn from(value: ApiKeyService) -> Self {
        Arc::new(value) as Self
    }
}
