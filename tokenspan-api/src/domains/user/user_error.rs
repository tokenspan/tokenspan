use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("invalid iterations")]
    InvalidIterations,
}
