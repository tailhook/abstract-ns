use futures::Future;
use futures::stream::Stream;
use void::Void;

use {Name, Address, Error};


/// A resolver of name to an address that runs once
///
/// You should prefer using `Resolver::subscribe` if your process
/// is long-running.
///
/// See also `Resolver`
pub trait PollResolver {

    /// A future returned from `resolve()`
    type Future: Future<Item=Address, Error=Error>;

    /// Resolve a name to an address once
    fn resolve(&self, name: &Name) -> Self::Future;

}

/// Main trait that does name resolution
///
/// This is a trait to be implemented for the service that does name resolution
///
/// If your protocol implementation requires name resolution you can either:
///
/// * Accept an object implementing `Resolver` if this is a complex protocol
/// * Or, accept a future or a stream that returns address
///
/// Latter is preferable if your protocol implemetation only needs one name,
/// while former is more flexible.
///
/// Also accepting a stream is more preferable than a future, because names
/// do change over time and it's weird if user needs to restart an application
/// to update host name.
pub trait Resolver: PollResolver {

    /// A stream returned from `subscribe()`
    type Stream: Stream<Item=Address, Error=Void>;

    /// Resolve a name and subscribe to the updates
    fn subscribe(&self, name: &Name) -> Self::Stream;
}
