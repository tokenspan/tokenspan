use async_graphql::{Context, Guard};
use axum::headers::authorization::Bearer;
use axum::headers::{Authorization, HeaderMapExt};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

use crate::api::models::{ParsedToken, Role};
use crate::api::services::AuthService;

pub async fn guard<T>(mut req: Request<T>, next: Next<T>) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .map(|header| header.token().to_owned());
    let headers = req.headers().clone();
    req.extensions_mut().insert(None::<ParsedToken>);
    req.extensions_mut().insert(headers);

    if let Some(jwt) = token {
        let parsed_token = AuthService::decode_token(&jwt, "secret".as_ref());

        if let Ok(parsed_token) = parsed_token {
            req.extensions_mut().insert(Some(parsed_token));
        }
    }

    Ok(next.run(req).await)
}

pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        Self { role }
    }
}

#[async_trait::async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let parsed_token = ctx.data_opt::<Option<ParsedToken>>();

        if let Some(parsed_token) = parsed_token {
            if let Some(parsed_token) = parsed_token {
                return match self.role {
                    Role::Admin if parsed_token.role == Role::Admin => Ok(()),
                    Role::User if parsed_token.role == Role::Admin => Ok(()),
                    Role::User if parsed_token.role == Role::User => Ok(()),
                    _ => Err("Forbidden".into()),
                };
            }
        }

        Err("Forbidden".into())
    }
}
