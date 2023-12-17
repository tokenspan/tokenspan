use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use tokenspan_extra::pagination::{Cursor, CursorExt};

pub type ModelId = Uuid;

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct Pricing {
    pub price: f64,
    pub tokens: u32,
    pub currency: String,
}

#[derive(SimpleObject, Clone)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub context: u32,
    pub input_pricing: Pricing,
    pub output_pricing: Pricing,
    pub training_at: NaiveDateTime,
    pub provider_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Model {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::model::Model> for Model {
    fn from(value: entity::model::Model) -> Self {
        let input_pricing = serde_json::from_value(value.input_pricing).unwrap();
        let output_pricing = serde_json::from_value(value.output_pricing).unwrap();

        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            slug: value.slug,
            context: value.context as u32,
            input_pricing,
            output_pricing,
            training_at: value.training_at,
            provider_id: value.provider_id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
