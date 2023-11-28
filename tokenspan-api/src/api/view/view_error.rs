use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ViewError {
    #[error("unable to create view")]
    UnableToCreateView,

    #[error("unable to get views")]
    UnableToGetViews,

    #[error("unable to get api key")]
    UnableToGetView,

    #[error("unable to count views")]
    UnableToCountViews,

    #[error("unable to update view")]
    UnableToUpdateView,

    #[error("unable to delete view")]
    UnableToDeleteView,
}
