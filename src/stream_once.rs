use futures::{Async, Poll};
use futures::Future;
use futures::stream::Stream;

/// Future that forwards one element from the underlying future
/// (whether it is success of error) and **waits indefinitely** after that.
pub struct StreamOnce<F: Future> {
    future: Option<F>
}

impl<F: Future> StreamOnce<F> {
    pub fn new(future: F) -> StreamOnce<F> {
        StreamOnce {
            future: Some(future)
        }
    }
}

impl<F: Future> Stream for StreamOnce<F> {
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let ret = match self.future {
            None => return Ok(Async::NotReady),
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
