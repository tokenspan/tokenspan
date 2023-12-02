use bson::oid::ObjectId;
use std::fmt::Display;

use async_graphql::dataloader::DataLoader;
use async_graphql::{ComplexObject, Context, Result, Scalar, ScalarType, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::model::model_error::ModelError;
use crate::api::models::{Model, ModelId};
use crate::error::AppError;
use crate::loader::AppLoader;

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ParameterId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Parameter {
    pub id: ParameterId,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub model_id: ModelId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Parameter {
    pub async fn model<'a>(&self, ctx: &Context<'a>) -> Result<Option<Model>> {
        let app_loader = ctx
            .data::<DataLoader<AppLoader>>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let model = app_loader
            .load_one(self.model_id.clone())
            .await
            .map_err(|_| ModelError::UnableToGetModel)?;

        Ok(model)
    }
}

impl CursorExt<Cursor> for Parameter {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<super::parameter_repository::ParameterEntity> for Parameter {
    fn from(value: super::parameter_repository::ParameterEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            temperature: value.temperature,
            max_tokens: value.max_tokens,
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
