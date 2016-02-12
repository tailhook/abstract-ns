use std::sync::mpsc::{Receiver, SyncSender, sync_channel};

use {Resolver};

/// Async resolver extension trait
///
/// This is a convenience trait for using asynchronous resolvers using
/// blocking thread model.
pub trait AsyncResolver<A, E>: Resolver<SyncSender<Result<A, E>>> {
    fn resolve_async(&mut self, name: Self::Name)
        -> Receiver<Result<A, E>>
    {
        let (tx, rx) = sync_channel::<Result<A, E>>(1);
        self.request(name, tx);
        return rx;
    }
}

impl<A, E, T: Resolver<SyncSender<Result<A, E>>>> AsyncResolver<A, E> for T {}
