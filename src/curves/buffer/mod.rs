//! Defines different types for audio buffers.
//!
//! We distinguish three different kinds of buffers: those that hold a reference to its data, those
//! that hold a mutable reference to its data, and those that own its data.

use std::ops::{Index, IndexMut};

use crate::prelude::*;

#[cfg(feature = "hound")]
pub mod wav;

/// A trait common to all buffers.
pub trait BufTrait: AsRef<[Self::Item]> + std::ops::Index<usize, Output = Self::Item> {
    /// The type of sample stored in the buffer.
    type Item: Audio;

    /// Returns the length of the buffer.
    fn len(&self) -> usize {
        self.as_ref().len()
    }

    /// Returns whether the buffer is empty.
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    /// Returns the time that takes to play this buffer.
    #[must_use]
    fn time(&self) -> Time {
        Time::new(crate::units::FracInt::new(self.len() as u64))
    }

    /// Returns the inner slice.    
    #[must_use]
    fn as_slice(&self) -> &[Self::Item] {
        self.as_ref()
    }

    /// Gets a sample at a given index.
    #[must_use]
    fn get(&self, index: usize) -> Option<Self::Item> {
        self.as_ref().get(index).copied()
    }

    /// Returns the sample corresponding to peak amplitude on all channels.
    #[must_use]
    fn peak(&self) -> <Self::Item as ArrayLike>::Array<Vol> {
        /// Prevent code duplication.
        fn peak<A: Audio>(buf: &[A]) -> <A as ArrayLike>::Array<Vol> {
            let mut res = A::new_default();

            for sample in buf {
                A::for_each(|index| {
                    let peak = &mut res[index];
                    let new = sample[index].abs();

                    if *peak > new {
                        *peak = new;
                    }
                });
            }

            res.map_array(|&x| Vol::new(x))
        }

        peak(self.as_ref())
    }

    /// Calculates the RMS on all channels.
    #[must_use]
    fn rms(&self) -> <Self::Item as ArrayLike>::Array<Vol> {
        /// Prevent code duplication.
        fn rms<A: Audio>(buf: &[A]) -> <A as ArrayLike>::Array<Vol> {
            let mut res: <A as ArrayLike>::Array<f64> = ArrayLike::new_default();

            for sample in buf {
                A::for_each(|index| {
                    let new = sample[index];
                    res[index] += new * new;
                });
            }

            // Precision loss should not occur in practice.
            #[allow(clippy::cast_precision_loss)]
            A::for_each(|index| {
                res[index] = (res[index] / buf.len() as f64).sqrt();
            });

            res.map_array(|&x| Vol::new(x))
        }

        rms(self.as_ref())
    }
}

/// A trait for buffers that hold a mutable reference to its data.
pub trait BufMutTrait:
    BufTrait + AsMut<[Self::Item]> + std::ops::IndexMut<usize, Output = Self::Item>
{
    /// Returns a mutable reference to the inner slice.
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut()
    }

    /// Gets a mutable reference to a sample at a given index.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        self.as_mut().get_mut(index)
    }
}

/// A buffer that holds a reference to its data.
#[derive(Clone, Debug)]
pub struct BufRef<'a, A: Audio> {
    pub data: &'a [A],
}

/// A buffer that holds a mutable reference to its data.
#[derive(Debug)]
pub struct BufMut<'a, A: Audio> {
    pub data: &'a mut [A],
}

/// A sample buffer that owns its data.
#[derive(Clone, Debug, Default)]
pub struct Buffer<A: Audio> {
    /// The data stored by the buffer.
    data: Vec<A>,
}

impl<'a, A: Audio> AsRef<[A]> for BufRef<'a, A> {
    fn as_ref(&self) -> &[A] {
        self.data
    }
}

impl<'a, A: Audio> AsRef<[A]> for BufMut<'a, A> {
    fn as_ref(&self) -> &[A] {
        self.data
    }
}

impl<A: Audio> AsRef<[A]> for Buffer<A> {
    fn as_ref(&self) -> &[A] {
        &self.data
    }
}

impl<'a, A: Audio> AsMut<[A]> for BufMut<'a, A> {
    fn as_mut(&mut self) -> &mut [A] {
        self.data
    }
}

impl<A: Audio> AsMut<[A]> for Buffer<A> {
    fn as_mut(&mut self) -> &mut [A] {
        &mut self.data
    }
}

impl<'a, A: Audio> Index<usize> for BufRef<'a, A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<'a, A: Audio> Index<usize> for BufMut<'a, A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<A: Audio> Index<usize> for Buffer<A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<'a, A: Audio> IndexMut<usize> for BufMut<'a, A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl<A: Audio> IndexMut<usize> for Buffer<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut()[index]
    }
}

impl<'a, A: Audio> BufTrait for BufRef<'a, A> {
    type Item = A;
}

impl<'a, A: Audio> BufTrait for BufMut<'a, A> {
    type Item = A;
}

impl<A: Audio> BufTrait for Buffer<A> {
    type Item = A;
}

impl<'a, A: Audio> BufRef<'a, A> {
    /// Initializes a new [`BufRef`].
    pub const fn new(data: &'a [A]) -> Self {
        Self { data }
    }
}

impl<'a, A: Audio> BufMut<'a, A> {
    /// Initializes a new [`BufMut`].
    pub fn new(data: &'a mut [A]) -> Self {
        Self { data }
    }

    /// Converts `self` into a `BufRef`.
    ///
    /// Notes that this consumes the buffer, as mutable aliasing is prohibited.
    #[must_use]
    pub const fn buf_ref(self) -> BufRef<'a, A> {
        BufRef::new(self.data)
    }
}

impl<A: Audio> Buffer<A> {
    /// Initializes a new [`Buffer`] from data.
    #[must_use]
    pub const fn from_data(data: Vec<A>) -> Self {
        Self { data }
    }

    /// Initializes a new empty buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self::from_data(Vec::new())
    }

    /// Converts `self` into a `BufRef`.
    #[must_use]
    pub fn buf_ref(&self) -> BufRef<A> {
        BufRef::new(&self.data)
    }

    /// Converts `self` into a `BufMut`.
    #[must_use]
    pub fn buf_mut(&mut self) -> BufMut<A> {
        BufMut::new(&mut self.data)
    }
}

impl<A: Audio> FromIterator<A> for Buffer<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::from_data(FromIterator::from_iter(iter))
    }
}

impl<A: Audio> IntoIterator for Buffer<A> {
    type IntoIter = std::vec::IntoIter<A>;
    type Item = A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, A: Audio> IntoIterator for &'a Buffer<A> {
    type IntoIter = std::slice::Iter<'a, A>;
    type Item = &'a A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, A: Audio> IntoIterator for &'a mut Buffer<A> {
    type IntoIter = std::slice::IterMut<'a, A>;
    type Item = &'a mut A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

/// Boilerplate common to [`OnceBufGen`] and [`LoopBufGen`].
macro_rules! buf_gen_boilerplate {
    () => {
        /// Returns a reference to the underlying buffer.
        #[must_use]
        pub const fn buffer(&self) -> &B {
            &self.buffer
        }

        /// Returns a mutable reference to the underlying buffer.
        pub fn buffer_mut(&mut self) -> &mut B {
            &mut self.buffer
        }

        /// Returns the number of samples in the buffer.
        #[must_use]
        pub fn len(&self) -> usize {
            self.buffer.len()
        }

        /// Returns whether the buffer is empty.
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.buffer.is_empty()
        }

        /// Returns the inner slice.
        #[must_use]
        pub fn as_slice(&self) -> &[B::Item] {
            self.buffer.as_slice()
        }

        /// Returns a mutable reference to the inner slice.
        pub fn as_mut_slice(&mut self) -> &mut [B::Item]
        where
            B: BufMutTrait,
        {
            self.buffer.as_mut_slice()
        }
    };
}

/// A generator that reads through an audio buffer, once.
#[derive(Clone, Debug)]
pub struct OnceBufGen<B: BufTrait> {
    /// The inner buffer.
    buffer: B,

    /// The sample being read.
    index: usize,
}

impl<B: BufTrait> OnceBufGen<B> {
    /// Initializes a new [`OnceBufGen`].
    #[must_use]
    pub const fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    buf_gen_boilerplate!();
}

impl<B: BufTrait> Signal for OnceBufGen<B> {
    type Sample = B::Item;

    fn get(&self) -> B::Item {
        self.buffer().get(self.index).unwrap_or_default()
    }
}

impl<B: BufTrait> SignalMut for OnceBufGen<B> {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<B: BufTrait> Base for OnceBufGen<B> {
    impl_base!();
}

impl<B: BufTrait> Stop for OnceBufGen<B> {
    fn stop(&mut self) {
        self.index = self.buffer().len();
    }
}

impl<B: BufTrait> Done for OnceBufGen<B> {
    fn is_done(&self) -> bool {
        self.index >= self.buffer().len()
    }
}

impl<B: BufTrait> Panic for OnceBufGen<B> {
    fn panic(&mut self) {
        self.stop();
    }
}

/// A generator that loops an audio buffer.
#[derive(Clone, Debug)]
pub struct LoopBufGen<B: BufTrait> {
    /// The inner buffer.
    buffer: B,

    /// The sample being read.
    index: usize,
}

impl<B: BufTrait> LoopBufGen<B> {
    /// Initializes a new [`LoopBufGen`].
    #[must_use]
    pub const fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    buf_gen_boilerplate!();
}

impl<B: BufTrait> Signal for LoopBufGen<B> {
    type Sample = B::Item;

    fn get(&self) -> B::Item {
        self.buffer()[self.index]
    }
}

impl<B: BufTrait> SignalMut for LoopBufGen<B> {
    fn advance(&mut self) {
        crate::mod_inc(self.len(), &mut self.index);
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<B: BufTrait> Base for LoopBufGen<B> {
    impl_base!();
}
