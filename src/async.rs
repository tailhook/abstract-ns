use std::sync::mpsc::{Receiver, SyncSender, sync_channel};

use {Resolver, Receiver as RecvTrait};

/// Async resolver extension trait
///
/// This is a convenience trait for using asynchronous resolvers using
/// blocking thread model.
pub trait AsyncResolver<R: RecvTrait<Self::Address, Self::Error>>: Resolver<R>
{
    fn resolve_async(&mut self, name: Self::Name)
        -> Receiver<Result<Self::Address, Self::Error>>;
}

impl<A, E, T> AsyncResolver<SyncSender<Result<A, E>>> for T
    where T: Resolver<SyncSender<Result<A, E>>, Address=A, Error=E>
{
    fn resolve_async(&mut self, name: Self::Name)
        -> Receiver<Result<A, E>>
    {
        let (tx, rx) = sync_channel(1);
        self.request(name, tx);
        return rx;
    }
}
