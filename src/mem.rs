use std::collections::HashMap;
use std::net::IpAddr;
use std::net::SocketAddr;

use futures::stream::{Stream};
use futures::future::{ok, err, FutureResult};
use futures::{BoxFuture, IntoFuture, Future, Poll};

use {Name, Address, Resolver, Error};
use stream_once::StreamOnce;

/// A stream that resolves to a static addresss and never updates
///
/// This is useful to provide a stream to localhost or for testing. It's meant
/// to be used instead of `resolver.subscribe(...)`
pub struct StaticStream(StreamOnce<FutureResult<Address, Error>>);

/// A stub resolver that resolves names from in-memory hash table
///
/// While this resolver is mostly useful in tests, you can also use it inside
/// a chain of resolvers to resolve localhost or other built-in names.
pub struct MemResolver {
    names: HashMap<String, IpAddr>,
}

impl MemResolver {
    /// Create new empty stub resolver
    ///
    /// You should add some hosts to make it useful
    pub fn new() -> MemResolver {
        MemResolver {
            names: HashMap::new(),
        }
    }
    /// Add a single host to resolve
    ///
    /// Note: only name without port number should be specified as name
    pub fn add_host<S>(&mut self, host: S, address: IpAddr)
        where S: Into<String>,
    {
        self.names.insert(host.into(), address);
    }
    /// Check if name is in resolver
    pub fn contains_name(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }
}

impl Resolver for MemResolver {
    fn resolve(&self, name: &Name) -> BoxFuture<Address, Error> {
        match name.default_port() {
            Some(port) => {
                if let Some(addr) = self.names.get(name.host()) {
                    ok((*addr, port).into()).boxed()
                } else {
                    err(Error::NameNotFound).boxed()
                }
            }
            None => err(Error::NoDefaultPort).boxed()
        }

    }
}

impl StaticStream {
    /// Create a static stream from any thing convertible to the address
    pub fn new<T: Into<Address>>(addr: T) -> StaticStream {
        StaticStream(StreamOnce::new(ok(addr.into())))
    }
}

impl Stream for StaticStream {
    type Item = Address;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

#[cfg(test)]
mod test {
    use super::StaticStream;
    use std::net::SocketAddr;
    use futures::{Stream, Async};

    #[test]
    fn static_stream() {
        let mut s = StaticStream::new(
            "127.0.0.1:7879".parse::<SocketAddr>().unwrap());
        let a = if let Ok(Async::Ready(Some(x))) = s.poll() {
            x
        } else {
            panic!("No element returned");
        };
        assert_eq!(a.at(0).addresses().collect::<Vec<_>>(),
            vec!["127.0.0.1:7879".parse::<SocketAddr>().unwrap()]);
        if let Ok(Async::NotReady) = s.poll() {
        } else {
            panic!("another element in stream?");
        }
    }
}
