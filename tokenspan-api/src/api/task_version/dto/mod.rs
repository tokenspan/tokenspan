use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

pub use task_version_args::*;
pub use task_version_input::*;

use crate::prisma::TaskStatus;

mod task_version_args;
mod task_version_input;

#[Scalar]
impl ScalarType for TaskStatus {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            let value = match value.as_str() {
                "Draft" => Self::Draft,
                "Published" => Self::Published,
                "Archived" => Self::Archived,
                _ => return Err(InputValueError::custom("invalid task status")),
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
