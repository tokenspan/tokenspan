use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskVersionError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
