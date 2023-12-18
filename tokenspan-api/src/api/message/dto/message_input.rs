use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Clone, Serialize, Deserialize, Debug)]
pub struct MessageCreateInput {
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
}
