use async_graphql::SimpleObject;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use tokenspan_extra::pagination::{Cursor, CursorExt};
use tokenspan_extra::serialize_oid;
use tokenspan_macros::ID;

use crate::api::execution::execution_type::Usage;
use crate::api::models::{TaskVersionId, UserId};
use crate::api::repositories::ExecutionEntity;
use crate::api::types::{Endpoint, ExecutionStatus};

#[derive(SimpleObject, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Elapsed {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

#[derive(ID, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExecutionId(pub ObjectId);

#[derive(SimpleObject, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    #[serde(serialize_with = "serialize_oid")]
    pub id: ExecutionId,
    #[serde(serialize_with = "serialize_oid")]
    pub task_version_id: TaskVersionId,
    #[serde(serialize_with = "serialize_oid")]
    pub executed_by_id: UserId,
    pub endpoint: Endpoint,
    pub elapsed: Elapsed,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CursorExt<Cursor> for Execution {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<ExecutionEntity> for Execution {
    fn from(value: ExecutionEntity) -> Self {
        Self {
            id: value.id,
            task_version_id: value.task_version_id,
            executed_by_id: value.executed_by_id,
            endpoint: value.endpoint,
            elapsed: value.elapsed.into(),
            status: value.status,
            messages: value.messages,
            parameter: value.parameter,
            output: value.output,
            error: value.error,
            usage: value.usage,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
