use async_graphql::InputObject;

use crate::api::models::TaskVersionId;
use crate::prisma::{Endpoint, ExecutionStatus};

#[derive(InputObject)]
pub struct CreateExecutionHistoryInput {
    pub task_version_id: TaskVersionId,
    pub endpoint: Endpoint,
    pub elapsed_ms: i32,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: serde_json::Value,
}
