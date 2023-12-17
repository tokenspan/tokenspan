use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user not found: {0:?}")]
    UserNotFound(Option<Uuid>),

    #[error("invalid iterations")]
    InvalidIterations,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
