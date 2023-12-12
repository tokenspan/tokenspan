use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
