use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use tokenspan_extra::pagination::{Cursor, CursorExt};

use crate::api::loaders::ModelLoader;
use crate::api::models::Model;
use crate::error::AppError;

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
#[serde(rename(serialize = "camelCase"))]
#[graphql(complex)]
pub struct Parameter {
    pub id: Uuid,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub model_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Parameter {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

#[ComplexObject]
impl Parameter {
    pub async fn model<'a>(&self, ctx: &Context<'a>) -> async_graphql::Result<Option<Model>> {
        let model_loader = ctx
            .data::<DataLoader<ModelLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = model_loader.load_one(self.model_id.clone()).await?;

        Ok(model)
    }
}

impl From<entity::parameter::Model> for Parameter {
    fn from(value: entity::parameter::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            temperature: value.temperature,
            max_tokens: value.max_tokens as u32,
            stop_sequences: value.stop_sequences,
            top_p: value.top_p,
            frequency_penalty: value.frequency_penalty,
            presence_penalty: value.presence_penalty,
            extra: value.extra,
            model_id: value.model_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
