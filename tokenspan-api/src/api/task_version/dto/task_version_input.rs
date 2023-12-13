use std::collections::HashMap;

use async_graphql::InputObject;

use crate::api::models::TaskId;
use crate::api::repositories::TaskVersionStatus;
use crate::prompt::{ChatMessageInput, RawChatMessageInput};

#[derive(InputObject)]
pub struct TaskVersionCreateInput {
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<ChatMessageInput>,
    pub raw_messages: Vec<RawChatMessageInput>,
    pub variables: HashMap<String, String>,
    pub task_id: TaskId,
}

#[derive(InputObject)]
pub struct TaskVersionUpdateInput {
    pub version: Option<String>,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Option<Vec<ChatMessageInput>>,
    pub raw_messages: Option<Vec<String>>,
    pub variables: HashMap<String, String>,
    pub status: Option<TaskVersionStatus>,
    pub task_id: Option<String>,
}
