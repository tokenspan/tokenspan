use std::fmt::Display;

use async_graphql::{ComplexObject, Context, Result, Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::error::AppError;

#[derive(ID, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone, Serialize)]
#[graphql(complex)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

impl From<super::task_repository::TaskEntity> for Task {
    fn from(value: super::task_repository::TaskEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
            updated_at: value.updated_at,
            created_at: value.created_at,
        }
    }
}
