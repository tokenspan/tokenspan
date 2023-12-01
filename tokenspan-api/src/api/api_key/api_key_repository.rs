use bson::doc;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::api::models::{ApiKeyId, ProviderId, UserId};
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyEntity {
    #[serde(rename = "_id")]
    pub id: ApiKeyId,
    pub owner_id: UserId,
    pub provider_id: ProviderId,
    pub name: String,
    pub key: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyCreateEntity {
    pub owner_id: UserId,
    pub provider_id: ProviderId,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyUpdateEntity {
    pub name: Option<String>,
}

impl Repository<ApiKeyEntity> {
    pub async fn create(&self, doc: ApiKeyCreateEntity) -> Result<ApiKeyEntity> {
        let doc = ApiKeyEntity {
            id: ApiKeyId::new(),
            owner_id: doc.owner_id,
            provider_id: doc.provider_id,
            name: doc.name,
            key: doc.key,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(|id| ApiKeyId::from(id))
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("api key not found"))
    }

    pub async fn update_by_id(
        &self,
        id: ApiKeyId,
        doc: ApiKeyUpdateEntity,
    ) -> Result<Option<ApiKeyEntity>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };
        let update = doc! {
            "$set": {
                "name": doc.name,
                "updated_at": Utc::now(),
            },
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
