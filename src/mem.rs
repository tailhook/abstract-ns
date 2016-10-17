use std::collections::HashMap;
use std::net::IpAddr;

use futures::{BoxFuture, IntoFuture, Future};

use {Name, Address, Resolver, Error, parse_name};

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
    pub fn add_host<S>(&mut self, name: S, address: IpAddr)
        where S: Into<String>
    {
        self.names.insert(name.into(), address);
    }
    /// Check if name is in resolver
    pub fn contains_name(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }
}

impl Resolver for MemResolver {
    fn resolve(&self, name: Name) -> BoxFuture<Address, Error> {
        match parse_name(name) {
            Some((_, None)) => {
                Err(Error::InvalidName(name.to_string(),
                    "default port must be specified for stub resolver"))
            }
            Some((host, Some(port))) => {
                if let Some(addr) = self.names.get(host) {
                    Ok((*addr, port).into())
                } else {
                    Err(Error::NameNotFound)
                }
            }
            None => {
                Err(Error::InvalidName(name.to_string(),
                    "default port can't be parsed"))
            }
        }.into_future().boxed()
    }
}
