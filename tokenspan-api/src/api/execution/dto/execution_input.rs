use std::collections::HashMap;

use async_graphql::InputObject;
use uuid::Uuid;

use crate::api::models::{Elapsed, Usage};

#[derive(InputObject)]
pub struct ExecutionCreateInput {
    pub task_id: Uuid,
    pub task_version_id: Uuid,
    pub elapsed: Elapsed,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub variables: HashMap<String, String>,
}
