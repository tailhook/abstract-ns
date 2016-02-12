use std::thread::{JoinHandle, spawn};
use std::sync::mpsc::{channel, Sender};


use {BlockingResolver, Resolver, Receiver};


#[derive(Clone)]
pub struct ResolverThread<N:Send, R:Send, A:Send, E:Send>(Sender<(N, R)>)
    where R: Receiver<Address=A, Error=E>;

impl<N, R, A, E> Resolver<R> for ResolverThread<N, R, A, E>
    where R: Receiver<Address=A, Error=E>, N: Send, R: Send, A: Send, E: Send
{
    type Name = N;
    fn request(&mut self, name: Self::Name, dest: R) {
        self.0.send((name, dest))
            .expect("resolver thread is already shut down");
    }
}

pub fn resolver_thread<F, B, N, R, A, E>(fun: F)
    -> (JoinHandle<()>, ResolverThread<N, R, A, E>)
    where F: FnOnce() -> B + Send + 'static,
          B: BlockingResolver<Name=N, Address=A, Error=E> + Sized,
          R: Receiver<Address=A, Error=E>,
          N: Send + 'static, R: Send + 'static, A: Send, E: Send
{
    let (tx, rx) = channel::<(N, R)>();
    let handle = spawn(move || {
        let mut resolver = fun();
        for (name, mut out) in rx.into_iter() {
            out.result(resolver.resolve(name));
        }
    });
    return (handle, ResolverThread(tx));
}
