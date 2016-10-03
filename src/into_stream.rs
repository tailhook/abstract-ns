//! This file is from:
//!
//! https://github.com/alexcrichton/futures-rs/blob/master/src/into_stream.rs
//!
//! And is here just because package in crates.io is few days old
//!
use futures::{Async, Poll};
use futures::Future;
use futures::stream::Stream;

/// Future that forwards one element from the underlying future
/// (whether it is success of error) and emits EOF after that.
pub struct IntoStream<F: Future> {
    future: Option<F>
}

pub fn new<F: Future>(future: F) -> IntoStream<F> {
    IntoStream {
        future: Some(future)
    }
}

impl<F: Future> Stream for IntoStream<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let ret = match self.future {
            None => return Ok(Async::Ready(None)),
            Some(ref mut future) => {
                match future.poll() {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Err(e) => Err(e),
                    Ok(Async::Ready(r)) => Ok(r),
                }
            }
        };
        self.future = None;
        ret.map(|r| Async::Ready(Some(r)))
    }
}
