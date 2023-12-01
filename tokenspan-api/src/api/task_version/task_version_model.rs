use std::fmt::Display;

use async_graphql::{Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TaskVersionId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone)]
pub struct TaskVersion {
    pub id: TaskVersionId,
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<serde_json::Value>,
    pub task_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for TaskVersion {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<super::task_version_repository::TaskVersionEntity> for TaskVersion {
    fn from(value: super::task_version_repository::TaskVersionEntity) -> Self {
        Self {
            id: value.id,
            version: value.version,
            release_note: value.release_note,
            description: value.description,
            document: value.document,
            messages: value.messages,
            task_id: value.task_id.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
