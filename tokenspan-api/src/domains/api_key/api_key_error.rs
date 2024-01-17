use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiKeyError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
