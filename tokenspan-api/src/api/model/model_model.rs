use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::ProviderId;

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct Model {
    pub id: ModelId,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub pricing: String,
    pub provider_id: ProviderId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for Model {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<super::model_repository::ModelEntity> for Model {
    fn from(value: super::model_repository::ModelEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            context: value.context,
            pricing: value.pricing,
            provider_id: value.provider_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
