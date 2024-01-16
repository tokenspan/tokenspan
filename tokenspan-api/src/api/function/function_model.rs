use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::Model;
use serde::Deserialize;
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, Deserialize, Model)]
#[dojo(name = "functions", sort_keys = ["created_at", "id"])]
pub struct Function {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub response: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
