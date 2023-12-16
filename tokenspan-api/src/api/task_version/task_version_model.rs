use async_graphql::{Enum, SimpleObject};
use chrono::NaiveDateTime;
use sea_orm::prelude::Uuid;
use strum_macros::EnumString;

use tokenspan_extra::pagination::{Cursor, CursorExt};

use crate::api::models::TaskId;

pub type TaskVersionId = Uuid;

#[derive(SimpleObject, Clone)]
pub struct TaskVersion {
    pub id: TaskVersionId,
    pub version: u32,
    pub release_note: Option<String>,
    pub description: Option<String>,
    pub document: Option<String>,
    pub status: TaskVersionStatus,
    pub task_id: TaskId,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CursorExt<Cursor> for TaskVersion {
    fn cursor(&self) -> Cursor {
        self.created_at.into()
    }
}

impl From<entity::task_version::Model> for TaskVersion {
    fn from(value: entity::task_version::Model) -> Self {
        Self {
            id: TaskVersionId::from(value.id),
            version: value.version as u32,
            release_note: value.release_note,
            description: value.description,
            document: value.document,
            status: TaskVersionStatus::from(value.status),
            task_id: value.task_id.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Enum, Copy, Clone, Debug, Eq, PartialEq, EnumString)]
#[graphql(remote = "entity::sea_orm_active_enums::TaskVersionStatus")]
pub enum TaskVersionStatus {
    Draft,
    Released,
}
