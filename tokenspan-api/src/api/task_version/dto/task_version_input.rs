use async_graphql::InputObject;

use crate::api::models::TaskId;
use crate::api::repositories::TaskVersionStatus;

#[derive(InputObject)]
pub struct TaskVersionCreateInput {
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<serde_json::Value>,
    pub status: TaskVersionStatus,
    pub task_id: TaskId,
}

#[derive(InputObject)]
pub struct TaskVersionUpdateInput {
    pub version: Option<String>,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Option<Vec<serde_json::Value>>,
    pub status: Option<TaskVersionStatus>,
    pub task_id: Option<String>,
}
