//! Abstract traits for name service library
//!
//! Here are small breakdown:
//!
//! * `Address` is a structure which address resolves to. It's more than just
//!   a `SocketAddr` you have used to, but there is `Address::pick_one()`
//!   which does quick and dirty solution
//! * `Resolver` is a structure which is a configured resolver. You may use
//!   many different resolvers in your application.
//!   Use `Resolver::resolve(name)` to get a future `Resolver::subscribe(name)`
//!   to get a stream of updates.
//! * `MemResolver` is an in-memory hash table for addresses you may use for
//!   tests
//! * `Router` is a way to specify different sources for different names, for
//!   example serve `*.consul` from local consul, other things from
//!   conventional DNS servers
//!
#![deny(missing_docs)]

extern crate futures;
extern crate rand;
extern crate void;
#[macro_use] extern crate quick_error;

/// A type alias for a weight for each name in an address
///
/// (don't rely on actual type, it's likely to change in near future)
pub type Weight = u64;

mod address;
mod resolver;
mod error;
mod mem;
mod name;

pub use address::{Address};
pub use address::{AddressBuilder, AddressIter, PriorityIter};
pub use address::{WeightedSet};
pub use error::Error;
pub use mem::{MemResolver, MemSubscription};
pub use name::Name;
pub use resolver::{Resolver, PollResolver};
