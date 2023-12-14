use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_macros::ID;

use crate::api::models::TaskId;
use crate::api::repositories::{TaskVersionEntity, TaskVersionStatus};
use crate::api::task_version::models::Parameter;
use crate::prompt::ChatMessage;

#[derive(ID, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TaskVersionId(ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct TaskVersion {
    pub id: TaskVersionId,
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub parameters: Vec<Parameter>,
    pub messages: Vec<ChatMessage>,
    pub status: TaskVersionStatus,
    pub task_id: TaskId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for TaskVersion {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<TaskVersionEntity> for TaskVersion {
    fn from(value: TaskVersionEntity) -> Self {
        let parameters = value.parameters.into_iter().map(|p| p.into()).collect();
        Self {
            id: TaskVersionId::from(value.id),
            version: value.version,
            release_note: value.release_note,
            description: value.description,
            document: value.document,
            messages: value.messages,
            parameters,
            status: value.status,
            task_id: value.task_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}