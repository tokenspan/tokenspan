use crate::api::models::{ModelId, ProviderId};
use crate::repository::Repository;
use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelEntity {
    pub id: ModelId,
    pub provider_id: ProviderId,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub pricing: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelCreateEntity {
    pub provider_id: ProviderId,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub pricing: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelUpdateEntity {
    pub name: Option<String>,
    pub description: Option<String>,
    pub context: Option<u32>,
    pub pricing: Option<String>,
}

impl Repository<ModelEntity> {
    pub async fn create(&self, doc: ModelCreateEntity) -> Result<ModelEntity> {
        let doc = ModelEntity {
            id: ModelId::new(),
            provider_id: doc.provider_id,
            name: doc.name,
            description: doc.description,
            context: doc.context,
            pricing: doc.pricing,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(|id| ModelId::from(id))
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn update_by_id(
        &self,
        id: ModelId,
        doc: ModelUpdateEntity,
    ) -> Result<Option<ModelEntity>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };
        let update = doc! {
            "$set": {
                "updated_at": Utc::now(),
                "name": doc.name,
                "description": doc.description,
                "context": doc.context,
                "pricing": doc.pricing,
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
