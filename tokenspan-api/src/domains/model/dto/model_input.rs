use async_graphql::InputObject;
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, UpdateModel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::models::Pricing;

#[derive(InputObject, Clone, Serialize, Deserialize, Debug, EmbeddedModel)]
pub struct PricingInput {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

impl From<PricingInput> for Pricing {
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
    pub context: i32,
    pub input_pricing: PricingInput,
    pub output_pricing: PricingInput,
    pub training_at: NaiveDateTime,
    pub provider_id: Uuid,
}

#[derive(InputObject, UpdateModel)]
pub struct ModelUpdateInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub context: Option<i32>,
    pub input_pricing: Option<PricingInput>,
    pub output_pricing: Option<PricingInput>,
    pub training_at: Option<NaiveDateTime>,
}
