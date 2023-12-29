use async_graphql::SimpleObject;
use dojo_macros::EmbeddedModel;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Serialize, Deserialize, Debug, EmbeddedModel)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
}
