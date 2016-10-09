use futures::BoxFuture;
use futures::stream::{Stream, BoxStream};

use {Name, Address, Error};
use stream_once::StreamOnce;


pub trait Resolver {

    /// Resolve a name to an address once
    fn resolve(&self, name: Name) -> BoxFuture<Address, Error>;

    /// Resolve a name and subscribe to the updates
    ///
    /// Default implementation just yield a value once. But even if your source
    /// doesn't provide updates, you should implement some polling. The reason
    /// we don't do poling by default is because polling interval should either
    /// depend on TTL (of a DNS record for example) or on user-defined setting.
    fn subscribe(&self, name: Name) -> BoxStream<Address, Error> {
        StreamOnce::new(self.resolve(name)).boxed()
    }
}
