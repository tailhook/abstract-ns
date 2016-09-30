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
//!

extern crate futures;
#[macro_use] extern crate quick_error;

pub type Weight = u32;

mod name;
mod address;
mod resolver;
mod error;

pub use name::Name;
pub use address::{Address, AddressBuilder};
pub use resolver::Resolver;
pub use error::Error;
