mod parameter_args;
mod parameter_input;

use crate::prisma::ParameterStatus;
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
pub use parameter_args::*;
pub use parameter_input::*;

#[Scalar]
impl ScalarType for ParameterStatus {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            let value = match value.as_str() {
                "Draft" => Self::Draft,
                "Archived" => Self::Archived,
                "Published" => Self::Published,
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
