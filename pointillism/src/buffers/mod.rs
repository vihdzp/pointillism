//! Defines different types for audio buffers.
//!
//! You can load a buffer from a WAV file (if the `hound` feature is enabled), or you can create
//! your own buffer and write a signal into it, to then read it back. This can be useful if you want
//! to loop an expensive to compute signal. This is also used in [`eff::dly::Delay`].
//!
//! You can also use a buffer if you want to process large amounts of audio data before playing it
//! back. This is useful for certain algorithms, such as the
//! [FFT](https://en.wikipedia.org/wiki/Fast_Fourier_transform). Convenience methods such as
//! [`BufferMut::overwrite`] are provided for loading a buffer.
//!
//! We distinguish three different kinds of buffers: those that hold a reference to its data, those
//! that hold a mutable reference to its data, and those that own its data.

use crate::prelude::*;
use std::ops::{Index, IndexMut};

pub mod interpolate;
mod ring;
#[cfg(feature = "hound")]
pub mod wav;

pub use interpolate as int;
pub use ring::*;

/// A trait for readable buffers.
pub trait Buffer: AsRef<[Self::Item]> + std::ops::Index<usize, Output = Self::Item> {
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
    fn time(&self) -> unt::Time {
        unt::Time::new(crate::units::FracInt::new(self.len() as u64))
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

    // TODO: move these elsewhere?

    /// Returns the sample corresponding to peak amplitude on all channels.
    #[must_use]
    fn peak(&self) -> <Self::Item as smp::Array>::Array<unt::Vol> {
        /// Prevent code duplication.
        fn peak<A: Audio>(buf: &[A]) -> <A as smp::Array>::Array<unt::Vol> {
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

            res.map_array(|&x| unt::Vol::new(x))
        }

        peak(self.as_ref())
    }

    /// Calculates the RMS on all channels.
    #[must_use]
    fn rms(&self) -> <Self::Item as smp::Array>::Array<unt::Vol> {
        /// Prevent code duplication.
        fn rms<A: Audio>(buf: &[A]) -> <A as smp::Array>::Array<unt::Vol> {
            let mut res: <A as Array>::Array<f64> = Array::new_default();

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

            res.map_array(|&x| unt::Vol::new(x))
        }

        rms(self.as_ref())
    }
}

/// A trait for buffers that hold a mutable reference to its data.
pub trait BufferMut:
    Buffer + AsMut<[Self::Item]> + std::ops::IndexMut<usize, Output = Self::Item>
{
    /// Returns a mutable reference to the inner slice.
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut()
    }

    /// Currently, `rust-analyzer` trips up sometimes that `as_mut` is called directly, not
    /// recognizing that it can be dereferenced. This hack bypasses this.
    fn _as_mut(&mut self) -> &mut [Self::Item] {
        self.as_mut()
    }

    /// Gets a mutable reference to a sample at a given index.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        self._as_mut().get_mut(index)
    }

    /// Overwrites a buffer with the output from a song.
    ///
    /// The passed `time` parameter is used for the function.
    fn overwrite_time<F: FnMut(unt::Time) -> Self::Item>(
        &mut self,
        time: &mut unt::Time,
        mut song: F,
    ) {
        for sample in self._as_mut() {
            *sample = song(*time);
            time.advance();
        }
    }

    /// Overwrites a buffer with the output from a song.
    ///
    /// The timer starts at zero. Use [`Self::overwrite_time`] to specify the time.
    fn overwrite<F: FnMut(unt::Time) -> Self::Item>(&mut self, song: F) {
        let mut time = unt::Time::ZERO;
        self.overwrite_time(&mut time, song);
    }

    /// Overwrites a buffer with the output from a signal. The signal is not consumed.
    fn overwrite_from_sgn<S: SignalMut<Sample = Self::Item>>(&mut self, sgn: &mut S) {
        self.overwrite(|_| sgn.next());
    }

    /// Clears a buffer, without changing its length.
    fn clear(&mut self) {
        self.overwrite(|_| smp::SampleBase::ZERO);
    }
}

/// A buffer that holds a reference to its data.
#[derive(Clone, Debug)]
pub struct Slice<'a, A: Audio> {
    pub data: &'a [A],
}

/// A buffer that holds a mutable reference to its data.
#[derive(Debug)]
pub struct SliceMut<'a, A: Audio> {
    pub data: &'a mut [A],
}

/// A statically allocated, owned sample buffer.
#[derive(Clone, Copy, Debug)]
pub struct Stc<A: Audio, const N: usize> {
    /// The data stored by the buffer.
    pub data: [A; N],
}

/// A dynamically allocated, owned sample buffer.
#[derive(Clone, Debug, Default)]
pub struct Dyn<A: Audio> {
    /// The data stored by the buffer.
    pub data: Vec<A>,
}

impl<'a, A: Audio> AsRef<[A]> for Slice<'a, A> {
    fn as_ref(&self) -> &[A] {
        self.data
    }
}

impl<'a, A: Audio> AsRef<[A]> for SliceMut<'a, A> {
    fn as_ref(&self) -> &[A] {
        self.data
    }
}

impl<A: Audio, const N: usize> AsRef<[A]> for Stc<A, N> {
    fn as_ref(&self) -> &[A] {
        &self.data
    }
}

impl<A: Audio> AsRef<[A]> for Dyn<A> {
    fn as_ref(&self) -> &[A] {
        &self.data
    }
}

impl<'a, A: Audio> AsMut<[A]> for SliceMut<'a, A> {
    fn as_mut(&mut self) -> &mut [A] {
        self.data
    }
}

impl<A: Audio, const N: usize> AsMut<[A]> for Stc<A, N> {
    fn as_mut(&mut self) -> &mut [A] {
        &mut self.data
    }
}

impl<A: Audio> AsMut<[A]> for Dyn<A> {
    fn as_mut(&mut self) -> &mut [A] {
        &mut self.data
    }
}

impl<'a, A: Audio> Index<usize> for Slice<'a, A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<'a, A: Audio> Index<usize> for SliceMut<'a, A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<A: Audio, const N: usize> Index<usize> for Stc<A, N> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<A: Audio> Index<usize> for Dyn<A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<'a, A: Audio> IndexMut<usize> for SliceMut<'a, A> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut()[index]
    }
}

impl<A: Audio, const N: usize> IndexMut<usize> for Stc<A, N> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut()[index]
    }
}

impl<A: Audio> IndexMut<usize> for Dyn<A> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut()[index]
    }
}

impl<'a, A: Audio> Buffer for Slice<'a, A> {
    type Item = A;
}

impl<'a, A: Audio> Buffer for SliceMut<'a, A> {
    type Item = A;
}

impl<A: Audio, const N: usize> Buffer for Stc<A, N> {
    type Item = A;
}

impl<A: Audio> Buffer for Dyn<A> {
    type Item = A;
}

impl<'a, A: Audio> BufferMut for SliceMut<'a, A> {}
impl<A: Audio, const N: usize> BufferMut for Stc<A, N> {}
impl<A: Audio> BufferMut for Dyn<A> {}

impl<A: Audio, const N: usize> Default for Stc<A, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, A: Audio> Slice<'a, A> {
    /// Initializes a new [`buf::Slice`].
    pub const fn new(data: &'a [A]) -> Self {
        Self { data }
    }
}

impl<'a, A: Audio> SliceMut<'a, A> {
    /// Initializes a new [`buf::SliceMut`].
    pub fn new(data: &'a mut [A]) -> Self {
        Self { data }
    }

    /// Converts `self` into a `BufRef`.
    ///
    /// Notes that this consumes the buffer, as mutable aliasing is prohibited.
    #[must_use]
    pub const fn buf_ref(self) -> Slice<'a, A> {
        Slice::new(self.data)
    }
}

impl<A: Audio, const N: usize> Stc<A, N> {
    /// Initializes a new [`Stc`] from data.
    pub const fn from_data(data: [A; N]) -> Self {
        Self { data }
    }

    /// Initializes the zero [`Stc`].
    ///
    /// Use [`Self::from_data`] to initialize this with pre-existing data.
    #[must_use]
    pub const fn new() -> Self {
        Self::from_data([A::ZERO; N])
    }

    /// Converts `self` into a [`Slice`].
    #[must_use]
    pub fn slice(&self) -> Slice<A> {
        Slice::new(&self.data)
    }

    /// Converts `self` into a [`SliceMut`].
    #[must_use]
    pub fn slice_mut(&mut self) -> SliceMut<A> {
        SliceMut::new(&mut self.data)
    }
}

impl<A: Audio> Dyn<A> {
    /// Initializes a new [`Dyn`] from data.
    #[must_use]
    pub const fn from_data(data: Vec<A>) -> Self {
        Self { data }
    }

    /// Initializes an empty buffer with a given size. All samples are initialized to zero.
    ///
    /// Use [`Self::from_data`] to initialize this with pre-existing data.
    pub fn new(samples: usize) -> Self {
        Self::from_data(vec![A::ZERO; samples])
    }

    /// Initializes an empty buffer with a given length, rounded down to the nearest sample. All
    /// samples are initialized to zero.
    ///
    /// ## Panics
    ///
    /// On a 32-bit machine, panics if the buffer is too large.
    #[must_use]
    pub fn new_time(time: unt::Time) -> Self {
        Self::new(time.samples.int().try_into().expect("buffer too large"))
    }

    /// Initializes a buffer with zero initial capacity.
    #[must_use]
    pub const fn empty() -> Self {
        Self::from_data(Vec::new())
    }

    /// Converts `self` into a [`buf::Slice`].
    #[must_use]
    pub fn slice(&self) -> Slice<A> {
        Slice::new(&self.data)
    }

    /// Converts `self` into a [`buf::SliceMut`].
    #[must_use]
    pub fn slice_mut(&mut self) -> SliceMut<A> {
        SliceMut::new(&mut self.data)
    }

    /// Iterates over the slice.
    pub fn iter(&self) -> std::slice::Iter<A> {
        self.into_iter()
    }

    /// Mutably iterates over the slice.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<A> {
        self.into_iter()
    }

    /// Creates a buffer from the output of a song.
    ///
    /// Compare to [`crate::create`].
    ///
    /// ## Panics
    ///
    /// Panics if a buffer of this size can't be created.
    pub fn create<F: FnMut(unt::Time) -> A>(length: unt::Time, mut song: F) -> Self {
        let length = length.samples.int();
        let mut data = Vec::with_capacity(usize::try_from(length).expect("buffer too large"));

        let mut time = unt::Time::ZERO;
        for _ in 0..length {
            data.push(song(time));
            time.advance();
        }

        Self::from_data(data)
    }

    /// Creates a buffer from the output of a signal. The signal is not consumed.
    ///
    /// Compare to [`crate::create_from_sgn`].
    ///
    /// ## Panics
    ///
    /// Panics if a buffer of this size can't be created.
    pub fn create_from_sgn<S: SignalMut<Sample = A>>(length: unt::Time, sgn: &mut S) -> Self {
        Self::create(length, |_| sgn.next())
    }
}

impl<A: Audio> From<Vec<A>> for Dyn<A> {
    fn from(data: Vec<A>) -> Self {
        Self::from_data(data)
    }
}

impl<A: Audio> FromIterator<A> for Dyn<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::from_data(FromIterator::from_iter(iter))
    }
}

impl<A: Audio> IntoIterator for Dyn<A> {
    type IntoIter = std::vec::IntoIter<A>;
    type Item = A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, A: Audio> IntoIterator for &'a Dyn<A> {
    type IntoIter = std::slice::Iter<'a, A>;
    type Item = &'a A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, A: Audio> IntoIterator for &'a mut Dyn<A> {
    type IntoIter = std::slice::IterMut<'a, A>;
    type Item = &'a mut A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

/// An empty buffer.
pub type Empty<A> = Stc<A, 0>;
