use futures::BoxFuture;
use futures::stream::{Stream, BoxStream};

use {Name, Address, Error};
use stream_once::StreamOnce;


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
pub trait Resolver {

    /// Resolve a name to an address once
    fn resolve(&self, name: &Name) -> BoxFuture<Address, Error>;

    /// Resolve a name and subscribe to the updates
    ///
    /// Default implementation just yield a value once. But even if your source
    /// doesn't provide updates, you should implement some polling. The reason
    /// we don't do poling by default is because polling interval should either
    /// depend on TTL (of a DNS record for example) or on user-defined setting.
    fn subscribe(&self, name: &Name) -> BoxStream<Address, Error> {
        StreamOnce::new(self.resolve(name)).boxed()
    }
}
