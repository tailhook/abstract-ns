//! Abstract traits for name service library
//!
//! Here are small breakdown:
//!
//! * `Name` is basically `AsRef<str>` but we would like to keep it as a trait
//!   for later extension
//! * `Address` is a structure which address resolves to. It's more than just
//!   a `SocketAddr` you have used to, but there is `Address::pick_addr()`
//!   which does quick and dirty solution
//! * `Resolver` is a structure which is a configured resolver. You may use
//!   many different resolvers in your application.
//!   Use `Resolver::resolve(name)` to get a future `Resolver::subscribe(name)`
//!   to get a stream of updates.
//! * `StubResolver` is an in-memory hash table for addresses you may use for
//!   tests
//!

extern crate futures;
#[macro_use] extern crate quick_error;

pub type Name<'a> = &'a str;
pub type Weight = u32;

mod address;
mod resolver;
mod error;
mod stub;

pub use address::{Address, AddressBuilder};
pub use resolver::Resolver;
pub use error::Error;
pub use stub::StubResolver;
