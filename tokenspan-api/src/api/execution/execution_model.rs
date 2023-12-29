use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use dojo_macros::{EmbeddedModel, Model, Type};
use dojo_orm::pagination::{Cursor, CursorExt};
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
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

#[derive(SimpleObject, Clone, Serialize, Model)]
#[serde(rename_all = "camelCase")]
#[dojo(name = "executions")]
pub struct Execution {
    pub id: Uuid,
    pub task_version_id: Uuid,
    pub executed_by_id: Uuid,
    pub parameter_id: Uuid,
    #[dojo(embedded)]
    pub elapsed: Elapsed,
    #[dojo(embedded)]
    pub messages: Vec<Message>,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    #[dojo(embedded)]
    pub usage: Option<Usage>,
    pub status: ExecutionStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Execution {
    fn cursor(&self) -> Cursor {
        Cursor::new("created_at".to_string(), self.created_at.timestamp_micros())
    }
}

#[derive(Type, Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString, Display, Serialize)]
#[dojo(name = "execution_status", rename_all = "lowercase")]
pub enum ExecutionStatus {
    #[strum(serialize = "success")]
    Success,
    #[strum(serialize = "failed")]
    Failed,
}
