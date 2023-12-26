use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use rabbit_macros::Model;
use serde::{Deserialize, Serialize};

use rabbit_orm::pagination::{Cursor, CursorExt};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Serialize, Deserialize, Model)]
#[serde(rename(serialize = "camelCase"))]
#[rabbit(name = "parameters")]
pub struct Parameter {
    pub id: Uuid,
    pub name: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stop_sequences: Vec<String>,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub extra: Option<serde_json::Value>,
    pub model_id: Uuid,
    pub task_version_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Parameter {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}
