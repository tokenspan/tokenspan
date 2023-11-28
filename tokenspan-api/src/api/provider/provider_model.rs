use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, SimpleObject, Value};
use chrono::{DateTime, FixedOffset};
use std::fmt::Display;
use tokenspan_macros::TeraId;
use tokenspan_utils::pagination::{Cursor, CursorExt};

use crate::prisma::provider;

#[derive(TeraId, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProviderId(pub String);

#[derive(SimpleObject, Debug, Clone)]
pub struct Provider {
    pub id: ProviderId,
    pub name: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl CursorExt<Cursor> for Provider {
    fn cursor(&self) -> Cursor {
        self.id.clone().into()
    }
}

impl From<provider::Data> for Provider {
    fn from(value: provider::Data) -> Self {
        Self {
            id: ProviderId(value.id),
            name: value.name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
