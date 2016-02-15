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
mod identity;

pub use stdlib_dns::StdResolver;
pub use threaded::{ResolverThread, resolver_thread};
pub use async::{AsyncResolver};
pub use mocks::Mock;
pub use identity::IdentityResolver;

/// A traits that encapsulates name resolution
///
/// If your're implementing a name service you should implement this trait.
pub trait Resolver<R: Receiver<Self::Address, Self::Error>> {
    /// The name that can be resolved via this name service. Note in many
    /// cases type should implement Send
    type Name;
    /// The value of the address returned back
    type Address;
    /// Error that name resolver can return
    type Error;
    /// The method which does name resolution
    ///
    /// When name resolution is done, you should use `dest.result(x)` to
    /// deliver result.
    ///
    /// This trait is inherently asynchronous. For synchronous (blocking)
    /// implementations you should use `BlockingResolver`. This gives clear
    /// indicator for user that process may be slow, so they can offload it to
    /// thread if needed.
    fn request(&mut self, name: Self::Name, dest: R);
}

/// A helper trait used to deliver final name resolve result
///
/// This trait is implemented on rust channels, and should be implemented for
/// popular asynchronous main loops.
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
