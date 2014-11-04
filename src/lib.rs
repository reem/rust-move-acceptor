#![license = "MIT"]
#![deny(missing_docs)]
#![deny(warnings)]

//! A variant of the Acceptor trait which moves self in the `move_incoming` method.
//!
//! This creates an iterator which fulfills 'static.

use std::io::{Acceptor, IoResult};

/// A variant of the Acceptor trait which moves self in the `move_incoming` method.
pub trait MoveAcceptor<T>: Acceptor<T> {
    /// Wait for and accept an incoming connection.
    fn maccept(&mut self) -> IoResult<T> { self.accept() }

    /// Get an Iterator over incoming connections.
    fn move_incoming(self) -> MoveConnections<Self> {
        MoveConnections { underlying: self }
    }
}

/// An iterator over incoming connections.
pub struct MoveConnections<T> {
    underlying: T
}

impl<T, A: Acceptor<T>> Iterator<IoResult<T>> for MoveConnections<A> {
    fn next(&mut self) -> Option<IoResult<T>> {
        Some(self.underlying.accept())
    }
}

impl<A, T: Acceptor<A>> MoveAcceptor<A> for T {}

