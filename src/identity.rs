use std::marker::PhantomData;

use {Resolver, Receiver};

/// The stub resolver that resolves a type into same type
///
/// This is useful if you start without name resolving and want to supply
/// just address. I.e. `IdentityResolver<SocketAddr>` may be used when you
/// want to supply socket address directly. But will replace it with better
/// name resolution at future time.
///
/// Also it might be usedful when you're doing composition of name resolvers
/// and you might use identity resolver when user specified an IP address on
/// the command-line, and other resolver when name was specified.
///
/// Additionally it may be used in unit tests where `Mock` resolver is an
/// overkill
pub struct IdentityResolver<T>(PhantomData<*const T>);

impl<T> IdentityResolver<T> {
    pub fn new() -> IdentityResolver<T> {
        IdentityResolver(PhantomData)
    }
}

impl<T, R: Receiver<T, ()>> Resolver<R> for IdentityResolver<T> {
    type Name = T;
    type Address = T;
    type Error = (); // this should be void, but we avoid that dependency
    fn request(&mut self, name: T, mut dest: R) {
        dest.result(Ok(name))
    }
}
