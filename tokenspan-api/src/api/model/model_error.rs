use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ModelError {
    #[error("unable to create api key")]
    UnableToCreateModel,

    #[error("unable to get models")]
    UnableToGetModels,

    #[error("unable to get api key")]
    UnableToGetModel,

    #[error("unable to count models")]
    UnableToCountModels,

    #[error("unable to update model")]
    UnableToUpdateModel,

    #[error("unable to delete model")]
    UnableToDeleteModel,
}
