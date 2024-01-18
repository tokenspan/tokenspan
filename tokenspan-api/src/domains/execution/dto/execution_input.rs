use std::collections::HashMap;

use uuid::Uuid;

use crate::domains::models::{Elapsed, ExecutionStatus, Message, Parameter, Usage};

pub struct ExecutionCreateInput {
    pub thread_id: Uuid,
    pub thread_version_id: Uuid,
    pub parameter: Parameter,
    pub elapsed: Elapsed,
    pub input_messages: Vec<Message>,
    pub output_messages: Vec<Message>,
    pub response: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub status: ExecutionStatus,
    pub variables: HashMap<String, String>,
}
