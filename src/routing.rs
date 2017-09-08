use std::net::IpAddr;
use std::sync::Arc;
use std::collections::HashMap;

use futures::{BoxFuture, Future, failed};
use futures::stream::{BoxStream, Stream};

use stream_once::StreamOnce;
use {Name, Address, Error, Resolver, MemResolver};


/// A builder/fluent interface to create a `Router`
pub struct RouterBuilder(Inner);

/// A helper which allows to resolve different names by different means
///
/// Create it using RouterBuilder
///
/// Note: router resolves any name only once. I.e. if name matches a suffix
/// it doesn't get resolved using a fallback resolver even if original resolver
/// returned error.
#[derive(Clone)]
pub struct Router(Arc<Inner>);


struct Inner {
    names: MemResolver,
    suffixes: HashMap<String, Box<Resolver>>,
    fallback: Option<Box<Resolver>>,
}

impl RouterBuilder {
    /// Start creating DNS router object
    pub fn new() -> RouterBuilder {
        RouterBuilder(Inner {
            names: MemResolver::new(),
            suffixes: HashMap::new(),
            fallback: None,
        })
    }
    /// Add a name that is resolved to a single IP (just like in `MemResolver`)
    pub fn add_ip(&mut self, name: &str, ip: IpAddr) -> &mut Self {
        self.0.names.add_host(name, ip);
        self
    }
    /// Add a suffix which will be resolved using specified resolver object
    ///
    /// This is useful for example to resolve all `*.consul` addresses against
    /// consul.
    ///
    /// Suffixes are always matched after dot in the name. Suffixes passed to
    /// to this function must *not* contain initial dot.
    ///
    /// If overlapping suffixes are specified, longest matching suffix wins.
    pub fn add_suffix<S, R>(&mut self, suffix: S, resolver: R) -> &mut Self
        where S: Into<String>, R: Resolver + 'static,
    {
        self.0.suffixes.insert(suffix.into(), Box::new(resolver));
        self
    }
    /// Add a default resolver
    ///
    /// Default resolver works as fallback when neither specific names nor
    /// suffixes matches.
    ///
    /// Note: when suffix matches but returns non existent domain name
    /// the default resolver is *not* called.
    pub fn add_default<R>(&mut self, resolver: R) -> &mut Self
        where R: Resolver + 'static,
    {
        self.0.fallback = Some(Box::new(resolver));
        self
    }
    /// Build real router object which can be used to resolve names
    pub fn into_resolver(self) -> Router {
        Router(Arc::new(self.0))
    }
}

impl Resolver for Router {
    fn resolve(&self, name: &Name) -> BoxFuture<Address, Error> {
        let host = name.host();
        if self.0.names.contains_name(host) {
            return self.0.names.resolve(name);
        }
        if let Some(resolver) = self.0.suffixes.get(host) {
            return resolver.resolve(name);
        } else {
            for (idx, _) in host.match_indices('.') {
                let suffix = &host[idx+1..];
                if let Some(resolver) = self.0.suffixes.get(suffix) {
                    return resolver.resolve(name);
                }
            }
            if let Some(ref resolver) = self.0.fallback {
                return resolver.resolve(name);
            }
        }
        failed(Error::NameNotFound).boxed()
    }
    fn subscribe(&self, name: &Name) -> BoxStream<Address, Error> {
        let host = name.host();
        if self.0.names.contains_name(host) {
            return self.0.names.subscribe(name);
        }
        if let Some(resolver) = self.0.suffixes.get(host) {
            return resolver.subscribe(name);
        } else {
            for (idx, _) in host.match_indices('.') {
                let suffix = &host[idx+1..];
                if let Some(resolver) = self.0.suffixes.get(suffix) {
                    return resolver.subscribe(name);
                }
            }
            if let Some(ref resolver) = self.0.fallback {
                return resolver.subscribe(name);
            }
        }
        StreamOnce::new(failed(Error::NameNotFound)).boxed()
    }
}
