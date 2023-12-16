use async_graphql::{Context, Guard};
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};

use crate::api::models::{ParsedToken, UserRole};
use crate::api::services::AuthService;
use crate::configs::AppConfig;

pub async fn guard(
    State(config): State<AppConfig>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .map(|header| header.token().to_owned());
    let headers = req.headers().clone();
    req.extensions_mut().insert(None::<ParsedToken>);
    req.extensions_mut().insert(headers);

    if let Some(jwt) = token {
        let parsed_token = AuthService::decode_token(
            &jwt,
            config.auth.secret.as_ref(),
            config.auth.iss.clone(),
            config.auth.aud.clone(),
        );

        if let Ok(parsed_token) = parsed_token {
            req.extensions_mut().insert(Some(parsed_token));
        }
    }

    Ok(next.run(req).await)
}

pub struct RoleGuard {
    role: UserRole,
}

impl RoleGuard {
    pub fn new(role: UserRole) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let parsed_token = ctx.data_opt::<Option<ParsedToken>>();

        if let Some(Some(parsed_token)) = parsed_token {
            println!("parsed_token: {:?}", parsed_token);
            return match self.role {
                UserRole::Admin if parsed_token.role == UserRole::Admin => Ok(()),
                UserRole::User if parsed_token.role == UserRole::Admin => Ok(()),
                UserRole::User if parsed_token.role == UserRole::User => Ok(()),
                _ => Err(async_graphql::Error::new("Forbidden")),
            };
        }

        Err(async_graphql::Error::new("Forbidden"))
    }
}
