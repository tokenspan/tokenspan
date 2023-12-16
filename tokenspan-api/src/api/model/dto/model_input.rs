use async_graphql::InputObject;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::api::models::ProviderId;

#[derive(InputObject, Clone, Serialize, Deserialize)]
pub struct PricingInput {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

#[derive(InputObject)]
pub struct ModelCreateInput {
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: u32,
    pub input_pricing: PricingInput,
    pub output_pricing: PricingInput,
    pub training_at: NaiveDateTime,
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
    pub training_at: Option<NaiveDateTime>,
}
