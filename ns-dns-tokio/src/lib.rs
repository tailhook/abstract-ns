extern crate futures;
extern crate abstract_ns;
extern crate domain;

use std::str::FromStr;
use std::net::{IpAddr, SocketAddr};

use futures::{BoxFuture, Future, failed};
use domain::resolv;
use domain::iana::{RRType, Class};
use domain::rdata::A;
use domain::bits::DNameBuf;
use abstract_ns::{Name, Address, Error};

pub struct DnsResolver {
    internal: resolv::Resolver,
}


impl DnsResolver {
    pub fn new(internal: resolv::Resolver) -> DnsResolver {
        DnsResolver {
            internal: internal,
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

impl abstract_ns::Resolver for DnsResolver {
    fn resolve(&self, name: Name) -> BoxFuture<Address, Error> {
        match parse_name(name) {
            Some((_, None)) => {
                failed(Error::InvalidName(name.to_string(),
                    "SRV records are not supported, \
                        please specify default port"))
                    .boxed()
            }
            Some((host, Some(port))) => {
                match DNameBuf::from_str(&format!("{}.", host)) {
                    Ok(dname) => {
                        // TODO(tailhook) optimize this clone
                        let name = name.to_string();

                        self.internal.start().and_then(move |resolv| {
                            resolv.query(dname, RRType::A, Class::In)
                        }).map_err(|e| {
                            match e {
                                resolv::Error::Question(_) |
                                resolv::Error::NoName => {
                                    Error::InvalidName(name,
                                        "resolv::Error::Question")
                                }
                                e @ _ => {
                                    // TODO(tailhook) what returned if
                                    // there is no such name? Is it success?
                                    Error::TemporaryError(Box::new(e))
                                }
                            }
                        })
                        .and_then(move |buf| {
                            let answer = match buf.answer() {
                                Ok(ans) => ans,
                                Err(e) => {
                                    return Err(Error::TemporaryError(
                                        Box::new(e)));
                                }
                            };
                            let mut result = Vec::new();
                            for ip_res in answer.iter::<A>() {
                                match ip_res {
                                    Ok(ip) => result.push(SocketAddr::new(
                                        IpAddr::V4(ip.rdata().addr()),
                                        port)),
                                    Err(e) => {
                                        return Err(Error::TemporaryError(
                                            Box::new(e)))
                                    }
                                }
                            }
                            return Ok(result.into_iter().collect())
                        }).boxed()
                    }
                    Err(_) => {
                        // TODO(tailhook) propagate real error
                        failed(Error::InvalidName(name.to_string(),
                            "domain::resolv::DNameBuf::from_str failed"))
                        .boxed()
                    }
                }
            }
            None => {
                failed(Error::InvalidName(name.to_string(),
                    "default port can't be parsed"))
                    .boxed()
            }
        }
    }
}
