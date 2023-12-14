use async_graphql::InputObject;

use crate::api::dto::{ParameterCreateInput, ParameterInputBy};
use crate::api::models::TaskId;
use crate::prompt::ChatMessageInput;

#[derive(InputObject)]
pub struct TaskVersionCreateInput {
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<ChatMessageInput>,
    pub parameters: Vec<ParameterCreateInput>,
    pub task_id: TaskId,
}

#[derive(InputObject)]
pub struct TaskVersionUpdateInput {
    pub version: Option<String>,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Option<Vec<ChatMessageInput>>,
    pub parameters: Option<Vec<ParameterInputBy>>,
    pub task_id: Option<String>,
}
