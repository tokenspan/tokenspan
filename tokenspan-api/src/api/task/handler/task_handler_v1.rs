use crate::api::models::TaskId;
use axum::Json;
use tracing::info;

use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;

pub async fn execute_task_v1() -> Result<Json<Task>, TaskError> {
    info!("Executing task");
    Ok(Json(Task {
        id: TaskId::new(),
        name: "test".to_string(),
        created_at: chrono::DateTime::parse_from_rfc3339("2021-09-30T12:34:56.789+00:00").unwrap(),
        updated_at: chrono::DateTime::parse_from_rfc3339("2021-09-30T12:34:56.789+00:00").unwrap(),
    }))
}
