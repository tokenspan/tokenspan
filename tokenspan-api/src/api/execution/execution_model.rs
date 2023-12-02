use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::execution::execution_repository::{Endpoint, ExecutionStatus};
use crate::api::models::{TaskVersionId, UserId};

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExecutionId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct Execution {
    pub id: ExecutionId,
    pub task_version_id: TaskVersionId,
    pub executed_by_id: UserId,
    pub endpoint: Endpoint,
    pub elapsed_ms: u32,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for Execution {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<super::execution_repository::ExecutionEntity> for Execution {
    fn from(value: super::execution_repository::ExecutionEntity) -> Self {
        Self {
            id: value.id,
            task_version_id: value.task_version_id,
            executed_by_id: value.executed_by_id,
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
