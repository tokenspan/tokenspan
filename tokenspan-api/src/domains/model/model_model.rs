use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, Model};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize, EmbeddedModel)]
pub struct Pricing {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

#[derive(SimpleObject, Clone, Model, Debug, Deserialize)]
#[dojo(name = "models", sort_keys = ["created_at", "id"])]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: i32,
    pub input_pricing: Pricing,
    pub output_pricing: Pricing,
    pub training_at: NaiveDateTime,
    pub provider_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
