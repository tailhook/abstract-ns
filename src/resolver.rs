use futures::BoxFuture;
use futures::stream::BoxStream;

use {Name, Address, Error};


pub trait Resolver {
    fn resolve(name: Name) -> BoxFuture<Address, Error>;
    fn subscribe(name: Name) -> BoxStream<Address, Error>;
}
