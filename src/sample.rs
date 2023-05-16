//! Defines the [`Sample`] trait, and implements it for types [`Mono`], [`Stereo`], and [`Env`].
//!
//! [`Mono`] and [`Stereo`] are [`Audio`] samples, meaning that they can be written to a WAV file in
//! order to produce sound. [`Env`] is reserved for outputs from envelopes, such as an
//! [`Adsr`](crate::effects::adsr::Adsr).

use hound::WavWriter;
use rand::{distributions::Standard, prelude::Distribution, Rng};
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

/// A trait for either [`Mono`], [`Stereo`], or [`Env`] samples.
///
/// [`Mono`] and [`Stereo`] samples may be used for audio, while [`Env`] samples can be used for
/// envelopes such as in an LFO.
pub trait Sample: SampleLike {
    /// The number of values stored in the sample.
    const CHANNELS: u8;

    /// The array with the corresponding number of channels.
    type Array<T>: Index<usize, Output = T> + IndexMut<usize, Output = T>;

    /// Initializes a new array with the corresponding number of channels, by calling `f` with each
    /// index.
    fn new_array_with<T, F: FnMut(u8) -> T>(f: F) -> Self::Array<T>;

    /// Initializes the array `[0.0; CHANNELS]`.
    #[must_use]
    fn new_array_f64() -> Self::Array<f64> {
        Self::new_array_with(|_| 0.0)
    }

    /// Gets the value from channel `index`. Reads the last channel if out of
    /// bounds.
    fn get_unchecked(&self, index: u8) -> f64;

    /// Gets a mutable reference to the value from channel `index`. Reads the last
    /// channel if out of bounds.
    fn get_mut_unchecked(&mut self, index: u8) -> &mut f64;

    /// Gets the value from the first channel.
    fn fst(&self) -> f64 {
        self.get_unchecked(0)
    }

    /// Gets a mutable reference to the value from the first channel.
    fn fst_mut(&mut self) -> &mut f64 {
        self.get_mut_unchecked(0)
    }

    /// Gets the value from the second channel, defaulting to the first.
    fn snd(&self) -> f64 {
        self.get_unchecked(1)
    }

    /// Gets a mutable reference to the value from the second channel,
    /// defaulting to the first.
    fn snd_mut(&mut self) -> &mut f64 {
        self.get_mut_unchecked(1)
    }

    /// Gets the value from channel `index`.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the number of channels.
    fn get(&self, index: u8) -> f64 {
        assert!(index < Self::CHANNELS, "index {index} out of bounds");
        self.get_unchecked(index)
    }

    /// Gets a reference to the value from channel `index`.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the number of channels.
    fn get_mut(&mut self, index: u8) -> &mut f64 {
        assert!(index < Self::CHANNELS, "index {index} out of bounds");
        self.get_mut_unchecked(index)
    }

    /// Executes a function for each channel index.
    ///
    /// This unfolds to either 1 or 2 individual function calls.
    fn for_each<F: FnMut(u8)>(f: F);

    /// Initializes a new sample by calling `f(index)` on each index.
    fn from_fn<F: FnMut(u8) -> f64>(mut f: F) -> Self {
        let mut res = Self::default();
        Self::for_each(|index| *res.get_mut_unchecked(index) = f(index));
        res
    }

    /// Applies a function `f` to all entries of the sample.
    #[must_use]
    fn map<F: FnMut(f64) -> f64>(&self, mut f: F) -> Self {
        Self::from_fn(|index| f(self.get(index)))
    }

    /// Mutably applies a function `f` to all entries of the sample.
    fn map_mut<F: FnMut(&mut f64)>(&mut self, mut f: F) {
        Self::for_each(|index| f(self.get_mut_unchecked(index)));
    }

    /// Applies a function `f` to pairs of entries of the samples.
    #[must_use]
    fn pairwise<F: FnMut(f64, f64) -> f64>(&self, rhs: Self, mut f: F) -> Self {
        Self::from_fn(|index| f(self.get(index), rhs.get(index)))
    }

    /// Mutably applies a function `f` to pairs of entries of the samples.
    fn pairwise_mut<F: FnMut(&mut f64, f64)>(&mut self, rhs: Self, mut f: F) {
        Self::for_each(|index| f(self.get_mut(index), rhs.get(index)));
    }

    /// Initializes a sample where all channels use the specified value.
    #[must_use]
    fn from_val(val: f64) -> Self {
        Self::from_fn(|_| val)
    }

    /// Generates a random sample from a given `Rng` object.
    ///
    /// We use this, instead of `rng.gen()`, in order to avoid having to write down `where Standard:
    /// Distribution<S>` everywhere. However, all three instances of [`Sample`] implement this trait
    /// individually.
    #[must_use]
    fn rand_with<R: Rng + ?Sized>(rng: &mut R) -> Self {
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
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut WavWriter<W>,
    ) -> hound::Result<()> {
        for index in 0..Self::CHANNELS {
            // In practice, truncation should never occur.
            #[allow(clippy::cast_possible_truncation)]
            writer.write_sample(self.get_unchecked(index) as f32)?;
        }

        Ok(())
    }
}

impl SampleLike for Mono {
    const ZERO: Self = Self(0.0);
}

impl Sample for Mono {
    const CHANNELS: u8 = 1;

    type Array<T> = [T; 1];

    fn new_array_with<T, F: FnMut(u8) -> T>(mut f: F) -> Self::Array<T> {
        [f(0)]
    }

    fn for_each<F: FnMut(u8)>(mut f: F) {
        f(0);
    }

    fn get_unchecked(&self, _: u8) -> f64 {
        self.0
    }

    fn get_mut_unchecked(&mut self, _: u8) -> &mut f64 {
        &mut self.0
    }
}

impl Audio for Mono {}

impl SampleLike for Stereo {
    const ZERO: Self = Self(0.0, 0.0);
}

impl Sample for Stereo {
    const CHANNELS: u8 = 2;

    type Array<T> = [T; 2];

    fn new_array_with<T, F: FnMut(u8) -> T>(mut f: F) -> Self::Array<T> {
        [f(0), f(1)]
    }

    fn for_each<F: FnMut(u8)>(mut f: F) {
        f(0);
        f(1);
    }

    fn get_unchecked(&self, index: u8) -> f64 {
        if index == 0 {
            self.0
        } else {
            self.1
        }
    }

    fn get_mut_unchecked(&mut self, index: u8) -> &mut f64 {
        if index == 0 {
            &mut self.0
        } else {
            &mut self.1
        }
    }
}

impl Audio for Stereo {}

impl SampleLike for Env {
    const ZERO: Self = Self(0.0);
}

impl Sample for Env {
    const CHANNELS: u8 = 1;

    type Array<T> = [T; 1];

    fn new_array_with<T, F: FnMut(u8) -> T>(mut f: F) -> Self::Array<T> {
        [f(0)]
    }

    fn for_each<F: FnMut(u8)>(mut f: F) {
        f(0);
    }

    fn get_unchecked(&self, _: u8) -> f64 {
        self.0
    }

    fn get_mut_unchecked(&mut self, _: u8) -> &mut f64 {
        &mut self.0
    }
}

/// Implements traits `Add`, `Sub`.
macro_rules! impl_op {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op for $ty {
            type Output = Self;
            fn $fn(self, rhs: Self) -> Self::Output {
                self.pairwise(rhs, std::ops::$op::$fn)
            }
        })*
    };
}

/// Implements traits `AddAssign`, `SubAssign`.
macro_rules! impl_op_assign {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op for $ty {
            fn $fn(&mut self, rhs: Self) {
                self.pairwise_mut(rhs, std::ops::$op::$fn);
            }
        })*
    };
}

/// Implements traits `Mul<f64>`, `Div<f64>`.
macro_rules! impl_op_f64 {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op<f64> for $ty {
            type Output = Self;
            fn $fn(self, rhs: f64) -> Self::Output {
                self.map(|x| std::ops::$op::$fn(x, rhs))
            }
        })*
    };
}

/// Implements traits `MulAssign<f64>`, `DivAssign<f64>`.
macro_rules! impl_op_assign_f64 {
    ($ty: ty; $($op: ident, $fn: ident),*) => {
        $(impl $op<f64> for $ty {
            fn $fn(&mut self, rhs: f64) {
                self.map_mut(|x| std::ops::$op::$fn(x, rhs))
            }
        })*
    };
}

/// Implements trait `Neg`.
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

/// Implements trait `Sum`.
macro_rules! impl_sum {
    ($ty: ty) => {
        impl Sum for $ty {
            fn sum<I: IntoIterator<Item = Self>>(iter: I) -> Self {
                Self::_sum(iter)
            }
        }
    };
}

/// Implements `Distribution<Self>` for `Standard`.
macro_rules! impl_rand {
    ($ty: ty) => {
        impl Distribution<$ty> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $ty {
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
            impl_neg!($ty);
            impl_sum!($ty);
            impl_rand!($ty);
        )*
    };
}

impl_all!(Mono, Stereo, Env);

/// A numeric type that can store a raw sample from a WAV file.
///
/// The list of types that implement this trait is: `i8`, `i16`, `i32`, `f32`.
pub trait WavSample: hound::Sample {
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
