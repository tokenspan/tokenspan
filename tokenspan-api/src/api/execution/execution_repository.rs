use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::api::execution::execution_type::{Endpoint, ExecutionStatus, Usage};
use crate::api::models::{Elapsed, ExecutionId, TaskId, TaskVersionId, UserId};
use crate::repository::Repository;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElapsedEntity {
    pub pre_elapsed: f64,
    pub elapsed: f64,
    pub post_elapsed: f64,
}

impl From<ElapsedEntity> for Elapsed {
    fn from(value: ElapsedEntity) -> Self {
        Self {
            pre_elapsed: value.pre_elapsed,
            elapsed: value.elapsed,
            post_elapsed: value.post_elapsed,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionEntity {
    #[serde(rename = "_id")]
    pub id: ExecutionId,
    pub endpoint: Endpoint,
    pub elapsed: ElapsedEntity,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub task_id: TaskId,
    pub task_version_id: TaskVersionId,
    pub executed_by_id: UserId,
    pub executed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionCreateEntity {
    pub endpoint: Endpoint,
    pub elapsed: ElapsedEntity,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<Usage>,
    pub task_id: TaskId,
    pub task_version_id: TaskVersionId,
    pub executed_by_id: UserId,
}

impl Repository<ExecutionEntity> {
    pub async fn create(&self, doc: ExecutionCreateEntity) -> Result<ExecutionEntity> {
        let doc = ExecutionEntity {
            id: ExecutionId::new(),
            endpoint: doc.endpoint,
            elapsed: doc.elapsed,
            status: doc.status,
            messages: doc.messages,
            parameter: doc.parameter,
            output: doc.output,
            error: doc.error,
            usage: doc.usage,
            task_id: doc.task_id,
            task_version_id: doc.task_version_id,
            executed_by_id: doc.executed_by_id,
            executed_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let result = self.collection.insert_one(doc, None).await?;
        let id = result
            .inserted_id
            .as_object_id()
            .map(ExecutionId::from)
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }
}
