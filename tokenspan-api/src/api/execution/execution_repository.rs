use crate::api::models::{ExecutionId, TaskId, TaskVersionId, UserId};
use crate::repository::Repository;
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime, Utc};
use mongodb::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Endpoint {
    Studio,
    Http,
}

#[Scalar]
impl ScalarType for Endpoint {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value.clone() {
            Value::String(s) => match s.as_str() {
                "Studio" => Ok(Endpoint::Studio),
                "Http" => Ok(Endpoint::Http),
                _ => Err(InputValueError::expected_type(value)),
            },
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        match self {
            Endpoint::Studio => Value::String("Studio".to_string()),
            Endpoint::Http => Value::String("Http".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Success,
    Failure,
    Pending,
}

#[Scalar]
impl ScalarType for ExecutionStatus {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value.clone() {
            Value::String(s) => match s.as_str() {
                "Success" => Ok(ExecutionStatus::Success),
                "Failure" => Ok(ExecutionStatus::Failure),
                "Pending" => Ok(ExecutionStatus::Pending),
                _ => Err(InputValueError::expected_type(value)),
            },
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        match self {
            ExecutionStatus::Success => Value::String("Success".to_string()),
            ExecutionStatus::Failure => Value::String("Failure".to_string()),
            ExecutionStatus::Pending => Value::String("Pending".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionEntity {
    pub id: ExecutionId,
    pub endpoint: Endpoint,
    pub elapsed_ms: u32,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<serde_json::Value>,
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
    pub elapsed_ms: u32,
    pub status: ExecutionStatus,
    pub messages: Vec<serde_json::Value>,
    pub parameter: serde_json::Value,
    pub output: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
    pub usage: Option<serde_json::Value>,
    pub task_id: TaskId,
    pub task_version_id: TaskVersionId,
    pub executed_by_id: UserId,
}

impl Repository<ExecutionEntity> {
    pub async fn create(&self, doc: ExecutionCreateEntity) -> Result<ExecutionEntity> {
        let doc = ExecutionEntity {
            id: ExecutionId::new(),
            endpoint: doc.endpoint,
            elapsed_ms: doc.elapsed_ms,
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
            .map(|id| ExecutionId::from(id))
            .ok_or(Error::custom("invalid id"))?;

        self.find_by_id(id)
            .await?
            .ok_or(Error::custom("user not found"))
    }
}
