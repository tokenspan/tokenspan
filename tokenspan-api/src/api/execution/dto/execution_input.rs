use async_graphql::InputObject;

use crate::api::models::{TaskId, TaskVersionId};
use crate::api::repositories::{Endpoint, ExecutionStatus};

#[derive(InputObject)]
pub struct ExecutionCreateInput {
    pub task_id: TaskId,
    pub task_version_id: TaskVersionId,
    pub endpoint: Endpoint,
    pub elapsed_ms: u32,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<serde_json::Value>,
}
