use std::fmt::Display;

use async_graphql::{ComplexObject, Context, Result, Scalar, ScalarType, SimpleObject};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokenspan_extra::serialize_oid;

use tokenspan_macros::ID;
use tokenspan_utils::pagination::{Cursor, CursorExt};

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
        version: String,
    ) -> Result<Option<TaskVersion>> {
        let task_version_service = ctx
            .data::<TaskVersionServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError)?;

        task_version_service
            .get_task_version_by_version(version)
            .await
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
