use axum::Json;
use chrono::Utc;

use crate::api::models::TaskId;
use crate::api::services::TaskServiceDyn;
use crate::api::task::dto::TaskExecuteInput;
use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;

pub async fn execute_task_v1(
    _task_service: TaskServiceDyn,
    _input: TaskExecuteInput,
) -> Result<Json<Task>, TaskError> {
    Ok(Json(Task {
        id: TaskId::new(),
        name: "test".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }))
}
