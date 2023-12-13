use crate::api::models::{ModelId, ParameterId};
use crate::repository::Repository;
use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    #[serde(rename = "_id")]
    pub id: ParameterId,
    pub model_id: ModelId,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u16,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterCreateEntity {
    pub model_id: ModelId,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u16,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterUpdateEntity {
    pub name: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stop_sequences: Option<Vec<String>>,
    pub top_p: Option<f32>,
    pub frequency_penalty: Option<f32>,
    pub presence_penalty: Option<f32>,
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
            .map(ParameterId::from)
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
                "maxTokens": doc.max_tokens,
                "stopSequences": doc.stop_sequences,
                "topP": doc.top_p,
                "frequencyPenalty": doc.frequency_penalty,
                "presencePenalty": doc.presence_penalty,
                "extra": extra,
                "updatedAt": Utc::now(),
            }
        };

        self.collection
            .find_one_and_update(filter, update, None)
            .await
    }
}
