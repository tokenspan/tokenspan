use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum TaskVersionError {
    #[error("unable to create task revision")]
    UnableToCreateTaskVersion,

    #[error("unable to get task revisions")]
    UnableToGetTaskVersions,

    #[error("unable to get api key")]
    UnableToGetTaskVersion,

    #[error("unable to count task revisions")]
    UnableToCountTaskVersions,

    #[error("unable to update task revision")]
    UnableToUpdateTaskVersion,

    #[error("unable to delete task revision")]
    UnableToDeleteTaskVersion,
}
