use async_graphql::InputObject;

use crate::api::execution::execution_type::{Endpoint, ExecutionStatus};
use crate::api::models::{Elapsed, TaskId, TaskVersionId};
use crate::api::repositories::ElapsedEntity;
use crate::api::types::Usage;

#[derive(InputObject)]
pub struct ElapsedInput {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

impl From<ElapsedInput> for Elapsed {
    fn from(value: ElapsedInput) -> Self {
        Self {
            pre_elapsed: value.pre_elapsed,
            elapsed: value.elapsed,
            post_elapsed: value.post_elapsed,
        }
    }
}

impl From<ElapsedInput> for ElapsedEntity {
    fn from(value: ElapsedInput) -> Self {
        Self {
            pre_elapsed: value.pre_elapsed,
            elapsed: value.elapsed,
            post_elapsed: value.post_elapsed,
        }
    }
}

#[derive(InputObject)]
pub struct ExecutionCreateInput {
    pub task_id: TaskId,
    pub task_version_id: TaskVersionId,
    pub endpoint: Endpoint,
    pub elapsed: ElapsedInput,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
}
