use std::fmt::Display;

use async_graphql::{ComplexObject, Context, Result, Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokenspan_extra::serialize_oid;

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_macros::ID;

use crate::api::models::TaskVersion;
use crate::api::services::TaskVersionServiceDyn;
use crate::error::AppError;

#[derive(ID, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone, Serialize)]
#[graphql(complex)]
pub struct Task {
    #[serde(serialize_with = "serialize_oid")]
    pub id: TaskId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl Task {
    pub async fn version<'a>(
        &self,
        ctx: &Context<'a>,
        version: Option<String>,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        if let Some(version) = version {
            task_version_service
                .get_task_version_by_version(self.id.clone(), version)
                .await
        } else {
            task_version_service
                .get_latest_task_version_by_task_id(self.id.clone())
                .await
        }
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
