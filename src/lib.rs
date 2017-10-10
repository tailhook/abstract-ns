//! Abstract traits for name service library
//!
//! # Traits
//!
//! There are four traits:
//!
//! * [`ResolveHost`](trait.ResolveHost.html)
//!   -- resolves hostname to a list of IP addresses
//!   (maps to `A` record in DNS)
//! * [`Resolve`](trait.Resolve.html)
//!   -- resolves service name to a set of weighted and prioritized host:port
//!   pairs ([`Address`](struct.Address.html) struct).
//!   Maps to `SRV` record in DNS.
//! * [`HostSubscribe`](trait.HostSubscribe.html)
//!   -- resolves hostname to a list of IP addresses and tracks changes of
//!   the addresses
//! * [`Subscribe`](trait.Subscribe.html)
//!   -- resolves service name to an [`Address`](struct.Address.html) and
//!   subscribes on updates of the address
//!
//! And there are two address types:
//!
//! * [`IpList`](ip_list/struct.IpList.html) -- represents `Arc<Vec<IpAddr>>`
//! this is used as a result of hostname resolution and it should be converted
//! into an `Address` struct.
//! * [`Address`](addr/struct.Address.html) -- represets weighed and
//! prioritized list of addresses, this is what all user code should accept
//! for maximum flexibility.
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
//! The `Resolve*` traits are used for ad-hoc resolution of the addresses.
//!
//! The `*Subscribe` traits are used to get updates for the name. If your
//! name service supports updates you should implement it. If not, there
//! are shims which periodically poll `resolve*` methods to get the
//! update functionality.
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
extern crate void;
#[macro_use] extern crate quick_error;

mod error;
mod name;
mod resolver;
pub mod addr;
pub mod ip_list;
// pub mod mem;  # Temporary

pub use addr::Address;
pub use ip_list::IpList;
pub use error::Error;
pub use name::Name;
pub use resolver::{ResolveHost, Resolve, HostSubscribe, Subscribe};
