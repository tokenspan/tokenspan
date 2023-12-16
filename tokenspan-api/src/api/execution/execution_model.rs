use async_graphql::{InputObject, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_macros::ID;

use crate::api::models::{TaskVersionId, UserId};

#[derive(SimpleObject, InputObject, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

#[derive(SimpleObject, InputObject, Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Elapsed {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

pub type ExecutionId = Uuid;

#[derive(SimpleObject, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub id: ExecutionId,
    pub task_version_id: TaskVersionId,
    pub executed_by_id: UserId,
    pub elapsed: Elapsed,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for Execution {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::execution::Model> for Execution {
    fn from(value: entity::execution::Model) -> Self {
        let usage = value
            .usage
            .and_then(|usage| serde_json::from_value(usage).ok());

        let elapsed = serde_json::from_value(value.elapsed).unwrap_or_default();

        Self {
            id: value.id.into(),
            task_version_id: value.task_version_id.into(),
            executed_by_id: value.executor_id.into(),
            elapsed,
            usage,
            messages: value.messages,
            parameter: value.parameter,
            output: value.output,
            error: value.error,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
