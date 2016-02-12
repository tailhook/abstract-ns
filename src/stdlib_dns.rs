use std::io;
use std::marker::PhantomData;
use std::net::{SocketAddr, ToSocketAddrs};

use {BlockingResolver};


/// A name resolver that uses libc name resolution
///
/// Unfortunately it can't be asynchronous, so we implement `BlockingResolver`.
/// You can put it in the thread, however (see `resolver_thread`)
pub struct StdResolver<T:ToSocketAddrs>(PhantomData<T>);


impl<T:ToSocketAddrs> StdResolver<T> {
    pub fn new() -> StdResolver<T> { StdResolver(PhantomData) }
}

impl<T:ToSocketAddrs> BlockingResolver for StdResolver<T>
{
    type Name = T;
    type Address = Vec<SocketAddr>;
    type Error = io::Error;
    fn resolve(&mut self, name: T) -> Result<Self::Address, Self::Error> {
        name.to_socket_addrs().map(|x| x.collect())
    }
}

