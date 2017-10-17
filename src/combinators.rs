//! A number of combinators returned by methods on traits
use futures::{Async, Future, Stream};
use futures::future::{FutureResult, err};
use {Name, Address, IpList, Error};
use {Resolve, Subscribe, ResolveHost, HostSubscribe};

/// A stream returned from subscription on FrozenResolver
///
/// This stream basically yields a first value of a future and never returns
/// ready again, effectively making the stream unlimited (the end of name
/// stream shuts down the consumer by a convention)
#[derive(Debug)]
pub struct StreamOnce<F> {
    future: Option<F>,
}

/// A subscriber that resolves once and never updates the result
///
/// You can create it with `Resolve::frozen_subscriber`
#[derive(Debug)]
pub struct FrozenSubscriber<R> {
    pub(crate) resolver: R,
}

/// A resolver that implements implements Resolve+ResolveHost but returns
/// `NameNotFound` on `resolve`
///
/// This is needed to add resolver that can only resolve hostnames to
/// the router.
///
/// You can create it with `HostResolve::null_service_resolver`
#[derive(Debug)]
pub struct NullResolver<R> {
    pub(crate) resolver: R,
}

/// A resolver that implements implements Resolve+ResolveHost but returns
/// `NameNotFound` on `resolve_host`
///
/// This is needed to add resolver that can only resolve services to
/// the router.
///
/// You can create it with `Resolve::null_host_resolver`
#[derive(Debug)]
pub struct NullHostResolver<R> {
    pub(crate) resolver: R,
}

impl<F: Future> Stream for StreamOnce<F> {
    type Item = F::Item;
    type Error = F::Error;
    fn poll(&mut self) -> Result<Async<Option<F::Item>>, F::Error> {
        match self.future.as_mut() {
            Some(f) => {
                match f.poll()? {
                    Async::Ready(v) => Ok(Async::Ready(Some(v))),
                    Async::NotReady => Ok(Async::NotReady),
                }
            }
            None => Ok(Async::NotReady),
        }
    }
}

impl<R: Resolve> Resolve for NullHostResolver<R> {
    type Future = R::Future;
    fn resolve(&self, name: &Name) -> Self::Future {
        self.resolver.resolve(name)
    }
}

impl<R> Resolve for NullResolver<R> {
    type Future = FutureResult<Address, Error>;
    fn resolve(&self, _name: &Name) -> Self::Future {
        err(Error::NameNotFound)
    }
}

impl<R: Resolve> Resolve for FrozenSubscriber<R> {
    type Future = R::Future;
    fn resolve(&self, name: &Name) -> Self::Future {
        self.resolver.resolve(name)
    }
}

impl<R: Resolve> Subscribe for FrozenSubscriber<R> {
    type Stream = StreamOnce<R::Future>;
    type Error = <R::Future as Future>::Error;
    fn subscribe(&self, name: &Name) -> Self::Stream {
        StreamOnce { future: Some(self.resolve(name)) }
    }
}

impl<R: ResolveHost> ResolveHost for NullResolver<R> {
    type FutureHost = R::FutureHost;
    fn resolve_host(&self, name: &Name) -> Self::FutureHost {
        self.resolver.resolve_host(name)
    }
}

impl<R> ResolveHost for NullHostResolver<R> {
    type FutureHost = FutureResult<IpList, Error>;
    fn resolve_host(&self, _name: &Name) -> Self::FutureHost {
        err(Error::NameNotFound)
    }
}

impl<R: ResolveHost> ResolveHost for FrozenSubscriber<R> {
    type FutureHost = R::FutureHost;
    fn resolve_host(&self, name: &Name) -> Self::FutureHost {
        self.resolver.resolve_host(name)
    }
}

impl<R: ResolveHost> HostSubscribe for FrozenSubscriber<R> {
    type HostStream = StreamOnce<R::FutureHost>;
    type Error = <R::FutureHost as Future>::Error;
    fn subscribe_host(&self, name: &Name) -> Self::HostStream {
        StreamOnce { future: Some(self.resolve_host(name)) }
    }
}
