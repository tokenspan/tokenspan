mod task_handler_v1;

use axum::response::sse::EventExt;
use bytes::{Bytes, BytesMut};
pub use task_handler_v1::*;

/// Server-sent event
#[derive(Debug, Default, Clone)]
#[must_use]
pub struct TextEvent {
    buffer: BytesMut,
}

impl TextEvent {
    fn set(&mut self, value: impl AsRef<[u8]>) {
        let value = value.as_ref();
        self.buffer.extend_from_slice(value);
    }
}

impl EventExt for TextEvent {
    fn data<T>(mut self, data: T) -> Self
    where
        T: AsRef<str>,
    {
        let data = data.as_ref();
        self.set(data);
        self
    }

    fn finalize(self) -> Bytes {
        self.buffer.freeze()
    }
}
