use std::collections::HashMap;

use async_graphql::InputObject;
use serde::Serialize;
use uuid::Uuid;

use crate::api::dto::MessageCreateInput;
use crate::api::models::{Elapsed, ExecutionStatus, Usage};

#[derive(InputObject, Serialize)]
pub struct ElapsedInput {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

impl From<ElapsedInput> for Elapsed {
    fn from(value: ElapsedInput) -> Self {
        Elapsed {
            pre_elapsed: value.pre_elapsed,
            elapsed: value.elapsed,
            post_elapsed: value.post_elapsed,
        }
    }
}

#[derive(InputObject, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageInput {
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub total_tokens: i32,
}

impl From<UsageInput> for Usage {
    fn from(value: UsageInput) -> Self {
        Usage {
            input_tokens: value.input_tokens,
            output_tokens: value.output_tokens,
            total_tokens: value.total_tokens,
        }
    }
}

#[derive(InputObject)]
pub struct ExecutionCreateInput {
    pub task_id: Uuid,
    pub task_version_id: Uuid,
    pub parameter_id: Uuid,
    pub elapsed: ElapsedInput,
    pub messages: Vec<MessageCreateInput>,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<UsageInput>,
    pub status: ExecutionStatus,
    pub variables: HashMap<String, String>,
}
