use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("data in token is corrupted")]
    CorruptData,

    #[error("jwt error: {0}")]
    JwtError(#[from] JwtError),

    #[error("time addition overflow")]
    TimeAdditionOverflow,

    #[error("invalid token")]
    InvalidToken,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("invalid password")]
    InvalidPassword,

    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}
