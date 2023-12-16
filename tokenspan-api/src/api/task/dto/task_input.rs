use std::collections::HashMap;

use async_graphql::InputObject;
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use validator::Validate;

use crate::api::models::{ApiKeyId, ParameterId, TaskVersionId};

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

impl TaskUpdateInput {
    pub fn merge(&self, task: &mut entity::task::ActiveModel) {
        if let Some(ref name) = self.name {
            task.name = Set(name.clone());
        }

        if let Some(private) = self.private {
            task.private = Set(private);
        }
    }
}

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskExecuteInput {
    pub task_version_id: TaskVersionId,
    pub parameter_id: ParameterId,
    pub api_key_id: ApiKeyId,
    pub variables: HashMap<String, String>,
}
