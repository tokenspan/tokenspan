use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
