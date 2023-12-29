use crate::api::models::Message;
use async_graphql::InputObject;
use dojo_macros::EmbeddedModel;
use serde::{Deserialize, Serialize};

#[derive(InputObject, Clone, Serialize, Deserialize, Debug, EmbeddedModel)]
pub struct MessageCreateInput {
    pub raw: Option<String>,
    pub content: String,
    pub role: String,
}

impl From<MessageCreateInput> for Message {
    fn from(value: MessageCreateInput) -> Self {
        Message {
            raw: value.raw,
            content: value.content,
            role: value.role,
        }
    }
}

impl From<Message> for MessageCreateInput {
    fn from(value: Message) -> Self {
        MessageCreateInput {
            raw: value.raw,
            content: value.content,
            role: value.role,
        }
    }
}
