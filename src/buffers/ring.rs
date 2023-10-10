//! Implements ring or cyclic buffers.
//!
//!

use crate::prelude::*;
use buf::Ref;

/// A trait for ring or cyclic buffers.
///
///
pub trait Ring {
    /// The backing buffer type.
    type Buf: buf::Mut;

    /// Returns a reference to the backing buffer.
    fn buffer(&self) -> &Self::Buf;

    /// Returns a mutable reference to the backing buffer.
    fn buffer_mut(&mut self) -> &mut Self::Buf;

    /// The capacity of the buffer.
    ///
    /// This is the number of samples that can be pushed before the first one is deleted.
    fn capacity(&self) -> usize {
        self.buffer().len()
    }

    /// Pushes a value into the buffer.
    ///
    /// This will displace some earlier value.
    fn push(&mut self);

    /// Gets the `n`-th previously pushed value from the buffer.
    ///
    /// ## Panics
    ///
    /// Panics if `index` is greater than the capacity.
    fn get(&self, index: usize) -> <Self::Buf as buf::Ref>::Item;

    /// Gets the last pushed value from the buffer.
    fn fst(&self) -> <Self::Buf as buf::Ref>::Item {
        self.get(0)
    }
}
