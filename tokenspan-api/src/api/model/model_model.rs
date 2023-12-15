use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_macros::ID;

use crate::api::models::ProviderId;
use crate::api::repositories::{ModelEntity, PricingEntity};

#[derive(ID, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ModelId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct Pricing {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

impl From<PricingEntity> for Pricing {
    fn from(value: PricingEntity) -> Self {
        Self {
            price: value.price,
            tokens: value.tokens,
            currency: value.currency,
        }
    }
}

#[derive(SimpleObject, Debug, Clone)]
pub struct Model {
    pub id: ModelId,
    pub name: String,
    pub description: String,
    pub context: u32,
    pub input_pricing: Pricing,
    pub output_pricing: Pricing,
    pub training_at: DateTime<Utc>,
    pub provider_id: ProviderId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for Model {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<ModelEntity> for Model {
    fn from(value: ModelEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            context: value.context,
            provider_id: value.provider_id,
            input_pricing: value.input_pricing.into(),
            output_pricing: value.output_pricing.into(),
            training_at: value.training_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
