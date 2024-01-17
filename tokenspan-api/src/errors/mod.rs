use async_graphql::ErrorExtensions;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("context extraction error")]
    ContextExtractionError,

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("not found: {0}")]
    NotFound(String),
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        self.extend_with(|err, e| match err {
            AppError::ContextExtractionError => {
                e.set("code", "INTERNAL_SERVER_ERROR");
                e.set("reason", "context extraction error");
            }
            AppError::Unauthorized(reason) => {
                e.set("code", "UNAUTHORIZED");
                e.set("reason", reason);
            }
            AppError::NotFound(reason) => {
                e.set("code", "NOT_FOUND");
                e.set("reason", reason);
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrorResponse {
    pub message: String,
}
