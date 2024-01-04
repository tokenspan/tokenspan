use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user not found")]
    UserNotFound,

    #[error("invalid iterations")]
    InvalidIterations,

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
