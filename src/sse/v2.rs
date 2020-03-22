//! The V2 implementation not using io_channel

use std::convert::TryInto;
use std::io::{self, Cursor, Read};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::stream::Stream;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use tokio::io::AsyncRead;

use super::Event;

pin_project_lite::pin_project! {
    /// An SSE stream. This type implements `Responder`; see the
    /// [`from_stream`] function for a usage example.
    pub struct SSE2<S> {
        #[pin]
        stream: S,
        state: State,
    }
}

enum State {
    Pending,
    Partial(Cursor<Vec<u8>>),
    Done,
}

/// Creates an SSE stream from a [`Stream`] of [`Event`]s.
///
/// # Example
///
/// ```rust
/// # use rocket::get;
/// #
/// use futures::stream::Stream;
/// use rocket_rooms::sse::{self, Event, SSE2};
///
/// #[get("/stream")]
/// fn stream() -> SSE2<impl Stream<Item = sse::Event>> {
///     sse::from_stream(async_stream::stream! {
///         yield Event::data("data1");
///         yield Event::data("data2");
///         yield Event::data("data3");
///     })
/// }
/// ```
pub fn from_stream<S: Stream<Item=Event>>(stream: S) -> SSE2<S> {
    SSE2 { stream, state: State::Pending }
}

#[rocket::async_trait]
impl<'r, S: Stream<Item=Event> + Send + 'r> Responder<'r> for SSE2<S> {
    async fn respond_to(self, _req: &'r Request<'_>) -> rocket::response::Result<'r> {
        Response::build()
            .raw_header("Content-Type", "text/event-stream")
            .raw_header("Cache-Control", "no-cache")
            .raw_header("Expires", "0")
            .streamed_body(self)
            .ok()
    }
}

impl<S: Stream<Item=Event>> AsyncRead for SSE2<S> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<Result<usize, io::Error>> {
        let mut this = self.project();

        if buf.len() == 0 {
            return Poll::Ready(Ok(0));
        }

        loop {
            match this.state {
                State::Pending => {
                    // Get the next buffer
                    match this.stream.as_mut().poll_next(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(Some(next_event)) => *this.state = State::Partial(Cursor::new(next_event.serialize())),
                        Poll::Ready(None) => *this.state = State::Done,
                    }
                },
                State::Partial(cursor) => {
                    // Copy as much pending data as possible
                    let copied = cursor.read(buf)?;
                    if TryInto::<usize>::try_into(cursor.position()).unwrap() == cursor.get_ref().len() {
                        *this.state = State::Pending;
                    }
                    return Poll::Ready(Ok(copied));
                },
                State::Done => return Poll::Ready(Ok(0)),
            }
        }
    }
}
