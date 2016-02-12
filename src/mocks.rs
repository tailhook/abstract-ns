use std::hash::Hash;
use std::marker::PhantomData;
use std::collections::HashMap;

use {Resolver, Receiver, BlockingResolver};

/// A mock resolver
///
/// This receives a HashMap or a function and resolve names based on table
/// loopup or a function call.
///
/// # Example With HashMap
///
/// ``` use std::net::Ipv4Addr; use std::collections::HashMap; use
/// abstract_ns::{Mock, BlockingResolver};
///
///     let mut ns = Mock::table(vec![ ("example.com", Ipv4Addr::new(127, 0, 0,
///     1)), ]);
///
///     let ip = ns.resolve("example.com").unwrap(); assert_eq!("127.0.0.1",
///     &format!("{}", ip)); ```
///
/// # Example With Closure
///
/// ``` use std::net::Ipv4Addr; use std::collections::HashMap; use
/// abstract_ns::{Mock, BlockingResolver};
///
///     let mut ns = Mock::new(|n| match n { "example.com" =>
///     Ok(Ipv4Addr::new(127, 0, 0, 1)), _ => Err(()), });
///
///     let ip = ns.resolve("example.com").unwrap(); assert_eq!("127.0.0.1",
///     &format!("{}", ip)); ```
///
/// # Testing
///
/// While there are production use cases of both hash table lookups and
/// functional name lookup, it's advised to implement the trait directly
/// instead of using `Mock` for those cases.
pub struct Mock<N, M>(M, PhantomData<*const N>);

impl<N, M> Mock<N, M> {
    pub fn new(x: M) -> Mock<N, M> {
        Mock(x, PhantomData)
    }
}
impl<N, V> Mock<N, HashMap<N, V>>
    where N: Hash + Eq, V: Clone
{
    pub fn table(items: Vec<(N, V)>) -> Mock<N, HashMap<N, V>> {
        Mock(items.into_iter().collect(), PhantomData)
    }
}

impl<R, N, A> Resolver<R> for Mock<N, HashMap<N, A>>
    where R: Receiver<A, ()>,
          N: Hash + Eq,
          A: Clone,
{
    type Name = N;
    type Address = A;
    type Error = ();
    fn request(&mut self, name: Self::Name, mut dest: R) {
        dest.result(self.0.get(&name).map(|x| x.clone()).ok_or(()));
    }
}

impl<N, A> BlockingResolver for Mock<N, HashMap<N, A>>
    where N: Hash + Eq,
          A: Clone,
{
    type Name = N;
    type Address = A;
    type Error = ();
    fn resolve(&mut self, name: Self::Name) -> Result<A, ()> {
        self.0.get(&name).map(|x| x.clone()).ok_or(())
    }
}

impl<R, N, A, E, F> Resolver<R> for Mock<N, F>
    where R: Receiver<A, E>,
          F: FnMut(N) -> Result<A, E>
{
    type Name = N;
    type Address = A;
    type Error = E;
    fn request(&mut self, name: Self::Name, mut dest: R) {
        dest.result((self.0)(name));
    }
}

impl<N, A, E, F> BlockingResolver for Mock<N, F>
    where F: FnMut(N) -> Result<A, E>
{
    type Name = N;
    type Address = A;
    type Error = E;
    fn resolve(&mut self, name: Self::Name) -> Result<A, E> {
        (self.0)(name)
    }
}

