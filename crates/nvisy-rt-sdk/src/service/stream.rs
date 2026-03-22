//! Auto-paginating stream for list endpoints.
//!
//! Requires the `stream` feature. [`PageStream`] yields individual items,
//! fetching the next page automatically when the current one is exhausted.

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;

use crate::error::Result;
use crate::model::{Page, Pagination};

/// Boxed function that fetches a single page given pagination parameters.
type FetchFn<T> =
    Box<dyn Fn(Pagination) -> Pin<Box<dyn Future<Output = Result<Page<T>>> + Send>> + Send>;

/// A stream that auto-paginates through a list endpoint.
///
/// Yields individual `Result<T>` items, transparently fetching
/// subsequent pages as needed.
pub struct PageStream<T> {
    // Fetcher:
    /// Closure that fetches a single page from the API.
    fetch: FetchFn<T>,

    // Pagination state:
    /// Maximum number of items to request per page.
    page_size: u32,
    /// Offset into the full result set for the next request.
    offset: u32,
    /// Total item count reported by the server (set after the first response).
    total: Option<u32>,
    /// `true` once the last page has been received or an error occurred.
    done: bool,

    // Buffering:
    /// Items from the current page that have not yet been yielded.
    buffer: VecDeque<T>,
    /// The in-flight page request, if any.
    inflight: Option<Pin<Box<dyn Future<Output = Result<Page<T>>> + Send>>>,
}

impl<T: Send + 'static> PageStream<T> {
    pub(crate) fn new(fetch: FetchFn<T>, page_size: u32) -> Self {
        Self {
            fetch,
            page_size,
            offset: 0,
            buffer: VecDeque::new(),
            total: None,
            done: false,
            inflight: None,
        }
    }
}

impl<T: Send + Unpin + 'static> Stream for PageStream<T> {
    type Item = Result<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // Yield buffered items first.
        if let Some(item) = this.buffer.pop_front() {
            return Poll::Ready(Some(Ok(item)));
        }

        if this.done {
            return Poll::Ready(None);
        }

        // Start a new page fetch if none is in flight.
        if this.inflight.is_none() {
            let pagination = Pagination {
                offset: Some(this.offset),
                limit: Some(this.page_size),
            };
            this.inflight = Some((this.fetch)(pagination));
        }

        // Poll the in-flight future.
        match this.inflight.as_mut().unwrap().as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(e)) => {
                this.inflight = None;
                this.done = true;
                Poll::Ready(Some(Err(e)))
            }
            Poll::Ready(Ok(page)) => {
                this.inflight = None;
                this.total = Some(page.total);
                this.done = !page.has_more;
                this.offset = this.offset.saturating_add(this.page_size);
                this.buffer.extend(page.items);

                match this.buffer.pop_front() {
                    Some(item) => Poll::Ready(Some(Ok(item))),
                    None => Poll::Ready(None),
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let buffered = self.buffer.len();
        match self.total {
            Some(total) => {
                let remaining = (total as usize).saturating_sub(self.offset as usize) + buffered;
                (buffered, Some(remaining))
            }
            None => (buffered, None),
        }
    }
}
