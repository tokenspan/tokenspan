use async_graphql::{Context, ErrorExtensions, Object, Result};
use chrono::Duration;

use crate::domains::auth::auth_model::AuthPayload;
use crate::domains::auth::dto::{RefreshTokenInput, SignInInput, SignUpInput};
use crate::domains::models::RefreshPayload;
use crate::domains::services::AuthServiceDyn;
use crate::errors::AppError;

#[derive(Default)]
pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn sign_up<'a>(&self, ctx: &Context<'a>, input: SignUpInput) -> Result<AuthPayload> {
        let auth_service = ctx
            .data::<AuthServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError.extend())?;

        let payload = auth_service
            .sign_up(input.email, input.username, input.password)
            .await?;
        let token_max_age = Duration::days(30).num_seconds();

        ctx.insert_http_header(
            "Set-Cookie",
            format!(
                "refresh_token={}; Max-Age={}; Path=/; HttpOnly; SameSite=Strict",
                payload.refresh_token, token_max_age
            ),
        );

        Ok(payload)
    }

    async fn sign_in<'a>(&self, ctx: &Context<'a>, input: SignInInput) -> Result<AuthPayload> {
        let auth_service = ctx
            .data::<AuthServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError.extend())?;

        let payload = auth_service.sign_in(input.email, input.password).await?;
        let token_max_age = Duration::days(30).num_seconds();

        ctx.insert_http_header(
            "Set-Cookie",
            format!(
                "refresh_token={}; Max-Age={}; Path=/; HttpOnly; SameSite=Strict",
                payload.refresh_token, token_max_age
            ),
        );

        Ok(payload)
    }

    async fn refresh_token<'a>(
        &self,
        ctx: &Context<'a>,
        input: RefreshTokenInput,
    ) -> Result<RefreshPayload> {
        // let headers = ctx
        //     .data::<HeaderMap>()
        //     .map_err(|_| AppError::ContextExtractionError.extend())?;

        let auth_service = ctx
            .data::<AuthServiceDyn>()
            .map_err(|_| AppError::ContextExtractionError.extend())?;

        // let refresh_token = headers
        //     .get("Cookie")
        //     .ok_or(AppError::Unauthorized("no cookie header".to_string()).extend())?
        //     .to_str()
        //     .map_err(|_| AppError::Unauthorized("cookie header not a string".to_string()).extend())?
        //     .split(';')
        //     .find(|cookie| cookie.contains("refresh_token"))
        //     .ok_or(AppError::Unauthorized("no refresh_token cookie".to_string()).extend())?
        //     .split('=')
        //     .last()
        //     .ok_or(AppError::Unauthorized("no refresh_token cookie".to_string()).extend())?
        //     .to_owned();

        let payload = auth_service.refresh_token(input.refresh_token).await?;

        Ok(payload)
    }
}
