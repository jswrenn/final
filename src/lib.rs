#![no_std]

use core::ops::Deref;

/// This no-std crate provides the `Final`, struct which guarantees the interior
/// immutability of the value that it wraps. This is useful for preserving
/// invariants on the fields of structures, whose 'safe' mutation would cause
/// undefined behavior.

// https://www.reddit.com/r/rust/comments/26jzm5/immutable_struct_members_in_rust/chs3ljz/

/// Wraps a value in the `Final` type, which does not give out mutable
/// references.
#[derive(Debug)]
#[repr(transparent)]
pub struct Final<T>(T);

impl<T> Final<T> {
    /// Wrap a value `v` in a `Final`
    #[inline]
    pub fn new(v: T) -> Final<T> {
        Final(v)
    }
}

impl<T> Deref for Final<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
