use async_trait::async_trait;
use axum::extract::rejection::QueryRejection;
use axum::extract::{FromRequestParts, Query};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

use crate::error::ErrorResponse;

#[derive(Clone, Debug)]
pub struct ValidQuery<T: Validate>(pub T);

/// If the valid extractor fails it'll use this "rejection" type.
/// This rejection type can be converted into a response.
#[derive(Debug, Error)]
pub enum ValidQueryError {
    /// Validation errors
    #[error(transparent)]
    Valid(#[from] ValidationErrors),
    /// Json errors
    #[error(transparent)]
    Query(#[from] QueryRejection),
}

impl IntoResponse for ValidQueryError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ValidQueryError::Valid(validate_error) => {
                (StatusCode::UNPROCESSABLE_ENTITY, validate_error.to_string())
            }
            ValidQueryError::Query(query_error) => {
                // TODO: improve error message for response
                // JsonDataError(JsonDataError(Error { inner: Error { path: Path { segments: [] }, original: Error(\"missing field `wovenIds`\", line: 7, column: 1) } }))
                (StatusCode::BAD_REQUEST, format!("{:?}", query_error))
            }
        };

        let body = Json(ErrorResponse { message });

        (status, body).into_response()
    }
}

#[async_trait]
impl<T, S> FromRequestParts<S> for ValidQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ValidQueryError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let inner = Query::<T>::from_request_parts(parts, _state).await?;
        inner.0.validate()?;

        Ok(ValidQuery(inner.0))
    }
}
