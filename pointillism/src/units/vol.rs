//! Implements the [`Vol`] type, and defines many associated constants.

use crate::prelude::*;

/// Represents the gain of some signal.
///
/// You can use [`Self::new_db`] and [`Self::db`] to convert gain into decibels, and viceversa. A
/// unit gain corresponds to 0 dB.
///
/// This also implements the [`map::Map`] trait, thus doubling as a function that multiplies the
/// volume of a signal.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vol {
    /// Gain factor.
    pub gain: f64,
}

impl Vol {
    /// Silence.
    pub const ZERO: Self = Self::new(0.0);
    /// Half amplitude.
    pub const HALF: Self = Self::new(0.5);
    /// Full volume.
    pub const FULL: Self = Self::new(1.0);
    /// Twice the amplitude.
    pub const TWICE: Self = Self::new(2.0);

    /// -3 dB.
    ///
    /// Roughly corresponds to a halving of power.
    pub const MDB3: Self = Self::new(0.707_945_784_384_137_9);
    /// -6 dB.
    ///
    /// Roughly corresponds to a halving of amplitude, voltage, or sound power level (SPL).
    pub const MDB6: Self = Self::new(0.501_187_233_627_272_2);
    /// -10 dB.
    ///
    /// What a human might percieve as "half as loud".
    pub const MDB10: Self = Self::new(0.316_227_766_016_837_94);

    /// +3 dB.
    ///
    /// Roughly corresponds to a doubling of power.
    pub const DB3: Self = Self::new(1.412_537_544_622_754_4);
    /// +6 dB.
    ///
    /// Roughly corresponds to a doubling of amplitude, voltage, or sound power level (SPL).
    pub const DB6: Self = Self::new(1.995_262_314_968_879_5);
    /// +10 dB.
    ///
    /// What a human might percieve as "twice as loud".
    pub const DB10: Self = Self::new(3.162_277_660_168_379_5);

    /// Initializes a new volume variable.
    #[must_use]
    pub const fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Gain measured in decibels.
    #[must_use]
    pub fn new_db(db: f64) -> Self {
        Self::new(10f64.powf(db / 20.0))
    }

    /// Linearly converts MIDI velocity into gain.
    ///
    /// This is not necessarily the best way to interpret MIDI velocity, but it is the simplest.
    #[cfg(feature = "midly")]
    #[must_use]
    pub fn new_vel(vel: midly::num::u7) -> Self {
        Self::new(f64::from(vel.as_int()) / 127.0)
    }

    /// The gain in decibels.
    #[must_use]
    pub fn db(&self) -> f64 {
        20.0 * self.gain.log10()
    }
}

impl Default for Vol {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl map::Map for Vol {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        x * self.gain
    }
}
