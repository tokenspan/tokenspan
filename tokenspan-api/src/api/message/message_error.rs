use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
