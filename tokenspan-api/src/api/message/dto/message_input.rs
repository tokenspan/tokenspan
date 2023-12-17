use async_graphql::InputObject;
use serde::Serialize;

#[derive(InputObject, Clone, Serialize)]
pub struct MessageCreateInput {
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
}
