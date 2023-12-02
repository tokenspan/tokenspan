use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ExecutionError {
    #[error("unable to create execution history")]
    UnableToCreateExecution,

    #[error("unable to get execution histories")]
    UnableToGetExecutions,

    #[error("unable to get api key")]
    UnableToGetExecution,

    #[error("unable to delete execution history")]
    UnableToDeleteExecution,
}
