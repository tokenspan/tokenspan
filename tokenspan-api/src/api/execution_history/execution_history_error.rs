use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ExecutionHistoryError {
    #[error("unable to create execution history")]
    UnableToCreateExecutionHistory,

    #[error("unable to get execution histories")]
    UnableToGetExecutionHistories,

    #[error("unable to get api key")]
    UnableToGetExecutionHistory,

    #[error("unable to count execution histories")]
    UnableToCountExecutionHistories,

    #[error("unable to delete execution history")]
    UnableToDeleteExecutionHistory,
}
