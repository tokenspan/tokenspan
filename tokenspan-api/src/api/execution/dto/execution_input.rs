use std::collections::HashMap;

use uuid::Uuid;

use crate::api::models::{Elapsed, ExecutionStatus, Message, Usage};

pub struct ExecutionCreateInput {
    pub thread_id: Uuid,
    pub thread_version_id: Uuid,
    pub parameter_id: Uuid,
    pub elapsed: Elapsed,
    pub messages: Vec<Message>,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub status: ExecutionStatus,
    pub variables: HashMap<String, String>,
}
