use axum::response::sse::EventExt;
use axum::Json;
use bytes::{Bytes, BytesMut};

use crate::api::models::{Execution, ParsedToken};
use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;
use crate::api::task::task_error::TaskError;

/// Server-sent event
#[derive(Debug, Default, Clone)]
#[must_use]
pub struct TextEvent {
    buffer: BytesMut,
}

impl TextEvent {
    fn set(&mut self, value: impl AsRef<[u8]>) {
        let value = value.as_ref();
        self.buffer.extend_from_slice(value);
    }
}

impl EventExt for TextEvent {
    fn data<T>(mut self, data: T) -> Self
    where
        T: AsRef<str>,
    {
        let data = data.as_ref();
        self.set(data);
        self
    }

    fn finalize(self) -> Bytes {
        self.buffer.freeze()
    }
}

pub async fn execute_task_v1(
    task_service: TaskServiceDyn,
    input: TaskExecuteInput,
    token: Option<ParsedToken>,
) -> anyhow::Result<Json<Execution>, TaskError> {
    let parsed_token = token.ok_or(TaskError::Unknown(anyhow::anyhow!("no token".to_string())))?;
    let execution = task_service
        .execute_task(input, parsed_token.user_id)
        .await
        .unwrap();

    Ok(Json(execution))
}
