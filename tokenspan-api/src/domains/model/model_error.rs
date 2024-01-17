use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
