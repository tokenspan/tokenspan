use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, Model};
use dojo_orm::pagination::{Cursor, CursorExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize, EmbeddedModel)]
pub struct Pricing {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

#[derive(SimpleObject, Clone, Model)]
#[dojo(name = "models")]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: i32,
    #[dojo(embedded)]
    pub input_pricing: Pricing,
    #[dojo(embedded)]
    pub output_pricing: Pricing,
    pub training_at: NaiveDateTime,
    pub provider_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Model {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
