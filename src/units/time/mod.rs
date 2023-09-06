//! Implements the types for frequency: [`RawTime`] and [`Time`].

mod raw;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Div, DivAssign, Mul, MulAssign};

pub use raw::RawTime;

use super::{FracInt, SampleRate};

/// A time, measured in **samples**.
///
/// This is stored as our custom type [`FracInt`] and not as a float, as this allows us to entirely
/// get rid of cumulative error when incrementing a time by a sample. In particular, [`Time`] can be
/// multiplied or divided by an unsigned integer.
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
        write!(
            f,
            "{:.*} samples",
            f.precision().unwrap_or_default(),
            self.samples
        )
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

/// A helper struct that can be used to do an event once after a certain time duration.
#[derive(Clone, Copy, Debug)]
pub struct Timer {
    /// Length of the timer.
    length: Time,
    /// Whether the timer is active.
    active: bool,
}

impl Timer {
    /// Initializes a new timer with the given length.
    #[must_use]
    pub const fn new(length: Time) -> Self {
        Self {
            length,
            active: true,
        }
    }

    /// Whether the timer should activate, given the current time elapsed. A timer can only activate
    /// once.
    pub fn tick(&mut self, time: Time) -> bool {
        if !self.active {
            return false;
        }

        let done = time >= self.length;
        if done {
            self.active = false;
        }
        done
    }

    /// Resets the timer, allowing it to activate again.
    pub fn reset(&mut self) {
        self.active = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test [`RawTime`] printing.
    #[test]
    fn print_raw() {
        assert_eq!(format!("{}", RawTime::YR), "31536000s");

        let pretty = format!("{:#}", RawTime::YR);
        if cfg!(feature = "human-duration") {
            assert_eq!(pretty, "1y 0mon 0d 0h 0m 0s 0ms");
        } else {
            assert_eq!(pretty, "31536000s")
        }
    }

    /// Test [`Time`] printing.
    #[test]
    fn print_time() {
        assert_eq!(
            format!("{:.2}", Time::from_sec_default(1.0)),
            "44100.00 samples"
        );
    }
}
