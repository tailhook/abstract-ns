use futures::BoxFuture;
use futures::stream::BoxStream;

use {Name, Address, Error};


pub trait Resolver {
    fn resolve(&self, name: Name) -> BoxFuture<Address, Error>;
    fn subscribe(&self, name: Name) -> BoxStream<Address, Error>;
}
