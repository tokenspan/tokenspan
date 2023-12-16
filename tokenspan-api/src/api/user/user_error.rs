use crate::api::models::UserId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user not found: {0:?}")]
    UserNotFound(Option<UserId>),

    #[error("invalid iterations")]
    InvalidIterations,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
