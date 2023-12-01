use crate::api::models::TaskId;
use axum::Json;
use chrono::Utc;
use tracing::info;

use crate::api::task::task_error::TaskError;
use crate::api::task::task_model::Task;

pub async fn execute_task_v1() -> Result<Json<Task>, TaskError> {
    info!("Executing task");
    Ok(Json(Task {
        id: TaskId::new(),
        name: "test".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }))
}
