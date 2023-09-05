use crate::prelude::*;
use crate::units::FracInt;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Div, DivAssign, Mul, MulAssign},
};

/// A time, measured in **samples**.
///
/// Since we use a backing [`FracInt`], multiplication and division by an integer is allowed.
///
/// Note that in order to convert between a [`RawTime`] in seconds and this type, you must know the
/// [`SampleRate`].
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    PartialOrd,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Rem,
    derive_more::RemAssign,
    derive_more::Sum,
)]
pub struct Time {
    /// The number of samples.
    ///
    /// This is stored as our custom type [`FracInt`] and not as a float, as this allows us to
    /// entirely get rid of cumulative error when incrementing a time by a sample.
    pub samples: FracInt,
}

impl Time {
    /// No time.
    pub const ZERO: Self = Self::new(FracInt::ZERO);
    /// One sample.
    pub const SAMPLE: Self = Self::new(FracInt::ONE);

    /// Initializes a time in **samples**.
    ///
    /// If you want to use the more natural unit of seconds, see [`Self::from_raw`].
    #[must_use]
    pub const fn new(samples: FracInt) -> Self {
        Self { samples }
    }

    /// Converts [`RawTime`] into [`Time`], using the specified sample rate.
    #[must_use]
    pub fn from_raw(raw: RawTime, sample_rate: SampleRate) -> Self {
        raw * sample_rate
    }

    /// Converts [`RawTime`] into [`Time`], using the default sample rate.
    #[must_use]
    pub fn from_raw_default(raw: RawTime) -> Self {
        Self::from_raw(raw, SampleRate::default())
    }

    /// Initializes a [`Time`] from the value in seconds, and a sample rate.
    #[must_use]
    pub fn from_sec(seconds: f64, sample_rate: SampleRate) -> Self {
        Self::from_raw(RawTime::new(seconds), sample_rate)
    }

    /// Initializes a [`Time`] from the value in seconds, using the default sample rate.
    #[must_use]
    pub fn from_sec_default(seconds: f64) -> Self {
        Self::from_sec(seconds, SampleRate::default())
    }

    /// Converts [`Time`] into [`RawTime`], using the specified sample rate.
    #[must_use]
    pub fn into_raw(self, sample_rate: SampleRate) -> RawTime {
        self / sample_rate
    }

    /// Converts [`Time`] into [`RawTime`], using the default sample rate.
    #[must_use]
    pub fn into_raw_default(self) -> RawTime {
        self.into_raw(SampleRate::default())
    }

    /// Advances the time by one sample.
    pub fn advance(&mut self) {
        *self += Self::SAMPLE;
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} samples", self.samples)
    }
}

/// Implements the [`Mul`] and [`Div`] traits.
macro_rules! impl_mul_div {
    ($($ty: ty),*) => {$(
        impl Mul<Time> for $ty {
            type Output = Time;

            fn mul(self, rhs: Time) -> Time {
                Time::new(self * rhs.samples)
            }
        }

        impl Mul<$ty> for Time {
            type Output = Self;

            fn mul(self, rhs: $ty) -> Self {
                rhs * self
            }
        }

        impl MulAssign<$ty> for Time {
            fn mul_assign(&mut self, rhs: $ty) {
                *self = *self * rhs
            }
        }

        impl Div<$ty> for Time {
            type Output = Self;

            fn div(self, rhs: $ty) -> Self {
                Self::new(self.samples / rhs)
            }
        }

        impl DivAssign<$ty> for Time {
            fn div_assign(&mut self, rhs: $ty) {
                *self = *self / rhs
            }
        }
    )*};
}

impl_mul_div!(u8, u16, u32, u64, f64);

impl Div<Time> for Time {
    type Output = f64;

    fn div(self, rhs: Time) -> f64 {
        self.samples / rhs.samples
    }
}