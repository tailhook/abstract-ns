//! Abstract traits for name service library
//!
//! Here are small breakdown:
//!
//! * `Name` is currently an alias to `&str` (but it may change in future)
//! * `Address` is a structure which address resolves to. It's more than just
//!   a `SocketAddr` you have used to, but there is `Address::pick_one()`
//!   which does quick and dirty solution
//! * `Resolver` is a structure which is a configured resolver. You may use
//!   many different resolvers in your application.
//!   Use `Resolver::resolve(name)` to get a future `Resolver::subscribe(name)`
//!   to get a stream of updates.
//! * `MemResolver` is an in-memory hash table for addresses you may use for
//!   tests
//!
#![deny(missing_docs)]

extern crate futures;
extern crate rand;
#[macro_use] extern crate quick_error;

/// A type alias that represents a name resolved by a name service
pub type Name<'a> = &'a str;
/// A type alias for a weight for each name in an address
///
/// (don't rely on actual type, it's likely to change in near future)
pub type Weight = u64;

mod address;
mod resolver;
mod error;
mod mem;
mod stream_once;
mod routing;

pub use address::{Address, AddressBuilder, WeightedSet};
pub use resolver::Resolver;
pub use error::Error;
pub use mem::MemResolver;
pub use routing::{RouterBuilder, Router};


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
