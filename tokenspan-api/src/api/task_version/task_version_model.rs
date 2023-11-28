use std::fmt::Display;

use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value};
use chrono::{DateTime, FixedOffset};

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::prisma::{task_version, TaskStatus};

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TaskVersionId(pub String);

#[derive(SimpleObject, Debug, Clone)]
pub struct TaskVersion {
    pub id: TaskVersionId,
    pub version: String,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub messages: Vec<serde_json::Value>,
    pub status: TaskStatus,
    pub task_id: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl CursorExt<Cursor> for TaskVersion {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<task_version::Data> for TaskVersion {
    fn from(value: task_version::Data) -> Self {
        Self {
            id: TaskVersionId(value.id),
            version: value.version,
            release_note: value.release_note,
            description: value.description,
            document: value.document,
            messages: value.messages,
            status: value.status,
            task_id: value.task_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
