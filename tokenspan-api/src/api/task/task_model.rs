use std::fmt::Display;

use async_graphql::{
    ComplexObject, Context, Result, Scalar, ScalarType,
    SimpleObject,
};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;

use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::error::AppError;
use crate::prisma::task;

#[derive(TeraId, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TaskId(pub String);

#[derive(SimpleObject, Debug, Clone, Serialize)]
#[graphql(complex)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[ComplexObject]
impl Task {
    pub async fn versions<'a>(&self, ctx: &Context<'a>) -> Result<Vec<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        let task_versions = task_version_service
            .get_task_versions_by_task_id(self.id.clone())
            .await
            .unwrap_or_default();

        Ok(task_versions)
    }
}

impl CursorExt<Cursor> for Task {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<task::Data> for Task {
    fn from(value: task::Data) -> Self {
        Self {
            id: TaskId(value.id),
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
