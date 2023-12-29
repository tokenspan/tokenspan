use std::collections::HashMap;

use async_graphql::InputObject;
use dojo_macros::UpdateModel;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(InputObject)]
pub struct TaskCreateInput {
    pub name: String,
    pub slug: String,
    pub private: bool,
}

#[derive(InputObject, UpdateModel)]
pub struct TaskUpdateInput {
    pub name: Option<String>,
    pub private: Option<bool>,
}

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecuteInput {
    pub task_version_id: Uuid,
    pub parameter_id: Uuid,
    pub api_key_id: Uuid,
    pub variables: HashMap<String, String>,
}
