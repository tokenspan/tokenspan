use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ApiKeyError {
    #[error("unable to create api_key")]
    UnableToCreateApiKey,

    #[error("unable to get api_keys")]
    UnableToGetApiKeys,

    #[error("unable to get api key")]
    UnableToGetApiKey,

    #[error("unable to count api_keys")]
    UnableToCountApiKeys,

    #[error("unable to update api_key")]
    UnableToUpdateApiKey,

    #[error("unable to delete api_key")]
    UnableToDeleteApiKey,
}
