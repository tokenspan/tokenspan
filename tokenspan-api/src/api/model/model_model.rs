use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::ProviderId;
use crate::prisma::model;

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub String);

#[derive(SimpleObject, Debug, Clone)]
pub struct Model {
    pub id: ModelId,
    pub name: String,
    pub description: String,
    pub context: i32,
    pub pricing: String,
    pub provider_id: ProviderId,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl CursorExt<Cursor> for Model {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<model::Data> for Model {
    fn from(value: model::Data) -> Self {
        Self {
            id: ModelId(value.id),
            name: value.name,
            description: value.description,
            context: value.context,
            pricing: value.pricing,
            provider_id: ProviderId(value.provider_id),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
