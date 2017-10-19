//! This crate provides a simple name resolver that uses lib's name resolution.
//!
//! Unfortunately libc doesn't provide asyncrhonous name resolution for many
//! reasons so we run requests in a thread pool.
//!
//! For high-performance server applications this way is far from being
//! performant, still it is the most compatible to everything else. So it
//! might be used:
//!
//! 1. To provide maximum compatibility (i.e. good default for dev environment)
//! 2. In applications where name resolution is not slow part
//! 3. As a fallthrough resolver for `ns_router::Router` where more frequently
//!    used name suffixes are overriden with faster resolver for that namespace
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
extern crate futures;
extern crate abstract_ns;
extern crate futures_cpupool;

use std::fmt;
use std::net::ToSocketAddrs;

use futures::Async;
use abstract_ns::{HostResolve, Name, IpList, Error};
use futures_cpupool::{CpuPool, CpuFuture};

/// A resolver that uses ToSocketAddrs from stdlib in thread pool
#[derive(Clone, Debug)]
pub struct ThreadedResolver {
    pool: CpuPool,
}

/// A Future returned from resolver
pub struct Future(CpuFuture<IpList, Error>);

impl ThreadedResolver {
    /// Create a resolver with 8 threads in it's own thread pool
    ///
    /// Use `use_pool` with a configured `CpuPool` to change the
    /// configuration or share thread pool with something else
    pub fn new() -> Self {
        ThreadedResolver {
            pool: CpuPool::new(8),
        }
    }
    /// Create a new Resolver with the given thread pool
    ///
    /// This is often used to share thread pool with other service or to
    /// configure thread pool diffently
    pub fn use_pool(pool: CpuPool) -> Self {
        ThreadedResolver {
            pool: pool,
        }
    }
}


impl futures::Future for Future {
    type Item = IpList;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<IpList>, Error> {
        self.0.poll()
    }
}


impl HostResolve for ThreadedResolver {
    type HostFuture = Future;
    fn resolve_host(&self, name: &Name) -> Future {
        let name = name.clone();
        Future(self.pool.spawn_fn(move || {
            match (name.as_ref(), 0).to_socket_addrs() {
                Ok(it) => Ok(it.map(|sa| sa.ip())
                    .collect::<Vec<_>>().into()),
                Err(e) => Err(Error::TemporaryError(Box::new(e))),
            }
        }))
    }
}

impl fmt::Debug for Future {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ns_std_threaded::Future {{}}")
    }
}
