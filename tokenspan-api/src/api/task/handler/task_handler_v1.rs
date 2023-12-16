use axum::Json;

use crate::api::models::{Execution, ParsedToken};
use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;
use crate::api::task::task_error::TaskError;

pub async fn execute_task_v1(
    task_service: TaskServiceDyn,
    input: TaskExecuteInput,
    token: Option<ParsedToken>,
) -> anyhow::Result<Json<Execution>, TaskError> {
    let parsed_token = token.ok_or(TaskError::Unknown(anyhow::anyhow!("no token".to_string())))?;
    let execution = task_service
        .execute(input, parsed_token.user_id)
        .await
        .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

    Ok(Json(execution))
}
