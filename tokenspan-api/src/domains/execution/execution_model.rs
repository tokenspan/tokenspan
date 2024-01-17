use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, Model, Type};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::api::models::Message;

#[derive(SimpleObject, Debug, Clone, Serialize, Deserialize, EmbeddedModel)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub total_tokens: i32,
}

#[derive(SimpleObject, Default, Debug, Clone, Serialize, Deserialize, EmbeddedModel)]
#[serde(rename_all = "camelCase")]
pub struct Elapsed {
    pub api_key_elapsed: f64,
    pub thread_version_elapsed: f64,
    pub messages_elapsed: f64,
    pub parameter_elapsed: f64,
    pub model_elapsed: f64,
    pub provider_elapsed: f64,
    pub api_call_elapsed: f64,
    pub post_elapsed: f64,
}

#[derive(SimpleObject, Clone, Serialize, Debug, Model)]
#[serde(rename_all = "camelCase")]
#[dojo(name = "executions", sort_keys = ["created_at", "id"])]
pub struct Execution {
    pub id: Uuid,
    pub thread_version_id: Uuid,
    pub executed_by_id: Uuid,
    pub parameter_id: Uuid,
    #[dojo(embedded)]
    pub elapsed: Elapsed,
    #[dojo(embedded)]
    pub input_messages: Vec<Message>,
    pub output_messages: Vec<Message>,
    pub response: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    #[dojo(embedded)]
    pub usage: Option<Usage>,
    pub status: ExecutionStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Type, Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display, Serialize)]
#[dojo(name = "execution_status", rename_all = "lowercase")]
pub enum ExecutionStatus {
    #[strum(serialize = "success")]
    Success,
    #[strum(serialize = "failed")]
    Failed,
}