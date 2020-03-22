//! An SSE Responder.
//!
//! This module might be suitable for inclusion in rocket_contrib.

use tokio::io::{AsyncWrite, AsyncWriteExt};

mod v2;
pub use v2::{SSE2, from_stream};

// TODO: Comprehensive support for all possible message types and fields:
//   * comments
//   * 'retry' field
//   * custom fields (ignored by EventSource API, but worth considering)
/// A single SSE message, with optional `event`, `data`, and `id` fields.
#[derive(Clone)]
pub struct Event {
    event: Option<String>,
    id: Option<String>,
    data: Option<String>,
}

impl Event {
    /// Create a new Event with only the data field specified
    pub fn data<S: Into<String>>(data: S) -> Self {
        Self { event: None, id: None, data: Some(data.into()) }
    }

    // TODO: Result instead of panic!
    /// Create a new Event with event, data, and id all (optionally) specified
    ///
    /// # Panics
    ///
    /// Panics if either `event` or `id` contain newlines
    pub fn new(event: Option<String>, data: Option<String>, id: Option<String>) -> Self {
        if event.as_ref().map_or(false, |e| e.find(|b| b == '\r' || b == '\n').is_some()) {
            panic!("event cannot contain newlines");
        }

        if id.as_ref().map_or(false, |i| i.find(|b| b == '\r' || b == '\n').is_some()) {
            panic!("id cannot contain newlines");
        }

        Self { event, id, data }
    }

    /// Writes this event to a `writer` according in the EventStream
    /// format
    //TODO: Remove Unpin bound?
    pub async fn write_to<W: AsyncWrite + Unpin>(self, mut writer: W) -> Result<(), std::io::Error> {
        writer.write_all(&self.serialize()).await
    }

    pub fn serialize(self) -> Vec<u8> {
        let mut vec = vec![];

        if let Some(event) = self.event {
            vec.extend(b"event: ");
            vec.extend(event.into_bytes());
            vec.extend(b"\n");
        }
        if let Some(id) = self.id {
            vec.extend(b"id: ");
            vec.extend(id.into_bytes());
            vec.extend(b"\n");
        }
        if let Some(data) = self.data {
            for line in data.lines() {
                vec.extend(b"data: ");
                vec.extend(line.as_bytes());
                vec.extend(b"\n");
            }
        }
        vec.extend(b"\n");

        vec
    }
}
