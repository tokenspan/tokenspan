use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThreadError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for ThreadError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ThreadError::Unknown(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
