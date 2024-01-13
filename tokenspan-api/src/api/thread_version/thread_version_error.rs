use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThreadVersionError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
