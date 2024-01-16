use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::Model;
use serde::Deserialize;
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, Model, Deserialize)]
#[dojo(name = "providers", sort_keys = ["created_at", "id"])]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub base_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
