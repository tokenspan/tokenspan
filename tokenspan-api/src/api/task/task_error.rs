use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum TaskError {
    #[error("unable to create task")]
    UnableToCreateTask,

    #[error("unable to get tasks")]
    UnableToGetTasks,

    #[error("unable to get api key")]
    UnableToGetTask,

    #[error("unable to count tasks")]
    UnableToCountTasks,

    #[error("unable to update task")]
    UnableToUpdateTask,

    #[error("unable to delete task")]
    UnableToDeleteTask,
}

impl IntoResponse for TaskError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::UnableToCreateTask => (
                StatusCode::BAD_REQUEST,
                Self::UnableToCreateTask.to_string(),
            ),
            Self::UnableToGetTasks => (StatusCode::BAD_REQUEST, Self::UnableToGetTasks.to_string()),
            Self::UnableToGetTask => (StatusCode::BAD_REQUEST, Self::UnableToGetTask.to_string()),
            Self::UnableToCountTasks => (
                StatusCode::BAD_REQUEST,
                Self::UnableToCountTasks.to_string(),
            ),
            Self::UnableToUpdateTask => (
                StatusCode::BAD_REQUEST,
                Self::UnableToUpdateTask.to_string(),
            ),
            Self::UnableToDeleteTask => (
                StatusCode::BAD_REQUEST,
                Self::UnableToDeleteTask.to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
