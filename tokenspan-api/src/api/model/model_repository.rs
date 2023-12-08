use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::api::models::{ModelId, ProviderId};
use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingEntity {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelEntity {
    #[serde(rename = "_id")]
    pub id: ModelId,
    pub provider_id: ProviderId,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub input_pricing: PricingEntity,
    pub output_pricing: PricingEntity,
    pub training_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelCreateEntity {
    pub provider_id: ProviderId,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub input_pricing: PricingEntity,
    pub output_pricing: PricingEntity,
    pub training_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelUpdateEntity {
    pub name: Option<String>,
    pub description: Option<String>,
    pub context: Option<u32>,
    pub input_pricing: Option<PricingEntity>,
    pub output_pricing: Option<PricingEntity>,
    pub training_at: Option<DateTime<Utc>>,
}

impl Repository<ModelEntity> {
    pub async fn create(&self, doc: ModelCreateEntity) -> Result<ModelEntity> {
        let doc = ModelEntity {
            id: ModelId::new(),
            provider_id: doc.provider_id,
            name: doc.name,
            description: doc.description,
            context: doc.context,
            input_pricing: doc.input_pricing,
            output_pricing: doc.output_pricing,
            training_at: doc.training_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(ModelId::from)
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
                "training_at": doc.training_at,
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
