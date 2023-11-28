use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

pub use execution_history_args::*;
pub use execution_history_input::*;

use crate::prisma::{Endpoint, ExecutionStatus};

mod execution_history_args;
mod execution_history_input;

#[Scalar]
impl ScalarType for Endpoint {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            let value = match value.as_str() {
                "Studio" => Self::Studio,
                "Sandbox" => Self::Sanbox,
                "Api" => Self::Api,
                _ => return Err(InputValueError::custom("invalid endpoint")),
            };

            Ok(value)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar]
impl ScalarType for ExecutionStatus {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            let value = match value.as_str() {
                "Completed" => Self::Completed,
                "Failed" => Self::Failed,
                "Pending" => Self::Pending,
                "Running" => Self::Running,
                _ => return Err(InputValueError::custom("invalid execution status")),
            };

            Ok(value)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}
