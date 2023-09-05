use crate::units::SampleRate;

use super::raw_freq::RawFreq;

/// A frequency, measured in inverse samples.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Freq {
    /// The frequency in inverse samples.
    samples: f64,
}

impl Freq {
    /// Converts [`RawFreq`] into [`Freq`], using the specified sample rate.
    pub fn from_raw_with(raw: RawFreq, sample_rate: SampleRate) -> Self {
        Self {
            samples: raw.hz() / f64::from(sample_rate),
        }
    }

    /// Converts [`RawFreq`] into [`Freq`], using the standard 44.1 kHz sample rate.
    pub fn from_raw(raw: RawFreq) -> Self {
        Self::from_raw_with(raw, SampleRate::DEFAULT)
    }

    /// The frequency in inverse samples.
    pub const fn samples(self) -> f64 {
        self.samples
    }
}

/// We use `A4` as a default frequency, and 44.1 kHz as a default sample rate. This means that, for
/// instance,
///
/// ```
/// # use pointillism::prelude::*;
/// let osc = LoopGen::<Mono, Sin>::default();
/// ```
///
/// will result in a 440 Hz sine wave when sampled at 44.1 kHz.
impl Default for Freq {
    fn default() -> Self {
        Freq::from_raw(RawFreq::default())
    }
}
