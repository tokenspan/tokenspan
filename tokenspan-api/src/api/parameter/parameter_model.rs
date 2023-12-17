use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, SimpleObject};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

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
