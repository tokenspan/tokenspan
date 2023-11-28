use async_graphql::InputObject;

use crate::api::models::TaskId;
use crate::prisma::{task_version, TaskStatus};

#[derive(InputObject)]
pub struct CreateTaskVersionInput {
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<serde_json::Value>,
    pub status: TaskStatus,
    pub task_id: TaskId,
}

#[derive(InputObject)]
pub struct UpdateTaskVersionInput {
    pub version: Option<String>,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<serde_json::Value>,
    pub status: Option<TaskStatus>,
    pub task_id: Option<String>,
}

impl From<UpdateTaskVersionInput> for Vec<task_version::SetParam> {
    fn from(value: UpdateTaskVersionInput) -> Self {
        let mut params = Vec::new();
        if let Some(version) = value.version {
            params.push(task_version::version::set(version));
        }

        params.push(task_version::release_note::set(value.release_note));
        params.push(task_version::description::set(value.description));
        params.push(task_version::document::set(value.document));
        params.push(task_version::messages::set(value.messages));

        if let Some(status) = value.status {
            params.push(task_version::status::set(status));
        }

        params
    }
}
