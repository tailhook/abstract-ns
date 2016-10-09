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
//! * `StubResolver` is an in-memory hash table for addresses you may use for
//!   tests
//!

extern crate futures;
extern crate rand;
#[macro_use] extern crate quick_error;

pub type Name<'a> = &'a str;
pub type Weight = u32;

mod address;
mod resolver;
mod error;
mod stub;
mod into_stream;  // Temporary

pub use address::{Address, AddressBuilder};
pub use resolver::Resolver;
pub use error::Error;
pub use stub::StubResolver;
