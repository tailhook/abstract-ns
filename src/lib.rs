//! An abstract name service library
//!
//! The library abstract over DNS and allows easier switching between
//! different nameservices (say Zookeeper or Consul instead of DNS) and
//! provides simple mocks for testing purposes.
//!
//! We're providing simple implementation based on ``std::net`` so you can
//! start using the library without additional dependencies and add support
//! for more name services later.
//!
//! The library works both for synchronous (blocking) name resolution and
//! asynchronous stuff like [rotor]
//!
//! [rotor]: http://rotor.readthedocs.org/
//!

mod channels;
mod stdlib_dns;
mod threaded;
mod async;
mod mocks;

pub use stdlib_dns::StdResolver;
pub use threaded::{ResolverThread, resolver_thread};
pub use async::{AsyncResolver};
pub use mocks::Mock;

pub trait Resolver<R: Receiver<Self::Address, Self::Error>> {
    type Name;
    type Address;
    type Error;
    fn request(&mut self, name: Self::Name, dest: R);
}

pub trait Receiver<A, E> {
    fn result(&mut self, addr: Result<A, E>);
}

/// The traits denotes resolver that blocks and returns data synchronously
///
/// For synchronously applications it's okay to use the type directly for
/// asyncrhonous applications you may put the resolver into thread.
pub trait BlockingResolver {
    type Name;
    type Address;
    type Error;
    fn resolve(&mut self, name: Self::Name)
        -> Result<Self::Address, Self::Error>;
}
