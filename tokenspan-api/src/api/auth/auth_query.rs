use async_graphql::{Context, ErrorExtensions, Object, Result};

use crate::api::auth::auth_model::SessionPayload;
use crate::api::models::ParsedToken;
use crate::api::services::AuthServiceDyn;
use crate::error::AppError;

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn session<'a>(&self, ctx: &Context<'a>) -> Result<SessionPayload> {
        let parsed_token = ctx
            .data::<Option<ParsedToken>>()
            .map_err(|_| AppError::ContextExtractionError.extend())?
            .as_ref()
            .ok_or(AppError::Unauthorized("no token".to_string()).extend())?;

        let auth_service = ctx
            .data::<AuthServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError.extend())?;

        auth_service.session(parsed_token).await
    }
}
