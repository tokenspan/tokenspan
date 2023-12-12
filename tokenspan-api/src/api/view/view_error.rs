use thiserror::Error;

#[derive(Debug, Error)]
pub enum ViewError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
