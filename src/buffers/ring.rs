//! Implements ring or cyclic buffers.
//!
//! These are meant to be used when you continually need to store the last few samples output from
//! some signal, such as for a delay effect.
//!
//! ## Kinds of ring buffers
//!
//! We implement two kinds of ring buffers:
//!
//! - [`Shift`] buffers push new data and the start, and shift all pre-existing data by one entry.
//!   These are fast when they are very small (≤ 4), but are entirely impractical for larger
//!   buffers.
//! - [`Circ`] buffers write data into consecutive positions of a buffer, then loop around when they
//!   reach the end. The branching incurs some small overhead, but this is well worth it for large
//!   buffers.
//!
//! ## Todo
//!
//! Actually test that [`Shift`] buffers are more efficient!

use crate::{mod_inc, prelude::*};
use buf::Ref;

/// A trait for ring or cyclic buffers.
///
/// See the [module docs](self) for more info.
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

    /// Pushes a value into the buffer, phasing out some older value.
    fn push(&mut self, value: <Self::Buf as buf::Ref>::Item);

    /// Loads `count` samples from a signal, phasing out the old ones.
    fn push_many<S: SignalMut<Sample = <Self::Buf as buf::Ref>::Item>>(
        &mut self,
        sgn: &mut S,
        count: usize,
    ) {
        for _ in 0..count {
            self.push(sgn.next());
        }
    }

    /// Gets the `n`-th previously pushed value from the buffer.
    ///
    /// ## Panics
    ///
    /// Panics if `index` is greater than the capacity.
    fn get(&self, index: usize) -> <Self::Buf as buf::Ref>::Item;

    // TODO: get_mut?

    /// Gets the last pushed value from the buffer.
    fn fst(&self) -> <Self::Buf as buf::Ref>::Item {
        self.get(0)
    }
}

/// A ring buffer that inserts new data at the start, and shifts all previously written data when
/// doing so.
///
/// This is somewhat more efficient than [`Circ`] for small buffers, but the cost quickly adds up.
#[derive(Clone, Copy, Debug, Default)]
pub struct Shift<B: buf::Mut> {
    /// The inner buffer.
    pub inner: B,
}

impl<B: buf::Mut> Shift<B> {
    /// Initializes a new [`Shift`] buffer.
    pub const fn new(inner: B) -> Self {
        Self { inner }
    }

    /// Returns a reference to the inner buffer.
    pub const fn inner(&self) -> &B {
        &self.inner
    }
}

/// An auxiliary function that copies `index` to `index + count`.
///
/// Useful since rust-analyzer complains a lot about these lines if written in their simplest form.
fn shift<R: Ring>(ring: &mut R, index: usize, count: usize) {
    let buf = ring.buffer_mut().as_mut();
    *buf.get_mut(index + count).unwrap() = buf[index];
}

impl<B: buf::Mut> Ring for Shift<B> {
    type Buf = B;

    fn buffer(&self) -> &Self::Buf {
        &self.inner
    }

    fn buffer_mut(&mut self) -> &mut Self::Buf {
        &mut self.inner
    }

    fn get(&self, index: usize) -> <Self::Buf as buf::Ref>::Item {
        self.inner[index]
    }

    /// This has linear time complexity on the size of the buffer, so be careful!
    fn push(&mut self, value: B::Item) {
        for i in (1..self.capacity()).rev() {
            shift(self, i - 1, 1);
        }

        self.inner[0] = value;
    }

    fn push_many<S: SignalMut<Sample = <Self::Buf as buf::Ref>::Item>>(
        &mut self,
        sgn: &mut S,
        count: usize,
    ) {
        // We optimize by performing all shifts at once.
        // `new` is the number of new entries.

        let n = self.capacity();
        let new;
        if count < n {
            new = count;
        } else {
            for _ in 0..(count - n) {
                sgn.advance();
            }
            new = n;
        }

        for val in self.inner.as_mut_slice()[..new].iter_mut().rev() {
            *val = sgn.next();
        }
    }
}

/// A ring buffer that inserts new data at consecutive positions, looping back to the start after
/// reaching the end.
///
/// For very small buffers, [`Shift`] might be more efficient.
#[derive(Clone, Debug, Default)]
pub struct Circ<B: buf::Mut> {
    /// The inner buffer.
    pub inner: B,

    /// The current position to write data.
    pos: usize,
}

impl<B: buf::Mut> Circ<B> {
    /// Initializes a new [`Circ`] buffer.
    pub const fn new(inner: B) -> Self {
        Self { inner, pos: 0 }
    }
}

impl<B: buf::Mut> Ring for Circ<B> {
    type Buf = B;

    fn buffer(&self) -> &Self::Buf {
        &self.inner
    }

    fn buffer_mut(&mut self) -> &mut Self::Buf {
        &mut self.inner
    }

    fn get(&self, index: usize) -> <Self::Buf as buf::Ref>::Item {
        let index = if self.pos > index {
            self.pos - index
        } else {
            self.pos + self.capacity() - index
        };

        self.inner[index]
    }

    fn push(&mut self, value: B::Item) {
        let pos = self.pos;
        self.buffer_mut()[pos] = value;
        mod_inc(self.capacity(), &mut self.pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Tests shift buffers.
    fn shift() {
        let mut shift = Shift::new(buf::Stc::from_data(smp::Mono::array([1.0, 2.0, 3.0])));
        shift.push(smp::Mono::new(4.0));
        assert_eq!(shift.buffer().as_ref(), &smp::Mono::array([4.0, 1.0, 2.0]));
    }
}
