use crate::api::loaders::ModelLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::api::model::model_error::ModelError;
use crate::api::models::{Model, ModelId};
use crate::error::AppError;

pub type ParameterId = Uuid;

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[graphql(complex)]
pub struct Parameter {
    pub id: ParameterId,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u16,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub model_id: ModelId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl Parameter {
    pub async fn model<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Option<Model>> {
        let model_loader = ctx
            .data::<DataLoader<ModelLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_loader
            .load_one(self.model_id.clone())
            .await
            .map_err(|e| ModelError::Unknown(anyhow::anyhow!(e)))?;

        Ok(model)
    }
}

impl From<entity::parameter::Model> for Parameter {
    fn from(value: entity::parameter::Model) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            temperature: value.temperature,
            max_tokens: value.max_tokens as u16,
            stop_sequences: value.stop_sequences,
            top_p: value.top_p,
            frequency_penalty: value.frequency_penalty,
            presence_penalty: value.presence_penalty,
            extra: value.extra,
            model_id: value.model_id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
