use std::sync::mpsc::{Sender, SyncSender};
use std::sync::mpsc::TrySendError::{Disconnected, Full};


use {Receiver};


impl<A, E> Receiver<A, E> for SyncSender<Result<A, E>> {
    fn result(&mut self, addr: Result<A, E>) {
        match self.try_send(addr) {
            Ok(()) => {}
            // If receiver is already dead its fine,
            Err(Disconnected(_)) => {}
            // When channel is full, it's probably a bug
            Err(Full(_)) => {
                panic!("Name service receiver channel is full. \
                    Probably the capacity is too small. \
                    Alternatively use asynchrounous channel.")
            }
        }
    }
}

impl<A, E> Receiver<A, E> for Sender<Result<A, E>> {
    fn result(&mut self, addr: Result<A, E>) {
        // If receiver is already dead its fine
        self.send(addr).ok();
    }
}
