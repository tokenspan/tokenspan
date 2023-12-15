use async_graphql::InputObject;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::api::models::ProviderId;
use crate::api::repositories::PricingEntity;

#[derive(InputObject, Deserialize, Debug, Clone)]
pub struct PricingInput {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

impl From<PricingInput> for PricingEntity {
    fn from(value: PricingInput) -> Self {
        Self {
            price: value.price,
            tokens: value.tokens,
            currency: value.currency,
        }
    }
}

#[derive(InputObject)]
pub struct ModelCreateInput {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: u32,
    pub input_pricing: PricingInput,
    pub output_pricing: PricingInput,
    pub training_at: DateTime<Utc>,
    pub provider_id: ProviderId,
}

#[derive(InputObject)]
pub struct ModelUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub context: Option<u32>,
    pub input_pricing: Option<PricingInput>,
    pub output_pricing: Option<PricingInput>,
    pub training_at: Option<DateTime<Utc>>,
}
