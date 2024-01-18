use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, Model};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug, Model, EmbeddedModel)]
#[serde(rename(serialize = "camelCase"))]
#[dojo(name = "parameters", sort_keys = ["created_at", "id"])]
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
    pub thread_version_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_default: bool,
}
