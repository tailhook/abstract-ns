use futures::Future;
use futures::stream::Stream;
use error::Error;
use std::net::IpAddr;

use {Name, Address};


/// Resolves a hostname into a list of IpAddresses
///
/// This is usually equivalent of the resolving A or AAAA record. This
/// kind of resolution is used in two cases:
///
/// 1. If user specified port of the service explicitly (`example.org:1234`)
/// 2. When there is known default port like `80` for http
///
/// Note: akin to A records this method returns plain list of addresses so
/// it can't use a backup addresses and weights. So this should be used for
/// simple cases and full blown `Resolve` trait (i.e. SRV records) for
/// more complex ones.
pub trait ResolveHost {

    /// A future returned from `resolve()`
    type FutureHost: Future<Item=Vec<IpAddr>, Error=Error>;

    /// Resolve a name to an address once
    fn resolve_host(&self, name: Name) -> Self::FutureHost;

}

/// Resolves a name of the service in to a set of addresses
///
/// This is commonly done using SRV records, but can also be done
/// as a wrapper around resolver by resolving a host and adding a
/// default value (see `ResolveHost::with_default_port`.
pub trait Resolve {
    /// A future returned from `resolve()`
    type Future: Future<Item=Address, Error=Error>;

    /// Resolve a name to an address once
    fn resolve(&self, name: Name) -> Self::Future;
}

/// A resolver that allows to subscribe on the host name and receive updates
///
pub trait HostSubscribe {

    /// An error type returned by a stream
    ///
    /// This is usually either ``abstract_ns::Error`` or ``Void``, showing
    /// whether error can actually occur, but can be any other error at your
    /// convenience.
    ///
    /// Note: this is an associated type so that connection pool
    /// implementations could accept `SubscribeHost<Error=Void>` as there are
    /// no reason to shutdown pool if there is a temporary error in name
    /// resolution (and all errors should be considered temporary as
    /// user can even fix invalid name by fixing configuration file while
    /// connection pool is operating).
    type Error: Into<Error>;

    /// A stream returned from `subscribe()`
    type HostStream: Stream<Item=Vec<IpAddr>, Error=Self::Error>;

    /// Resolve a name and subscribe to the updates
    ///
    /// Note: errors returned by a stream are considered fatal but temporary.
    /// I.e. stream can't be used after an error, but user might subscribe
    /// again after a short interval.
    ///
    /// For efficiency it might be useful to attempt name resolution few
    /// times if the error is temporary before returning an error, but
    /// on network resolver fatal errors (or subsequent temporary ones)
    /// should be returned so middleware and routers can failover to other
    /// sources and put errors to log.
    fn subscribe_host(&self, name: &Name) -> Self::HostStream;
}

/// A resolver that allows to subscribe on the service name
/// and receive updates
pub trait Subscribe {

    /// An error type returned by a stream
    ///
    /// This is usually either ``abstract_ns::Error`` or ``Void``, showing
    /// whether error can actually occur, but can be any other error at your
    /// convenience.
    ///
    /// Note: this is an associated type so that connection pool
    /// implementations could accept `SubscribeHost<Error=Void>` as there are
    /// no reason to shutdown pool if there is a temporary error in name
    /// resolution (and all errors should be considered temporary as
    /// user can even fix invalid name by fixing configuration file while
    /// connection pool is operating).
    type Error: Into<Error>;

    /// A stream returned from `subscribe()`
    type Stream: Stream<Item=Vec<IpAddr>, Error=Self::Error>;

    /// Resolve a name and subscribe to the updates
    ///
    /// Note: errors returned by a stream are considered fatal but temporary.
    /// I.e. stream can't be used after an error, but user might subscribe
    /// again after a short interval.
    ///
    /// For efficiency it might be useful to attempt name resolution few
    /// times if the error is temporary before returning an error, but
    /// on network resolver fatal errors (or subsequent temporary ones)
    /// should be returned so middleware and routers can failover to other
    /// sources and put errors to log.
    fn subscribe(&self, name: &Name) -> Self::Stream;
}
