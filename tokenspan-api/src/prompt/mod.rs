use async_graphql::{Enum, InputObject, SimpleObject};
use async_openai::types::ChatCompletionRequestMessage;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Enum, Debug, Copy, Clone, Eq, PartialEq)]
pub enum PromptRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Deserialize, Serialize, InputObject, Debug, Validate, Clone)]
pub struct ChatMessageInput {
    pub raw: String,
    pub content: String,
    pub role: PromptRole,
}

impl From<ChatMessageInput> for ChatMessage {
    fn from(value: ChatMessageInput) -> Self {
        Self {
            raw: value.raw,
            content: value.content,
            role: value.role,
        }
    }
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Validate, Clone)]
pub struct ChatMessage {
    pub raw: String,
    pub content: String,
    pub role: PromptRole,
}

impl TryFrom<ChatMessage> for ChatCompletionRequestMessage {
    type Error = anyhow::Error;

    fn try_from(value: ChatMessage) -> Result<Self, Self::Error> {
        let content = value.content.clone();
        let message = match value.role {
            PromptRole::User => {
                async_openai::types::ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()
                    .map_err(|e| anyhow::anyhow!(e))?
                    .into()
            }
            PromptRole::System => {
                async_openai::types::ChatCompletionRequestSystemMessageArgs::default()
                    .content(content)
                    .build()
                    .map_err(|e| anyhow::anyhow!(e))?
                    .into()
            }
            PromptRole::Assistant => {
                async_openai::types::ChatCompletionRequestAssistantMessageArgs::default()
                    .content(content)
                    .build()
                    .map_err(|e| anyhow::anyhow!(e))?
                    .into()
            }
        };

        Ok(message)
    }
}
