use async_graphql::InputObject;
use dojo_macros::UpdateModel;
use serde::Deserialize;
use uuid::Uuid;

#[derive(InputObject, Debug, Clone, Deserialize)]
pub struct MessageCreateInput {
    pub raw: String,
    pub content: String,
    pub role: String,
    pub thread_version_id: Uuid,
    pub index: Option<i32>,
}

#[derive(InputObject, UpdateModel)]
pub struct MessageUpdateInput {
    pub raw: Option<String>,
    pub content: Option<String>,
    pub role: Option<String>,
    pub index: Option<i32>,
}
