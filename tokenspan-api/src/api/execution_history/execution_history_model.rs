use crate::api::models::{TaskVersionId, UserId};
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value};
use chrono::{DateTime, FixedOffset};
use std::fmt::Display;
use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::prisma::{execution_history, Endpoint, ExecutionStatus};

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExecutionHistoryId(pub String);

#[derive(SimpleObject, Debug, Clone)]
pub struct ExecutionHistory {
    pub id: ExecutionHistoryId,
    pub task_version_id: TaskVersionId,
    pub executed_by_id: UserId,
    pub endpoint: Endpoint,
    pub elapsed_ms: i32,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<serde_json::Value>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl CursorExt<Cursor> for ExecutionHistory {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<execution_history::Data> for ExecutionHistory {
    fn from(value: execution_history::Data) -> Self {
        Self {
            id: ExecutionHistoryId(value.id),
            task_version_id: TaskVersionId(value.task_version_id),
            executed_by_id: UserId(value.executed_by_id),
            endpoint: value.endpoint,
            elapsed_ms: value.elapsed_ms,
            status: value.status,
            messages: value.messages,
            parameter: value.parameter,
            output: value.output,
            error: value.error,
            usage: value.usage,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
