//! This crate provides a simple name resolver that based on `domain` crate
//!
//! The `domain` crate is made as a one-stop thing for any DNS. Still
//! `abstract-ns` provides subscriptions for updates and service discovery
//! based on different services (and mapping between names and resolvers).
//!
//! Use this crate:
//!
//! 1. As a more efficient `ns-std-threaded` (with care!)
//! 2. For DNS-based name resolution (no SRV yet)
//!
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
extern crate futures;
extern crate abstract_ns;
extern crate domain;
extern crate tokio_core;

use std::fmt;
use std::str::FromStr;
use std::net::{IpAddr};
use std::error::Error as StdError;

use futures::{Future, Async};
use tokio_core::reactor::Handle;
use domain::resolv;
use domain::iana::{Rtype, Class};
use domain::rdata::A;
use domain::bits::{Question, DNameBuf};
use abstract_ns::{Name, IpList, Error};


/// A main DNS resolver which implements all needed resolver traits
#[derive(Clone, Debug)]
pub struct DnsResolver {
    internal: resolv::Resolver,
}

/// A Future returned by `DnsResolver::resolve_host`
pub struct HostFuture {
    name: Name,
    query: Option<resolv::Query>,
    error: Option<Error>,
}


impl fmt::Debug for HostFuture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HostFuture")
        .field("name", &self.name)
        .finish()
    }
}

impl Future for HostFuture {
    type Item = IpList;
    type Error = Error;
    fn poll(&mut self) -> Result<Async<IpList>, Error> {
        if let Some(err) = self.error.take() {
            return Err(err);
        }
        match self.query.as_mut().expect("future polled twice").poll() {
            Ok(Async::Ready(buf)) => {
                let answer = match buf.answer() {
                    Ok(ans) => ans,
                    Err(e) => {
                        return Err(Error::TemporaryError(
                            Box::new(e)));
                    }
                };
                let mut result = Vec::new();
                for ip_res in answer.limit_to::<A>() {
                    match ip_res {
                        Ok(ip) => result.push(
                            IpAddr::V4(ip.data().addr())),
                        Err(e) => {
                            return Err(Error::TemporaryError(
                                Box::new(e)))
                        }
                    }
                }
                Ok(Async::Ready(result.into()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(resolv::error::Error::Question(_)) |
            Err(resolv::error::Error::NoName) => {
                Err(Error::InvalidName(self.name.to_string(),
                    "resolv::error::Error::Question"))
            }
            Err(e) => {
                // TODO(tailhook) what returned if
                // there is no such name? Is it success?
                Err(Error::TemporaryError(Box::new(e)))
            }
        }
    }
}

impl DnsResolver {
    /// Create a DNS resolver with system config
    pub fn system_config(lp: &Handle) -> Result<DnsResolver, Box<StdError>> {
        Ok(DnsResolver {
            internal: resolv::Resolver::new(lp),
        })
    }
    /// Create a resolver from `domain::resolv::Resolver` instance
    ///
    /// This method provides the most comprehensive configuration
    pub fn new_from_resolver(internal: resolv::Resolver) -> DnsResolver {
        DnsResolver {
            internal: internal,
        }
    }
}

impl abstract_ns::ResolveHost for DnsResolver {
    type FutureHost = HostFuture;
    fn resolve_host(&self, name: &Name) -> HostFuture {
        match DNameBuf::from_str(&format!("{}.", name)) {
            Ok(dname) => {
                HostFuture {
                    name: name.clone(),
                    query: Some(self.internal.clone()
                        .query(Question::new(dname, Rtype::A, Class::In))),
                    error: None,
                }
            }
            Err(_) => {
                HostFuture {
                    name: name.clone(),
                    query: None,
                    error: Some(Error::InvalidName(name.to_string(),
                    "domain::resolv::DNameBuf::from_str failed")),
                }
            }
        }
    }
}
