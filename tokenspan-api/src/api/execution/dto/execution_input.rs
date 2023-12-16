use std::collections::HashMap;

use async_graphql::InputObject;

use crate::api::models::{Elapsed, TaskId, TaskVersionId, Usage};

#[derive(InputObject)]
pub struct ExecutionCreateInput {
    pub task_id: TaskId,
    pub task_version_id: TaskVersionId,
    pub elapsed: Elapsed,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub variables: HashMap<String, String>,
}
