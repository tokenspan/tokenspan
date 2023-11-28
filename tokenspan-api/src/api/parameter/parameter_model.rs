use crate::api::model::model_error::ModelError;
use crate::api::models::{Model, ModelId};
use crate::error::AppError;
use crate::loader::AppLoader;
use async_graphql::dataloader::DataLoader;
use async_graphql::{
    ComplexObject, Context, InputValueError, InputValueResult, Result, Scalar, ScalarType,
    SimpleObject, Value,
};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::prisma::{parameter, ParameterStatus};

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ParameterId(pub String);

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Parameter {
    pub id: ParameterId,
    pub name: String,
    pub temperature: f64,
    pub max_tokens: i32,
    pub stop_sequences: Vec<String>,
    pub top_p: f64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
    pub status: ParameterStatus,
    pub extra: Option<serde_json::Value>,
    pub model_id: ModelId,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
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

impl From<parameter::Data> for Parameter {
    fn from(value: parameter::Data) -> Self {
        Self {
            id: ParameterId(value.id),
            name: value.name,
            temperature: value.temperature,
            max_tokens: value.max_tokens,
            stop_sequences: value.stop_sequences,
            top_p: value.top_p,
            frequency_penalty: value.frequency_penalty,
            presence_penalty: value.presence_penalty,
            status: value.status,
            extra: value.extra,
            model_id: ModelId(value.model_id),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
