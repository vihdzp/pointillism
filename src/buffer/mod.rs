//! Defines different types for audio buffers.
//!
//! You can load a buffer from a WAV file (if the `hound` feature is enabled), or you can create
//! your own buffer and write a signal into it, to then read it back. This can be useful if you want
//! to loop an expensive to compute signal, for instance. This is also used in [`Delay`].
//!
//! You can also use a buffer if you want to process large amounts of audio data before playing it
//! back. This is useful for certain algorithms, such as the
//! [FFT](https://en.wikipedia.org/wiki/Fast_Fourier_transform). Convenience methods such as
//! [`buf::Mut::overwrite`] are provided for loading a buffer.
//!
//! We distinguish three different kinds of buffers: those that hold a reference to its data, those
//! that hold a mutable reference to its data, and those that own its data.

use std::ops::{Index, IndexMut};

use crate::prelude::*;

#[cfg(feature = "hound")]
pub mod wav;

/// A trait for readable buffers.
pub trait Ref: AsRef<[Self::Item]> + std::ops::Index<usize, Output = Self::Item> {
    /// The type of sample stored in the buffer.
    type Item: smp::Audio;

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

    /// Returns the sample corresponding to peak amplitude on all channels.
    #[must_use]
    fn peak(&self) -> <Self::Item as smp::Array>::Array<unt::Vol> {
        /// Prevent code duplication.
        fn peak<A: smp::Audio>(buf: &[A]) -> <A as smp::Array>::Array<unt::Vol> {
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
        fn rms<A: smp::Audio>(buf: &[A]) -> <A as smp::Array>::Array<unt::Vol> {
            use smp::Array;
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
pub trait Mut: Ref + AsMut<[Self::Item]> + std::ops::IndexMut<usize, Output = Self::Item> {
    /// Returns a mutable reference to the inner slice.
    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut()
    }

    /// Gets a mutable reference to a sample at a given index.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        self.as_mut().get_mut(index)
    }

    /// Overwrites a buffer with the output from a song.
    ///
    /// The passed `time` parameter is used for the function.
    fn overwrite_time<F: FnMut(unt::Time) -> Self::Item>(
        &mut self,
        time: &mut unt::Time,
        mut song: F,
    ) {
        for sample in self.as_mut() {
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
        self.overwrite(|_| smp::Base::ZERO);
    }
}

/// A buffer that holds a reference to its data.
#[derive(Clone, Debug)]
pub struct BufRef<'a, A: smp::Audio> {
    pub data: &'a [A],
}

/// A buffer that holds a mutable reference to its data.
#[derive(Debug)]
pub struct BufMut<'a, A: smp::Audio> {
    pub data: &'a mut [A],
}

/// A sample buffer that owns its data.
#[derive(Clone, Debug, Default)]
pub struct Buffer<A: smp::Audio> {
    /// The data stored by the buffer.
    data: Vec<A>,
}

impl<'a, A: smp::Audio> AsRef<[A]> for BufRef<'a, A> {
    fn as_ref(&self) -> &[A] {
        self.data
    }
}

impl<'a, A: smp::Audio> AsRef<[A]> for BufMut<'a, A> {
    fn as_ref(&self) -> &[A] {
        self.data
    }
}

impl<A: smp::Audio> AsRef<[A]> for Buffer<A> {
    fn as_ref(&self) -> &[A] {
        &self.data
    }
}

impl<'a, A: smp::Audio> AsMut<[A]> for BufMut<'a, A> {
    fn as_mut(&mut self) -> &mut [A] {
        self.data
    }
}

impl<A: smp::Audio> AsMut<[A]> for Buffer<A> {
    fn as_mut(&mut self) -> &mut [A] {
        &mut self.data
    }
}

impl<'a, A: smp::Audio> Index<usize> for BufRef<'a, A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<'a, A: smp::Audio> Index<usize> for BufMut<'a, A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<A: smp::Audio> Index<usize> for Buffer<A> {
    type Output = A;

    fn index(&self, index: usize) -> &A {
        &self.as_ref()[index]
    }
}

impl<'a, A: smp::Audio> IndexMut<usize> for BufMut<'a, A> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut()[index]
    }
}

impl<A: smp::Audio> IndexMut<usize> for Buffer<A> {
    fn index_mut(&mut self, index: usize) -> &mut A {
        &mut self.as_mut()[index]
    }
}

impl<'a, A: smp::Audio> Ref for BufRef<'a, A> {
    type Item = A;
}

impl<'a, A: smp::Audio> Ref for BufMut<'a, A> {
    type Item = A;
}

impl<A: smp::Audio> Ref for Buffer<A> {
    type Item = A;
}

impl<'a, A: smp::Audio> Mut for BufMut<'a, A> {}
impl<A: smp::Audio> Mut for Buffer<A> {}

impl<'a, A: smp::Audio> BufRef<'a, A> {
    /// Initializes a new [`BufRef`].
    pub const fn new(data: &'a [A]) -> Self {
        Self { data }
    }
}

impl<'a, A: smp::Audio> BufMut<'a, A> {
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

impl<A: smp::Audio> Buffer<A> {
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

    /// Initializes an empty buffer with a given size. All samples are initialized to zero.
    pub fn empty(samples: usize) -> Self {
        Self::from_data(vec![A::ZERO; samples])
    }

    /// Initializes an empty buffer with a given length, rounded down to the nearest sample. All
    /// samples are initialized to zero.
    ///
    /// ## Panics
    ///
    /// On a 32-bit machine, panics if the buffer is too large.
    #[must_use]
    pub fn empty_time(time: unt::Time) -> Self {
        Self::empty(time.samples.int().try_into().expect("buffer too large"))
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

impl<A: smp::Audio> From<Vec<A>> for Buffer<A> {
    fn from(data: Vec<A>) -> Self {
        Self::from_data(data)
    }
}

impl<A: smp::Audio> FromIterator<A> for Buffer<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::from_data(FromIterator::from_iter(iter))
    }
}

impl<A: smp::Audio> IntoIterator for Buffer<A> {
    type IntoIter = std::vec::IntoIter<A>;
    type Item = A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, A: smp::Audio> IntoIterator for &'a Buffer<A> {
    type IntoIter = std::slice::Iter<'a, A>;
    type Item = &'a A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, A: smp::Audio> IntoIterator for &'a mut Buffer<A> {
    type IntoIter = std::slice::IterMut<'a, A>;
    type Item = &'a mut A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}
