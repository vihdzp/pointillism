//! Defines the [`Sample`] trait, and implements it for types [`Mono`],
//! [`Stereo`], and [`Env`].
//!
//! [`Mono`] and [`Stereo`] are [`Audio`] samples, meaning that they can be
//! written to a WAV file in order to produce sound. [`Env`] is reserved for
//! outputs from envelopes, such as an [`Adsr`](crate::effects::adsr::Adsr).

use hound::WavWriter;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, FnMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// A sample of mono audio, typically holding a value between `-1.0` and `1.0`.
///
/// This is distinguished from [`Env`] as they have different uses, but one may
/// freely convert one to the other if so needed.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Mono(pub f64);

/// A sample of stereo audio, typically holding values between `-1.0` and `1.0`.
///
/// The left channel is stored in `.0`, the right channel is stored in `.1`.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Stereo(pub f64, pub f64);

/// A data sample from an envelope, typically holding a value between `-1.0` and
/// `1.0`.
///
/// This is distinguished from [`Mono`] as they have different uses, but one may
/// freely convert one to the other if so needed.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Env(pub f64);

/// A trait for things that one can perform arithmetic on, like a sample.
/// This includes anything implementing [`Sample`] as well as `f64`.
///
/// This trait exists mostly for convenient, general implementations of methods
/// such as [`linear_inter`](crate::curves::buffer::linear_inter), which make
/// sense both for samples and for floating point values.
#[allow(clippy::module_name_repetitions)]
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
/// [`Mono`] and [`Stereo`] samples may be used for audio, while [`Env`] samples
/// can be used for envelopes such as in an LFO.
pub trait Sample: SampleLike {
    /// The number of values stored in the sample.
    const CHANNELS: u8;

    /// Gets the value from channel `idx`. Reads the last channel if out of
    /// bounds.
    fn get_unchecked(&self, idx: u8) -> f64;

    /// Gets a reference to the value from channel `idx`. Reads the last channel
    /// if out of bounds.
    fn get_mut_unchecked(&mut self, idx: u8) -> &mut f64;

    /// Gets the value from channel `idx`.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the number of channels.
    fn get(&self, idx: u8) -> f64 {
        assert!(idx < Self::CHANNELS, "index {idx} out of bounds");
        self.get_unchecked(idx)
    }

    /// Gets a reference to the value from channel `idx`.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the number of channels.
    fn get_mut(&mut self, idx: u8) -> &mut f64 {
        assert!(idx < Self::CHANNELS, "index {idx} out of bounds");
        self.get_mut_unchecked(idx)
    }

    /// Executes a function for each channel index.
    fn for_each<F: FnMut(u8)>(mut f: F) {
        for idx in 0..Self::CHANNELS {
            f(idx);
        }
    }

    /// Initializes a new sample by calling `f(idx)` on each index.
    fn from_fn<F: FnMut(u8) -> f64>(mut f: F) -> Self {
        let mut res = Self::default();
        Self::for_each(|idx| *res.get_mut_unchecked(idx) = f(idx));
        res
    }

    /// Applies a function `f` to all entries of the sample.
    #[must_use]
    fn map<F: FnMut(f64) -> f64>(&self, mut f: F) -> Self {
        Self::from_fn(|idx| f(self.get(idx)))
    }

    /// Mutably applies a function `f` to all entries of the sample.
    fn map_mut<F: FnMut(&mut f64)>(&mut self, mut f: F) {
        Self::for_each(|idx| f(self.get_mut_unchecked(idx)));
    }

    /// Applies a function `f` to pairs of entries of the samples.
    #[must_use]
    fn pairwise<F: FnMut(f64, f64) -> f64>(&self, rhs: Self, mut f: F) -> Self {
        Self::from_fn(|idx| f(self.get(idx), rhs.get(idx)))
    }

    /// Mutably applies a function `f` to pairs of entries of the samples.
    fn pairwise_mut<F: FnMut(&mut f64, f64)>(&mut self, rhs: Self, mut f: F) {
        Self::for_each(|idx| f(self.get_mut(idx), rhs.get(idx)));
    }

    /// Initializes a sample where all channels use the specified value.
    #[must_use]
    fn from_val(val: f64) -> Self {
        Self::from_fn(|_| val)
    }

    /// Generates a random sample from a given `Rng` object.
    ///
    /// We use this, instead of `rng.gen()`, in order to avoid having to write
    /// down `where Standard: Distribution<S>` everywhere. However, all three
    /// instances of [`Sample`] implement this trait individually.
    #[must_use]
    fn rand_with<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::from_fn(|_| crate::sgn(rng.gen::<f64>()))
    }

    /// Generates a random sample.
    ///
    /// We use this, instead of `thread_rng().gen()`, in order to avoid having
    /// to write down `where Standard: Distribution<S>` everywhere. However, all
    /// three instances of [`Sample`] implement this individually.
    #[must_use]
    fn rand() -> Self {
        Self::rand_with(&mut rand::thread_rng())
    }

    /// A default implementation of the [`Sum`] trait.
    fn _sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::ZERO;
        for sample in iter {
            res += sample;
        }
        res
    }
}

/// A [`Sample`] specifically for audio, meaning [`Mono`] or [`Stereo`].
pub trait Audio: Sample {
    /// Duplicates a mono signal to convert it into stereo. Leaves a stereo
    /// signal unchanged.
    fn duplicate(&self) -> Stereo {
        Stereo(self.get_unchecked(0), self.get_unchecked(1))
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
        for idx in 0..Self::CHANNELS {
            // In practice, truncation should never occur.
            #[allow(clippy::cast_possible_truncation)]
            writer.write_sample(self.get_unchecked(idx) as f32)?;
        }

        Ok(())
    }
}

impl SampleLike for Mono {
    const ZERO: Self = Self(0.0);
}

impl Sample for Mono {
    const CHANNELS: u8 = 1;

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

    fn get_unchecked(&self, idx: u8) -> f64 {
        if idx == 0 {
            self.0
        } else {
            self.1
        }
    }

    fn get_mut_unchecked(&mut self, idx: u8) -> &mut f64 {
        if idx == 0 {
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
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
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
/// The list of types that implement this trait is: `i8`, `i16`, `i32`, `f64`.
#[allow(clippy::module_name_repetitions)]
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

    #[test]
    fn size_align() {
        assert_eq!(size_of::<Mono>(), 8);
        assert_eq!(align_of::<Mono>(), 8);
        assert_eq!(size_of::<Stereo>(), 16);
        assert_eq!(align_of::<Stereo>(), 8);
        assert_eq!(size_of::<Env>(), 8);
        assert_eq!(align_of::<Env>(), 8);
    }
}
