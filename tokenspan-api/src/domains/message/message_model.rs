use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, Model};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize, Model, EmbeddedModel)]
#[dojo(name = "messages", sort_keys = ["created_at", "id"])]
pub struct Message {
    pub id: Uuid,
    pub thread_version_id: Uuid,
    pub owner_id: Uuid,
    pub raw: String,
    pub content: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
