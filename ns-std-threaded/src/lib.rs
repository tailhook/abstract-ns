extern crate futures;
extern crate abstract_ns;
extern crate futures_cpupool;

use std::net::ToSocketAddrs;

use futures::{BoxFuture, failed, Future};
use abstract_ns::{Resolver, Name, Address, Error};
use futures_cpupool::CpuPool;

/// A resolver that uses ToSocketAddrs from stdlib in thread pool
#[derive(Clone)]
pub struct ThreadedResolver {
    pool: CpuPool,
}

impl ThreadedResolver {
    /// Create a new Resolver with the given thread pool
    pub fn new(pool: CpuPool) -> Self {
        ThreadedResolver {
            pool: pool,
        }
    }
}

fn parse_name(name: &str) -> Option<(&str, Option<u16>)> {
    if let Some(idx) = name.find(':') {
        match name[idx+1..].parse() {
            Ok(port) => Some((&name[..idx], Some(port))),
            Err(_) => None,
        }
    } else {
        Some((name, None))
    }
}

impl Resolver for ThreadedResolver {
    fn resolve(&self, name: Name) -> BoxFuture<Address, Error> {
        match parse_name(name) {
            Some((_, None)) => {
                failed(Error::InvalidName(name.to_string(),
                    "default port must be specified for stub resolver"))
                    .boxed()
            }
            Some((host, Some(port))) => {
                let host = host.to_string();
                self.pool.spawn_fn(move || {
                    match (&host[..], port).to_socket_addrs() {
                        Ok(it) => Ok(it.collect()),
                        Err(e) => Err(Error::TemporaryError(Box::new(e))),
                    }
                }).boxed()
            }
            None => {
                failed(Error::InvalidName(name.to_string(),
                    "default port can't be parsed"))
                    .boxed()
            }
        }
    }
}
