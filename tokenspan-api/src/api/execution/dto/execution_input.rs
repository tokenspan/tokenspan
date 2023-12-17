use std::collections::HashMap;

use async_graphql::InputObject;
use serde::Serialize;
use uuid::Uuid;

use crate::api::models::{ExecutionStatus, Usage};

#[derive(InputObject, Serialize)]
pub struct ElapsedInput {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

#[derive(InputObject)]
pub struct ExecutionCreateInput {
    pub task_id: Uuid,
    pub task_version_id: Uuid,
    pub elapsed: ElapsedInput,
    pub messages: serde_json::Value,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub status: ExecutionStatus,
    pub variables: HashMap<String, String>,
}
