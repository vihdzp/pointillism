//! Defines the [`Sample`] trait, and implements it for types [`Mono`], [`Stereo`], and [`Env`].
//!
//! [`Mono`] and [`Stereo`] are [`Audio`] samples, meaning that they can be written to a WAV file in
//! order to produce sound. [`Env`] is reserved for outputs from envelopes, such as an
//! [`Adsr`](crate::effects::adsr::Adsr).

use std::{
    fmt::Debug,
    iter::Sum,
    ops::{
        Add, AddAssign, Div, DivAssign, FnMut, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
    },
};

/// A sample of mono audio, typically holding a value between `-1.0` and `1.0`.
///
/// This is distinguished from [`Env`], as they have different uses. There shouldn't be much reason
/// to convert one to the other.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Mono(pub f64);

/// A sample of stereo audio, typically holding values between `-1.0` and `1.0`.
///
/// The left channel is stored in `.0`, the right channel is stored in `.1`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Stereo(pub f64, pub f64);

/// A data sample from an envelope, typically holding a value between `-1.0` and `1.0`.
///
/// This is distinguished from [`Mono`], as they have different uses. There shouldn't be much reason
/// to convert one to the other.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Env(pub f64);

/// A trait for things that one can perform arithmetic on, like a sample. This includes anything
/// implementing [`Sample`] as well as `f64`.
///
/// This trait exists mostly for convenient, general implementations of methods such as
/// [`linear_inter`](crate::curves::interpolate::linear), which make sense both for samples and for
/// floating point values.
pub trait SampleLike:
    Copy
    + Default
    + Debug
    + Add<Output = Self>
    + AddAssign
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + Mul<f64, Output = Self>
    + MulAssign<f64>
    + Div<f64, Output = Self>
    + DivAssign<f64>
    + Sum
{
    /// The zero value.
    const ZERO: Self;
}

impl SampleLike for f64 {
    const ZERO: Self = 0.0;
}

/// A trait for array-like types that store a compile-time amount of data contiguously.
///
/// This trait serves two purposes:
///
/// - Provide simple convenience functions for the [`Sample`] types.
/// - Allow functions on [`Samples`](Sample) to return an array type of the corresponding size,
///   which can be generically manipulated.
///
/// ## Safety
///
/// Implementors of the trait must guarantee that the type has the same size and alignment as
/// `[Self::Item; Self::SIZE]`.
pub unsafe trait ArrayLike:
    AsRef<[Self::Item]>
    + AsMut<[Self::Item]>
    + Index<usize, Output = Self::Item>
    + IndexMut<usize>
    + Sized
{
    /// The type of items this array stores.
    type Item;

    /// The size of the array.
    const SIZE: usize;

    /// The array type with the same number of elements as `SIZE`.
    ///
    /// If we could use `[T; Self::SIZE]`, this wouldn't be needed.
    type Array<T>: ArrayLike<Item = T>;

    /// Creates a value from an array.
    fn from_array(array: Self::Array<Self::Item>) -> Self;

    /// Turns this value into an array.
    fn into_array(self) -> Self::Array<Self::Item>;

    /// Creates the array `[f(0), f(1), ...]`.
    fn from_fn<F: FnMut(usize) -> Self::Item>(f: F) -> Self;

    /// Initializes a new array with default values.
    #[must_use]
    fn new_default() -> Self
    where
        Self::Item: Default,
    {
        Self::from_fn(|_| Default::default())
    }

    /// Gets the value from channel `index`.
    fn get(&self, index: usize) -> Option<&Self::Item> {
        if index < Self::SIZE {
            Some(&self.as_ref()[index])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the value from channel `index`.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the number of channels.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        if index < Self::SIZE {
            Some(&mut self.as_mut()[index])
        } else {
            None
        }
    }

    /// Executes a function for each element in the array type.
    fn for_each<F: FnMut(usize)>(mut f: F) {
        for i in 0..Self::SIZE {
            f(i);
        }
    }

    /// Applies a function `f` to all entries of the sample.
    #[must_use]
    fn map<F: FnMut(&Self::Item) -> Self::Item>(&self, mut f: F) -> Self {
        Self::from_fn(|index| f(&self[index]))
    }

    /// Applies a function `f` to all entries of the sample.
    #[must_use]
    fn map_array<T: ArrayLike, F: FnMut(&Self::Item) -> T::Item>(&self, mut f: F) -> T {
        T::from_fn(|index| f(&self[index]))
    }

    /// Mutably applies a function `f` to all entries of the sample.
    fn map_mut<F: FnMut(&mut Self::Item)>(&mut self, mut f: F) {
        Self::for_each(|index| f(&mut self[index]));
    }

    /// Applies a function `f` to pairs of entries of the samples.
    #[must_use]
    fn pairwise<F: FnMut(&Self::Item, &Self::Item) -> Self::Item>(
        &self,
        rhs: Self,
        mut f: F,
    ) -> Self {
        Self::from_fn(|index| f(&self[index], &rhs[index]))
    }

    /// Mutably applies a function `f` to pairs of entries of the samples.
    fn pairwise_mut<F: FnMut(&mut Self::Item, &Self::Item)>(&mut self, rhs: Self, mut f: F) {
        Self::for_each(|index| f(&mut self[index], &rhs[index]));
    }

    /// Initializes an array filled with the specified value.
    ///
    /// This could easily be made to take in a `Clone` value instead, if the need arose.
    #[must_use]
    fn from_val(val: Self::Item) -> Self
    where
        Self::Item: Copy,
    {
        Self::from_fn(|_| val)
    }

    /// A default implementation for the [`AsRef`] trait.
    fn _as_ref(&self) -> &[Self::Item] {
        // Safety: this works due to the safety guarantees on the trait.
        unsafe { std::slice::from_raw_parts((self as *const Self).cast(), Self::SIZE) }
    }

    /// A default implementation for the [`AsMut`] trait.
    fn _as_mut(&mut self) -> &mut [Self::Item] {
        // Safety: this works due to the safety guarantees on the trait.
        unsafe { std::slice::from_raw_parts_mut((self as *mut Self).cast(), Self::SIZE) }
    }
}

/// A trait for either [`Mono`], [`Stereo`], or [`Env`] samples.
///
/// [`Mono`] and [`Stereo`] samples may be used for audio, while [`Env`] samples can be used for
/// envelopes such as in an LFO.
pub trait Sample: SampleLike + ArrayLike<Item = f64> {
    /// The size as a `u8`.
    #[must_use]
    fn size_u8() -> u8 {
        // The size is either 1 or 2.
        #[allow(clippy::cast_possible_truncation)]
        {
            Self::SIZE as u8
        }
    }

    /// Gets the value from the first channel.
    fn fst(&self) -> f64 {
        self[0]
    }

    /// Gets a mutable reference to the value from the first channel.
    fn fst_mut(&mut self) -> &mut f64 {
        &mut self[0]
    }

    /// Gets the value from the second channel, defaulting to the first.
    fn snd(&self) -> f64 {
        if Self::SIZE >= 2 {
            self[1]
        } else {
            self[0]
        }
    }

    /// Gets a mutable reference to the value from the second channel, defaulting to the first.
    fn snd_mut(&mut self) -> &mut f64 {
        if Self::SIZE >= 2 {
            &mut self[1]
        } else {
            &mut self[0]
        }
    }

    /// Generates a random sample from a given `Rng` object.
    ///
    /// We use this, instead of `rng.gen()`, in order to avoid having to write down `where Standard:
    /// Distribution<S>` everywhere. However, all three instances of [`Sample`] implement this trait
    /// individually.
    #[must_use]
    fn rand_with<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        Self::from_fn(|_| crate::sgn(rng.gen::<f64>()))
    }

    /// Generates a random sample.
    ///
    /// We use this, instead of `thread_rng().gen()`, in order to avoid having to write down `where
    /// Standard: Distribution<S>` everywhere. However, all three instances of [`Sample`] implement
    /// this individually.
    #[must_use]
    fn rand() -> Self {
        Self::rand_with(&mut rand::thread_rng())
    }

    /// A default implementation of the [`Sum`] trait.
    fn _sum<I: IntoIterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::ZERO;
        for sample in iter {
            res += sample;
        }
        res
    }
}

/// A [`Sample`] specifically for audio, meaning [`Mono`] or [`Stereo`].
pub trait Audio: Sample {
    /// Duplicates a mono signal to convert it into stereo. Leaves a stereo signal unchanged.
    fn duplicate(&self) -> Stereo {
        Stereo(self.fst(), self.snd())
    }

    /// Writes the sample to a WAV file.
    ///
    /// ## Errors
    ///
    /// This should only return an error in case of an IO error.
    #[cfg(feature = "hound")]
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut hound::WavWriter<W>,
    ) -> hound::Result<()> {
        for index in 0..Self::SIZE {
            // In practice, truncation should never occur.
            #[allow(clippy::cast_possible_truncation)]
            writer.write_sample(self[index] as f32)?;
        }

        Ok(())
    }
}

impl SampleLike for Mono {
    const ZERO: Self = Self(0.0);
}

/// Safety: The type is tagged as `#[repr(C)]`.
unsafe impl ArrayLike for Mono {
    const SIZE: usize = 1;

    type Item = f64;
    type Array<T> = [T; 1];

    fn from_array(array: [f64; 1]) -> Self {
        Self(array[0])
    }

    fn into_array(self) -> [f64; 1] {
        [self.0]
    }

    fn from_fn<F: FnMut(usize) -> Self::Item>(mut f: F) -> Self {
        Self(f(0))
    }
}

impl Sample for Mono {}

impl Audio for Mono {}

impl SampleLike for Stereo {
    const ZERO: Self = Self(0.0, 0.0);
}

/// Safety: The type is tagged as `#[repr(C)]`.
unsafe impl ArrayLike for Stereo {
    const SIZE: usize = 2;

    type Item = f64;
    type Array<T> = [T; 2];

    fn from_array(array: [f64; 2]) -> Self {
        Self(array[0], array[1])
    }

    fn into_array(self) -> [f64; 2] {
        [self.0, self.1]
    }

    fn from_fn<F: FnMut(usize) -> Self::Item>(mut f: F) -> Self {
        Self(f(0), f(1))
    }
}

impl Sample for Stereo {}

impl Audio for Stereo {}

impl SampleLike for Env {
    const ZERO: Self = Self(0.0);
}

/// Safety: The type is tagged as `#[repr(C)]`.
unsafe impl ArrayLike for Env {
    const SIZE: usize = 2;

    type Item = f64;

    type Array<T> = [T; 1];

    fn from_array(array: [f64; 1]) -> Self {
        Self(array[0])
    }

    fn into_array(self) -> [f64; 1] {
        [self.0]
    }

    fn from_fn<F: FnMut(usize) -> Self::Item>(mut f: F) -> Self {
        Self(f(0))
    }
}

impl Sample for Env {}

/// Safety: `[T; N]` has the same layout as itself.
unsafe impl<T, const N: usize> ArrayLike for [T; N] {
    const SIZE: usize = N;

    type Item = T;

    type Array<U> = [U; N];

    fn from_fn<F: FnMut(usize) -> Self::Item>(f: F) -> Self {
        std::array::from_fn(f)
    }

    fn from_array(array: Self) -> Self {
        array
    }

    fn into_array(self) -> Self {
        self
    }
}

/// Implements traits [`Add`], [`Sub`].
macro_rules! impl_op {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op for $ty {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                self.pairwise(rhs, |x, y| std::ops::$op::$fn(x, y))
            }
        })*
    };
}

/// Implements traits [`AddAssign`], [`SubAssign`].
macro_rules! impl_op_assign {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op for $ty {
            fn $fn(&mut self, rhs: Self) {
                self.pairwise_mut(rhs, |x, y| std::ops::$op::$fn(x, y));
            }
        })*
    };
}

/// Implements traits [`Mul`] and [`Div`].
macro_rules! impl_op_f64 {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op<f64> for $ty {
            type Output = Self;
            fn $fn(self, rhs: f64) -> Self {
                self.map(|x| std::ops::$op::$fn(x, rhs))
            }
        })*
    };
}

/// Implements [`Mul`] on the other side.
macro_rules! impl_mul_left {
    ($ty: ty) => {
        impl Mul<$ty> for f64 {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> $ty {
                rhs * self
            }
        }
    };
}

/// Implements traits [`MulAssign<f64>`](MulAssign), [`DivAssign<f64>`](DivAssign).
macro_rules! impl_op_assign_f64 {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op<f64> for $ty {
            fn $fn(&mut self, rhs: f64) {
                self.map_mut(|x| std::ops::$op::$fn(x, rhs))
            }
        })*
    };
}

/// Implements trait [`Neg`].
macro_rules! impl_neg {
    ($ty: ty) => {
        impl Neg for $ty {
            type Output = Self;
            fn neg(self) -> Self {
                self.map(|x| -x)
            }
        }
    };
}

/// Implements trait [`Sum`].
macro_rules! impl_sum {
    ($ty: ty) => {
        impl Sum for $ty {
            fn sum<I: IntoIterator<Item = Self>>(iter: I) -> Self {
                Self::_sum(iter)
            }
        }
    };
}

/// Implements trait [`Index`].
macro_rules! impl_index {
    ($ty: ty) => {
        impl Index<usize> for $ty {
            type Output = f64;

            fn index(&self, index: usize) -> &f64 {
                self.get(index).unwrap()
            }
        }

        impl IndexMut<usize> for $ty {
            fn index_mut(&mut self, index: usize) -> &mut f64 {
                self.get_mut(index).unwrap()
            }
        }
    };
}

/// Implements traits [`AsRef<\[f64\]>`](AsRef) and [`AsMut<\[f64\]>`](AsMut).
macro_rules! impl_as {
    ($ty: ty) => {
        impl AsRef<[f64]> for $ty {
            fn as_ref(&self) -> &[f64] {
                self._as_ref()
            }
        }

        impl AsMut<[f64]> for $ty {
            fn as_mut(&mut self) -> &mut [f64] {
                self._as_mut()
            }
        }
    };
}

/// Implements `Distribution<Self>` for `Standard`.
macro_rules! impl_rand {
    ($ty: ty) => {
        impl rand::prelude::Distribution<$ty> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty {
                <$ty>::rand_with(rng)
            }
        }
    };
}

/// Implements all traits for the specified type.
macro_rules! impl_all {
    ($($ty: ty),*) => {
        $(
            impl_op!($ty; Add, add, Sub, sub);
            impl_op_assign!($ty; AddAssign, add_assign, SubAssign, sub_assign);
            impl_op_f64!($ty; Mul, mul, Div, div);
            impl_op_assign_f64!($ty; MulAssign, mul_assign, DivAssign, div_assign);
            impl_mul_left!($ty);
            impl_neg!($ty);
            impl_sum!($ty);
            impl_index!($ty);
            impl_rand!($ty);
            impl_as!($ty);
        )*
    };
}

impl_all!(Mono, Stereo, Env);

/// A numeric type that can store a raw sample from a WAV file.
///
/// The list of types that implement this trait is: `i8`, `i16`, `i32`, `f32`.
#[cfg(feature = "hound")]
pub trait WavSample: hound::Sample {
    /// Re-scales and converts the sample into `Mono`.
    fn into_mono(self) -> Mono;
}

/// A numeric type that can store a raw sample from a WAV file.
///
/// The list of types that implement this trait is: `i8`, `i16`, `i32`, `f32`.
#[cfg(not(feature = "hound"))]
pub trait WavSample {
    /// Re-scales and converts the sample into `Mono`.
    fn into_mono(self) -> Mono;
}

/// Implements `WavSample` for the signed types.
macro_rules! impl_wav_signed {
    ($($ty: ty),*) => {
        $(
            impl WavSample for $ty {
                fn into_mono(self) -> Mono {
                    Mono(self as f64 / <$ty>::MAX as f64)
                }
            }
        )*
    };
}

impl_wav_signed!(i8, i16, i32);

impl WavSample for f32 {
    fn into_mono(self) -> Mono {
        Mono(f64::from(self))
    }
}

#[cfg(test)]

mod test {
    use super::*;
    use std::mem::{align_of, size_of};

    /// Tests that all the [`Sample`] types have the expected size and alignment.
    #[test]
    fn size_align() {
        assert_eq!(size_of::<Mono>(), 8);
        assert_eq!(align_of::<Mono>(), 8);
        assert_eq!(size_of::<Stereo>(), 16);
        assert_eq!(align_of::<Stereo>(), 8);
        assert_eq!(size_of::<Env>(), 8);
        assert_eq!(align_of::<Env>(), 8);
    }

    /// Tests that we can transmute an array of [`Mono`] into an array of [`Stereo`]. This is needed
    /// for reading a stereo [`Buffer`](crate::curves::buffer::Buffer).
    #[test]
    fn transmute_test() {
        let stereo: [Stereo; 2] =
            unsafe { std::mem::transmute([Mono(1.0), Mono(2.0), Mono(3.0), Mono(4.0)]) };
        assert_eq!(stereo, [Stereo(1.0, 2.0), Stereo(3.0, 4.0)]);
    }
}
