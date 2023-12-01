use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, FixedOffset};

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

impl CursorExt<Cursor> for Execution {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}
