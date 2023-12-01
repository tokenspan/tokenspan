use crate::api::models::{ModelId, ParameterId};
use crate::repository::Repository;
use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterEntity {
    pub id: ParameterId,
    pub model_id: ModelId,
    pub name: String,
    pub temperature: f64,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub extra: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterCreateEntity {
    pub model_id: ModelId,
    pub name: String,
    pub temperature: f64,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterUpdateEntity {
    pub name: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub presence_penalty: Option<f64>,
    pub extra: Option<serde_json::Value>,
}

impl Repository<ParameterEntity> {
    pub async fn create(&self, doc: ParameterCreateEntity) -> Result<ParameterEntity> {
        let doc = ParameterEntity {
            id: ParameterId::new(),
            model_id: doc.model_id,
            name: doc.name,
            temperature: doc.temperature,
            max_tokens: doc.max_tokens,
            stop_sequences: doc.stop_sequences,
            top_p: doc.top_p,
            frequency_penalty: doc.frequency_penalty,
            presence_penalty: doc.presence_penalty,
            extra: doc.extra,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(|id| ParameterId::from(id))
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }

    pub async fn update_by_id(
        &self,
        id: ParameterId,
        doc: ParameterUpdateEntity,
    ) -> Result<Option<ParameterEntity>> {
        let filter = doc! {
            "_id": ObjectId::from(id),
        };
        let extra = doc
            .extra
            .and_then(|config| bson::ser::to_bson(&config).ok());
        let update = doc! {
            "$set": {
                "name": doc.name,
                "temperature": doc.temperature,
                "max_tokens": doc.max_tokens,
                "stop_sequences": doc.stop_sequences,
                "top_p": doc.top_p,
                "frequency_penalty": doc.frequency_penalty,
                "presence_penalty": doc.presence_penalty,
                "extra": extra,
                "updated_at": Utc::now(),
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
