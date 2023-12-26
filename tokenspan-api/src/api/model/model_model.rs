use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use rabbit_macros::Model;
use rabbit_orm::pagination::{Cursor, CursorExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

#[derive(SimpleObject, Clone, Model)]
#[rabbit(name = "models")]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: i32,
    #[rabbit(embedded)]
    pub input_pricing: Pricing,
    #[rabbit(embedded)]
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
