//! Abstract traits for name service library
//!
//! # Traits
//!
//! There are four traits:
//!
//! * ResolveHost
//! * Resolve
//! * HostSubscribe
//! * Subscribe
//!
//! There are three category of users of the library:
//!
//! * Implementors of resolution methods
//! * Service authors
//! * Application writers
//!
//! Let's explain how to use traits to all of them.
//!
//! # Implementing A Resolver
//!
//! To implement a DNS resolver library implement
//! [`Resolver`](trait.Resolver.html) or
//! [`PollResolver`](trait.PollResolver.html) traits
//!
//! In particular, [`Resolver`](trait.Resolver.html) allows to
//! [subscribe](trait.Resolver.html#tymethod.subscribe) on
//! updates, and this is very important for long-running servers.
//!
//! On the other hand, if you don't have to implement `Resolver` if your
//! data source doesn't support subscribing for updates. We have all the
//! needed abstractions to periodially poll for updates.
//!
//! Resolver should do minimum work. I.e. if it uses DNS subsystem, it should
//! only check DNS, and give `/etc/hosts` parsing ot other parts of the stack.
//!
//! # Writing Protocols
//!
//! In general, your library should depend on a minimum set of runctionality
//! here. Here are the rules of thumb:
//!
//! 1. Clients: when you need to connect once, accept
//!    `T: Future<Item=SocketAddr>`, there are adapters that pick a random
//!    host from `Future<Item=Address>` returned by `PollResolver::resolve`
//! 2. Clients: when writing a connection pool, accept
//!    `T: Stream<Item=Address>`, there are adapters to make that stream
//!    by resolving a single name (into potentially multiple IP addresses),
//!    a list of names, and a `Stream<Item=Vec<Name>>` (so that config is
//!    adaptable). As well as adapters that help diffing the `Address`,
//!    effectively allowing connection pool to adapt.
//! 3. Servers: accept `T: AsyncRead + AsyncWrite`, we have
//!    [`tk-listen`](https://crates.io/crates/tk-listen) crate
//!    that can turn all kinds of configuration into actually accepted
//!    connections.
//!
//! # Writing Applications
//!
//! Applications should use `ns-router` crate that supports multiple resolvers,
//! and configuring them on-the-fly.
//!
//! # Writing Connection Pools
//!
//! As said in [Writing Protocols](#writing-protocols) section a single
//! connection pool should use `T: Stream<Item=Address>` for as a name
//! source, this allows good flexibility
//!
//! But in case you need kinda connection pool to a lot of different names
//! and services, this is the good case for accepting `Resolver` trait itself.
//! (Still, most of the time actual application should supply
//!  `ns_router::Router`)
//!
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

extern crate futures;
extern crate rand;
#[macro_use] extern crate quick_error;

mod error;
mod name;
mod resolver;
pub mod addr;
// pub mod mem;  # Temporary

pub use addr::{Address};
pub use error::Error;
pub use name::Name;
pub use resolver::{ResolveHost, Resolve, HostSubscribe, Subscribe};
