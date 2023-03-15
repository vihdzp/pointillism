//! Defines the [`Sample`] trait, and implements it for types [`Mono`],
//! [`Stereo`], and [`Env`].

use hound::WavWriter;
use rand::Rng;
use std::{fmt::Debug, iter::Sum, ops::*};

/// A sample of mono audio, typically holding a value between `-1.0` and `1.0`.
///
/// This is distinguished from [`Env`] as they have different uses, but one may
/// freely convert one to the other if so needed.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mono(pub f64);

/// A sample of stereo audio, typically holding values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Stereo(pub f64, pub f64);

/// A data sample from an envelope, typically holding a value between `-1.0` and
/// `1.0`.
///
/// This is distinguished from [`Mono`] as they have different uses, but one may
/// freely convert one to the other if so needed.
#[derive(Clone, Copy, Debug, Default)]
pub struct Env(pub f64);

impl From<Env> for Mono {
    fn from(value: Env) -> Self {
        Self(value.0)
    }
}

impl From<Mono> for Env {
    fn from(value: Mono) -> Self {
        Self(value.0)
    }
}

/// A trait for either [`Mono`], [`Stereo`], or [`Env`] samples.
///
/// [`Mono`] and [`Stereo`] samples may be used for audio, while [`Env`] samples
/// can be used for envelopes such as in an LFO.
pub trait Sample:
    Copy
    + Default
    + Debug
    + Add
    + AddAssign
    + Neg
    + Sub
    + SubAssign
    + Mul<f64, Output = Self>
    + MulAssign<f64>
    + Div<f64, Output = Self>
    + DivAssign<f64>
    + Sum
{
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
        if idx >= Self::CHANNELS {
            panic!("index {idx} out of bounds");
        }

        self.get_unchecked(idx)
    }

    /// Gets a reference to the value from channel `idx`.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than the number of channels.
    fn get_mut(&mut self, idx: u8) -> &mut f64 {
        if idx >= Self::CHANNELS {
            panic!("index {idx} out of bounds");
        }

        self.get_mut_unchecked(idx)
    }

    /// Applies a function `f` to all entries of the sample.
    fn map<F: FnMut(f64) -> f64>(&self, mut f: F) -> Self {
        let mut res = Self::default();
        for idx in 0..Self::CHANNELS {
            *res.get_mut(idx) = f(self.get(idx));
        }
        res
    }

    /// Mutably applies a function `f` to all entries of the sample.
    fn map_mut<F: FnMut(&mut f64)>(&mut self, mut f: F) {
        for idx in 0..Self::CHANNELS {
            f(self.get_mut(idx));
        }
    }

    /// Applies a function `f` to pairs of entries of the samples.
    fn pairwise<F: FnMut(f64, f64) -> f64>(&self, rhs: Self, mut f: F) -> Self {
        let mut res = Self::default();
        for idx in 0..Self::CHANNELS {
            *res.get_mut(idx) = f(self.get(idx), rhs.get(idx));
        }
        res
    }

    /// Mutably applies a function `f` to pairs of entries of the samples.
    fn pairwise_mut<F: FnMut(&mut f64, f64)>(&mut self, rhs: Self, mut f: F) {
        for idx in 0..Self::CHANNELS {
            f(self.get_mut(idx), rhs.get(idx));
        }
    }

    /// The zero sample.
    fn zero() -> Self {
        Self::default()
    }

    /// Generates a random sample.
    fn rand() -> Self {
        Self::default().map(|_| super::to_sgn(rand::thread_rng().gen::<f64>()))
    }

    fn _sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = Self::zero();

        for sample in iter {
            res += sample;
        }

        res
    }
}

/// A [`Sample`] specifically for audio, meaning [`Mono`] or [`Stereo`].
pub trait AudioSample: Sample {
    /// Duplicates a mono signal to convert it into stereo. Leaves a stereo
    /// signal unchanged.
    fn duplicate(&self) -> Stereo {
        Stereo(self.get_unchecked(0), self.get_unchecked(1))
    }

    /// Writes the sample to a WAV file.
    fn write<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut WavWriter<W>,
        channels: u8,
    ) -> Result<(), hound::Error> {
        if channels == 1 {
            writer.write_sample(self.get(0) as f32)
        } else {
            let sample = self.duplicate();
            writer.write_sample(sample.get(0) as f32)?;
            writer.write_sample(sample.get(1) as f32)
        }
    }
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

impl AudioSample for Mono {}

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

impl AudioSample for Stereo {}

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
        )*
    };
}

impl_all!(Mono, Stereo, Env);
