use async_graphql::{Enum, InputObject, SimpleObject};
use async_openai::types::ChatCompletionRequestMessage;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

#[derive(Deserialize, Serialize, Enum, EnumString, Display, Debug, Copy, Clone, Eq, PartialEq)]
pub enum PromptRole {
    #[serde(rename = "user")]
    #[strum(serialize = "USER", serialize = "user")]
    User,
    #[serde(rename = "system")]
    #[strum(serialize = "SYSTEM", serialize = "system")]
    System,
    #[serde(rename = "assistant")]
    #[strum(serialize = "ASSISTANT", serialize = "assistant")]
    Assistant,
}

#[derive(Deserialize, Serialize, InputObject, Debug, Validate, Clone)]
pub struct ChatMessageInput {
    pub content: String,
    pub role: PromptRole,
}

impl From<ChatMessageInput> for ChatMessage {
    fn from(value: ChatMessageInput) -> Self {
        Self {
            content: value.content,
            role: value.role,
        }
    }
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Validate, Clone)]
pub struct ChatMessage {
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
