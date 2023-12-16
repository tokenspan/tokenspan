use axum::Json;
use serde_json::json;

use crate::api::models::ParsedToken;
use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;
use crate::api::task::task_error::TaskError;

pub async fn execute_task_v1(
    _task_service: TaskServiceDyn,
    _input: TaskExecuteInput,
    token: Option<ParsedToken>,
) -> anyhow::Result<Json<serde_json::Value>, TaskError> {
    let _parsed_token = token.ok_or(TaskError::Unknown(anyhow::anyhow!("no token".to_string())))?;
    // let execution = task_service
    //     .execute(input, parsed_token.user_id)
    //     .await
    //     .map_err(|e| TaskError::Unknown(anyhow::anyhow!(e)))?;

    Ok(Json(json!({
        "hello": "world"
    })))
}
