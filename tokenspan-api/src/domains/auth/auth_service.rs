use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use tracing::info;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::configs::AuthConfig;
use crate::domains::auth::auth_error::AuthError;
use crate::domains::auth::auth_model::{AuthPayload, Claims, ParsedToken, SessionPayload};
use crate::domains::models::{RefreshPayload, UserRole};
use crate::domains::services::UserServiceDyn;

#[async_trait::async_trait]
pub trait AuthServiceExt {
    async fn sign_up(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<AuthPayload>;
    async fn sign_up_with_role(
        &self,
        email: String,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<AuthPayload>;
    async fn sign_in(&self, email: String, password: String) -> Result<AuthPayload>;

    async fn session(&self, parsed_token: &ParsedToken) -> Result<SessionPayload>;

    async fn refresh_token(&self, refresh_token: String) -> Result<RefreshPayload>;
}

pub type AuthServiceDyn = Arc<dyn AuthServiceExt + Send + Sync>;

#[derive(TypedBuilder)]
pub struct AuthService {
    user_service: UserServiceDyn,
    auth_config: AuthConfig,
}

impl AuthService {
    fn create_token(&self, user_id: Uuid, role: &UserRole) -> Result<String, AuthError> {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.auth_config.token_exp))
            .ok_or(AuthError::TimeAdditionOverflow)?
            .timestamp();

        let claims = Claims {
            iss: self.auth_config.iss.clone(),
            aud: self.auth_config.aud.clone(),
            sub: user_id,
            exp,
            role: role.to_string(),
        };

        let header = Header::new(Algorithm::HS512);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.auth_config.secret.as_ref()),
        )
        .map_err(AuthError::JwtError)
    }

    fn create_refresh_token(&self, user_id: Uuid, role: &UserRole) -> Result<String, AuthError> {
        let exp = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(
                self.auth_config.refresh_token_exp,
            ))
            .ok_or(AuthError::TimeAdditionOverflow)?
            .timestamp();

        let claims = Claims {
            iss: self.auth_config.iss.clone(),
            aud: self.auth_config.aud.clone(),
            sub: user_id.into(),
            exp,
            role: role.to_string(),
        };

        let header = Header::new(Algorithm::HS512);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.auth_config.secret.as_ref()),
        )
        .map_err(AuthError::JwtError)
    }

    pub fn decode_token(
        jwt: &str,
        secret: &[u8],
        iss: String,
        aud: String,
    ) -> Result<ParsedToken, AuthError> {
        let mut validation = Validation::new(Algorithm::HS512);
        validation.set_issuer(&[iss]);
        validation.set_audience(&[aud]);

        info!("jwt: {:?}", jwt);
        let decoded = decode::<Claims>(jwt, &DecodingKey::from_secret(secret), &validation)
            .map_err(AuthError::JwtError)?;
        info!("decoded: {:?}", decoded);

        let role = UserRole::from_str(&decoded.claims.role).map_err(|_| AuthError::InvalidToken)?;
        Ok(ParsedToken {
            role,
            user_id: decoded.claims.sub,
        })
    }
}

#[async_trait::async_trait]
impl AuthServiceExt for AuthService {
    async fn sign_up(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<AuthPayload> {
        self.sign_up_with_role(email, username, password, UserRole::User)
            .await
    }

    async fn sign_up_with_role(
        &self,
        email: String,
        username: String,
        password: String,
        role: UserRole,
    ) -> Result<AuthPayload> {
        let user_service = self.user_service.clone();
        let created_user = user_service
            .create_with_role(email, username, password, role)
            .await?;

        let token = self.create_token(created_user.id.clone(), &created_user.role)?;

        let refresh_token =
            self.create_refresh_token(created_user.id.clone(), &created_user.role)?;

        Ok(AuthPayload {
            token,
            refresh_token,
            user: created_user,
        })
    }

    async fn sign_in(&self, email: String, password: String) -> Result<AuthPayload> {
        let user_service = self.user_service.clone();
        let user = user_service
            .find_by_email(&email)
            .await?
            .ok_or(AuthError::InvalidCredentials)?;

        user.verify_password(&password)?;
        let token = self.create_token(user.id.clone(), &user.role)?;

        let refresh_token = self.create_refresh_token(user.id.clone(), &user.role)?;

        Ok(AuthPayload {
            token,
            refresh_token,
            user,
        })
    }

    async fn session(&self, parsed_token: &ParsedToken) -> Result<SessionPayload> {
        let user_service = self.user_service.clone();
        let user = user_service
            .find_by_id(&parsed_token.user_id)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        Ok(SessionPayload { user })
    }

    async fn refresh_token(&self, refresh_token: String) -> Result<RefreshPayload> {
        let parsed_token = AuthService::decode_token(
            &refresh_token,
            self.auth_config.secret.as_ref(),
            self.auth_config.iss.clone(),
            self.auth_config.aud.clone(),
        )?;

        let new_token = self.create_token(parsed_token.user_id, &parsed_token.role)?;

        Ok(RefreshPayload {
            token: new_token,
            refresh_token: refresh_token.clone(),
        })
    }
}

impl From<AuthService> for AuthServiceDyn {
    fn from(value: AuthService) -> Self {
        Arc::new(value) as Self
    }
}
