extern crate futures;
extern crate abstract_ns;
extern crate futures_cpupool;

use std::net::ToSocketAddrs;

use futures::Async;
use abstract_ns::{ResolveHost, Name, IpList, Error};
use futures_cpupool::{CpuPool, CpuFuture};

/// A resolver that uses ToSocketAddrs from stdlib in thread pool
#[derive(Clone)]
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


impl ResolveHost for ThreadedResolver {
    type FutureHost = Future;
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
