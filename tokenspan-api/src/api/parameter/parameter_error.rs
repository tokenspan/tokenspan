use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ParameterError {
    #[error("unable to create parameter")]
    UnableToCreateParameter,

    #[error("unable to get parameters")]
    UnableToGetParameters,

    #[error("unable to get api key")]
    UnableToGetParameter,

    #[error("unable to count parameters")]
    UnableToCountParameters,

    #[error("unable to update parameter")]
    UnableToUpdateParameter,

    #[error("unable to delete parameter")]
    UnableToDeleteParameter,
}
