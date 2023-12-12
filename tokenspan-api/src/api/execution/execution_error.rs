use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
