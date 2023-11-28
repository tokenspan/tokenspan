use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ProviderError {
    #[error("unable to create provider")]
    UnableToCreateProvider,

    #[error("unable to get providers")]
    UnableToGetProviders,

    #[error("unable to get api key")]
    UnableToGetProvider,

    #[error("unable to count providers")]
    UnableToCountProviders,

    #[error("unable to update provider")]
    UnableToUpdateProvider,

    #[error("unable to delete provider")]
    UnableToDeleteProvider,
}
