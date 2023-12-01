use crate::api::models::{ApiKeyId, ParameterId, TaskVersionId};
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct TaskCreateInput {
    pub name: String,
    pub slug: String,
    pub private: bool,
}

#[derive(InputObject)]
pub struct TaskUpdateInput {
    pub name: Option<String>,
    pub private: Option<bool>,
}

#[derive(InputObject)]
pub struct TaskExecuteInput {
    pub task_version_id: TaskVersionId,
    pub parameter_id: ParameterId,
    pub api_key_id: ApiKeyId,
}
