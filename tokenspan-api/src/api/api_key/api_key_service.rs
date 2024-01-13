use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use dojo_orm::pagination::Pagination;
use dojo_orm::predicates::*;
use dojo_orm::Database;
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::api::api_key::api_key_model::ApiKey;
use crate::api::api_key::dto::{ApiKeyArgs, ApiKeyCreateInput, ApiKeyUpdateInput};

#[async_trait::async_trait]
pub trait ApiKeyServiceExt {
    fn decrypt(&self, key: String) -> String;
    async fn paginate(&self, args: ApiKeyArgs) -> Result<Pagination<ApiKey>>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<ApiKey>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ApiKey>>;
    async fn create(&self, input: ApiKeyCreateInput, owner_id: Uuid) -> Result<ApiKey>;
    async fn update_by_id(&self, id: &Uuid, input: ApiKeyUpdateInput) -> Result<ApiKey>;
    async fn delete_by_id(&self, id: &Uuid) -> Result<ApiKey>;
}

pub type ApiKeyServiceDyn = Arc<dyn ApiKeyServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct ApiKeyService {
    db: Database,
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

    async fn paginate(&self, args: ApiKeyArgs) -> Result<Pagination<ApiKey>> {
        let mut predicates: Vec<Predicate> = vec![];
        if let Some(where_args) = args.r#where {
            if let Some(provider_id) = where_args.provider_id {
                if let Some(id) = provider_id.equals {
                    predicates.push(equals("provider_id", &id));
                }
            }
        }

        self.db
            .bind::<ApiKey>()
            .cursor(args.first, args.after, args.last, args.before)
            .await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<ApiKey>> {
        self.db
            .bind::<ApiKey>()
            .where_by(equals("id", id))
            .first()
            .await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<ApiKey>> {
        self.db
            .bind::<ApiKey>()
            .where_by(in_list("id", &ids))
            .all()
            .await
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

        self.db.insert(&input).await
    }

    async fn update_by_id(&self, id: &Uuid, input: ApiKeyUpdateInput) -> Result<ApiKey> {
        self.db
            .update(&input)
            .where_by(equals("id", id))
            .exec()
            .await
    }

    async fn delete_by_id(&self, id: &Uuid) -> Result<ApiKey> {
        self.db.delete().where_by(equals("id", id)).exec().await
    }
}

impl From<ApiKeyService> for ApiKeyServiceDyn {
    fn from(value: ApiKeyService) -> Self {
        Arc::new(value) as Self
    }
}
