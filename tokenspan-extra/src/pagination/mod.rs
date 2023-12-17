use std::marker::PhantomData;

use anyhow::Result;
use async_graphql::connection::{Connection, CursorType, Edge};
use async_graphql::{
    InputValueError, InputValueResult, OutputType, Scalar, ScalarType, SimpleObject, Value,
};
use base64ct::{Base64, Encoding};
use chrono::NaiveDateTime;
use thiserror::Error;

pub trait CursorExt<C: CursorType> {
    fn cursor(&self) -> C;
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub id: i64,
}

impl Cursor {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
    pub fn encode(&self) -> String {
        Base64::encode_string(format!("{}", self.id).as_bytes())
    }

    pub fn decode(encoded: &str) -> Result<Self, OffsetEncodedError> {
        let decoded = Base64::decode_vec(encoded).map_err(|_| OffsetEncodedError::InvalidBase64)?;
        let id = String::from_utf8(decoded).map_err(OffsetEncodedError::Utf8Error)?;
        let id = id
            .parse::<i64>()
            .map_err(|_| OffsetEncodedError::InvalidCursor)?;

        Ok(Self { id })
    }
}

impl From<NaiveDateTime> for Cursor {
    fn from(value: NaiveDateTime) -> Self {
        Self {
            id: value.timestamp_micros(),
        }
    }
}

impl TryFrom<Cursor> for NaiveDateTime {
    type Error = OffsetEncodedError;

    fn try_from(value: Cursor) -> std::result::Result<Self, Self::Error> {
        Self::from_timestamp_micros(value.id).ok_or(OffsetEncodedError::InvalidCursor)
    }
}

#[Scalar]
impl ScalarType for Cursor {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            let cursor =
                Cursor::decode(value).map_err(|e| InputValueError::custom(e.to_string()))?;
            Ok(cursor)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        let value = self.encode();
        Value::String(value)
    }
}

impl CursorType for Cursor {
    type Error = OffsetEncodedError;

    fn decode_cursor(s: &str) -> std::result::Result<Self, Self::Error> {
        Self::decode(s)
    }

    fn encode_cursor(&self) -> String {
        self.encode()
    }
}

#[derive(SimpleObject, Debug, Clone)]
pub struct AdditionalFields {
    pub total_nodes: u64,
}

#[derive(Debug, Error)]
pub enum OffsetEncodedError {
    #[error("invalid base64")]
    InvalidBase64,

    #[error("invalid utf8: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("invalid cursor")]
    InvalidCursor,
}

pub struct Pagination<C, T>
where
    T: CursorExt<C>,
    C: CursorType + Send + Sync,
{
    pub items: Vec<T>,
    pub before: Option<Cursor>,
    pub after: Option<Cursor>,
    pub take: u64,
    pub total_nodes: u64,
    _phantom: PhantomData<C>,
}

impl<C, T> Pagination<C, T>
where
    T: CursorExt<C>,
    C: CursorType + Send + Sync,
{
    pub fn new(
        items: Vec<T>,
        before: Option<Cursor>,
        after: Option<Cursor>,
        take: u64,
        total_nodes: u64,
    ) -> Self {
        Self {
            items,
            before,
            after,
            take,
            total_nodes,
            _phantom: PhantomData,
        }
    }

    /// Returns a tuple of (has_previous, has_next)
    fn has_page(&self) -> (bool, bool) {
        let item_size = self.items.len();
        if self.after.is_some() {
            match item_size {
                0 => (false, false),
                size if size - 1 <= self.take as usize => (true, false),
                // if size is greater than 2, it means we have a next page and previous page
                // [A](cursor) -> (B) -> (C) -> (D) -> [E](check next)
                size if size > 2 => (true, true),
                // if size is less than 2, it means we have a previous page but no no page
                // [A](cursor) -> (C)
                _ => (true, false),
            }
        } else if self.before.is_some() {
            match item_size {
                0 => (false, false),
                // if size - 1 is less than or equal to take, it means we have a previous page but no next page
                // (check previous)[A] <- (B) <- (cursor)[C]
                size if size - 1 <= self.take as usize => (false, true),
                // if size is greater than 2, it means we have a next page and previous page
                // (check previous)[A] <- (B) <- (C) <- (D) <- (cursor)[E]
                size if size > 2 => (true, true),
                // otherwise, it means we have a next page but no previous page
                _ => (false, true),
            }
        } else {
            match item_size {
                0 => (false, false),
                // if size - 1 is greater than 2, it means we have a next page and no previous page
                size if size > self.take as usize => (false, true),
                // otherwise, it means we have no next page and no previous page
                _ => (false, false),
            }
        }
    }
}

impl<C, N> From<Pagination<C, N>> for Connection<C, N, AdditionalFields>
where
    C: CursorType + Send + Sync,
    N: OutputType + CursorExt<C>,
{
    fn from(value: Pagination<C, N>) -> Self {
        let item_size = value.items.len();
        let (has_previous, has_next) = value.has_page();
        let mut connection = Connection::with_additional_fields(
            has_previous,
            has_next,
            AdditionalFields {
                total_nodes: value.total_nodes,
            },
        );

        let mut edges = vec![];
        for (index, item) in value.items.into_iter().enumerate() {
            if value.after.is_some() || value.before.is_some() {
                match item_size {
                    // if size is less than or equal to take and after is present, then we need to skip the first item
                    // after: [A](cursor) -> (B) -> [C](check next)
                    // if size is less than or equal to take and before is present, then we need to skip the last item
                    // before: (check previous)[A] <- (B) <- (cursor)[C]
                    size if size - 1 <= value.take as usize => {
                        let matcher = if value.after.is_some() {
                            0
                        } else {
                            item_size - 1
                        };

                        if index == matcher {
                            continue;
                        }
                    }
                    // if size is greater than 2, then we need to skip the first and last item
                    // [A](cursor) -> (B) -> (C) -> (D) -> [E](check next)
                    // (check previous)[A] <- (B) <- (C) <- (D) <- (cursor)[E]
                    size if size > 2 => {
                        if index == 0 {
                            continue;
                        }
                        if index == item_size - 1 {
                            break;
                        }
                    }
                    // otherwise, we need to skip the first item
                    // when size is 2: [A](cursor) -> (B)
                    // when size is 1: [A](cursor)
                    // when size is 0: _
                    _ => {
                        if index == 0 {
                            continue;
                        }
                    }
                }
            } else if item_size > value.take as usize {
                // if size is greater than take, then we need to skip the last item
                // (A) -> (B) -> (C) -> (D) -> [E](check next)
                if index == item_size - 1 {
                    continue;
                }
            }

            let cursor = item.cursor();
            edges.push(Edge::new(cursor, item));
        }

        connection.edges.extend(edges);

        connection
    }
}
