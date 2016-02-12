mod channels;
mod stdlib_dns;

pub use stdlib_dns::StdResolver;

pub trait Resolver<R: Receiver> {
    type Name;
    fn request(&mut self, name: Self::Name, dest: R);
}

pub trait Receiver {
    type Address;
    type Error;
    fn result(&mut self, addr: Result<Self::Address, Self::Error>);
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
