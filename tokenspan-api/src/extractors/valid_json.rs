use async_trait::async_trait;
use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

use crate::errors::ErrorResponse;

#[derive(Clone, Debug)]
pub struct ValidJson<T: Validate>(pub T);

/// If the valid extractor fails it'll use this "rejection" type.
/// This rejection type can be converted into a response.
#[derive(Debug, Error)]
pub enum ValidJsonError {
    /// Validation errors
    #[error(transparent)]
    Valid(#[from] ValidationErrors),
    /// Json errors
    #[error(transparent)]
    Json(#[from] JsonRejection),
}

impl IntoResponse for ValidJsonError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ValidJsonError::Valid(validate_error) => {
                (StatusCode::UNPROCESSABLE_ENTITY, validate_error.to_string())
            }
            ValidJsonError::Json(json_error) => {
                // TODO: improve error message for response
                // JsonDataError(JsonDataError(Error { inner: Error { path: Path { segments: [] }, original: Error(\"missing field `wovenIds`\", line: 7, column: 1) } }))
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("{:?}", json_error),
                )
            }
        };

        let body = Json(ErrorResponse { message });

        (status, body).into_response()
    }
}

#[async_trait]
impl<S, T> FromRequest<S> for ValidJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidJsonError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let inner = Json::<T>::from_request(req, state).await?;
        inner.0.validate()?;

        Ok(ValidJson(inner.0))
    }
}
