use std::collections::HashMap;

use async_graphql::InputObject;
use serde::Deserialize;
use validator::Validate;

use crate::api::models::{ApiKeyId, ModelId, ParameterId, TaskVersionId};

#[derive(InputObject)]
pub struct TaskCreateInput {
    pub name: String,
    pub slug: String,
    pub private: bool,
    pub model_id: ModelId,
}

#[derive(InputObject)]
pub struct TaskUpdateInput {
    pub name: Option<String>,
    pub private: Option<bool>,
}

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecuteInput {
    pub task_version_id: TaskVersionId,
    pub parameter_id: ParameterId,
    pub api_key_id: ApiKeyId,
    pub variables: HashMap<String, String>,
}
