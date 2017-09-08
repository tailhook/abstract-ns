use std::collections::HashMap;
use std::net::IpAddr;

use futures::stream::{Stream};
use futures::future::{ok, err, FutureResult};
use futures::{Async, Poll};
use void::Void;

use {Name, Address, PollResolver, Resolver, Error};

/// A Stream that returns address from `MemResolver::subscribe`
pub struct MemSubscription(Option<Address>);

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

impl PollResolver for MemResolver {
    type Future = FutureResult<Address, Error>;
    fn resolve(&self, name: &Name) -> FutureResult<Address, Error> {
        match name.default_port() {
            Some(port) => {
                if let Some(addr) = self.names.get(name.host()) {
                    ok((*addr, port).into())
                } else {
                    err(Error::NameNotFound)
                }
            }
            None => err(Error::NoDefaultPort)
        }

    }
}

impl Resolver for MemResolver {
    type Stream = MemSubscription;
    fn subscribe(&self, name: &Name) -> MemSubscription {
        match name.default_port() {
            Some(port) => {
                if let Some(addr) = self.names.get(name.host()) {
                    MemSubscription(Some((*addr, port).into()))
                } else {
                    MemSubscription(None)
                }
            }
            None => MemSubscription(None),
        }
    }
}

impl Stream for MemSubscription {
    type Item = Address;
    type Error = Void;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.0.take() {
            Some(x) => Ok(Async::Ready(Some(x))),
            None => Ok(Async::NotReady),
        }
    }
}

#[cfg(test)]
mod test {
    use super::MemSubscription;
    use std::net::SocketAddr;
    use futures::{Stream, Async};

    #[test]
    fn static_stream() {
        let mut s = MemSubscription(
            Some("127.0.0.1:7879".parse::<SocketAddr>().unwrap().into()));
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
