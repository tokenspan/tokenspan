use axum::Json;

use crate::domains::models::{Execution, ParsedToken};
use crate::domains::services::ThreadServiceDyn;
use crate::domains::thread::dto::ThreadExecuteInput;
use crate::domains::thread::thread_error::ThreadError;

pub async fn execute_thread_v1(
    thread_service: ThreadServiceDyn,
    input: ThreadExecuteInput,
    token: Option<ParsedToken>,
) -> anyhow::Result<Json<Execution>, ThreadError> {
    let parsed_token = token.ok_or(ThreadError::Unknown(
        anyhow::anyhow!("no token".to_string()),
    ))?;
    let execution = thread_service
        .execute(input, parsed_token.user_id)
        .await
        .map_err(|e| ThreadError::Unknown(anyhow::anyhow!(e)))?;

    Ok(Json(execution))
}
