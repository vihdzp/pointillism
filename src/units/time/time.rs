use crate::prelude::*;
use crate::units::FracInt;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// A time, measured in **samples**.
///
/// Note that in order to convert between a [`RawTime`] in seconds and this type, you must know the
/// [`SampleRate`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Time {
    /// The number of samples.
    ///
    /// This is stored as our custom type [`FracInt`] and not as a float, as this allows us to
    /// entirely get rid of cumulative error when incrementing a time by a sample.
    pub samples: FracInt,
}

impl Time {
    /// Initializes a time in **samples**.
    ///
    /// If you want to use the more natural unit of seconds, see [`Self::from_raw`].
    pub const fn new(samples: FracInt) -> Self {
        Self { samples }
    }

    /// The time in samples.
    pub const fn samples(self) -> FracInt {
        self.samples
    }

    /// Converts [`RawTime`] into [`Time`], using the specified sample rate.
    pub fn from_raw(raw: RawTime, sample_rate: SampleRate) -> Self {
        Self::new(FracInt::from(raw.seconds() * f64::from(sample_rate)))
    }

    /// Converts [`RawTime`] into [`Time`], using the default sample rate.
    pub fn from_raw_default(raw: RawTime) -> Self {
        Self::from_raw(raw, SampleRate::default())
    }

    /// Initializes a [`Time`] from the value in seconds, and a sample rate.
    pub fn from_sec(seconds: f64, sample_rate: SampleRate) -> Self {
        Self::from_raw(RawTime::new(seconds), sample_rate)
    }

    /// Initializes a [`Time`] from the value in seconds, using the default sample rate.
    pub fn from_sec_default(hz: f64) -> Self {
        Self::from_sec(hz, SampleRate::default())
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} samples", self.samples)
    }
}
