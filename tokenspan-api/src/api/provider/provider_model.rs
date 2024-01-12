use async_graphql::SimpleObject;
use chrono::NaiveDateTime;
use dojo_macros::Model;
use uuid::Uuid;

#[derive(SimpleObject, Debug, Clone, Model)]
#[dojo(name = "providers", sort_keys = ["created_at", "id"])]
pub struct Provider {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
