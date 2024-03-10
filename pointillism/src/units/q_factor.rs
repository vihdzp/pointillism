//! Implements the [`QFactor`] type.

use crate::prelude::*;

/// The [Q factor](https://en.wikipedia.org/wiki/Q_factor) of a filter. Used in the construction of
/// [`eff::flt::Biquad`] filters.
///
/// We provide convenience methods [`Self::from_bw`] and [`Self::from_slope`] which create this
/// value from other units.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct QFactor(pub f64);

impl QFactor {
    /// Initializes a Q factor.
    ///
    /// This should be a positive quantity.
    #[must_use]
    pub const fn new(q: f64) -> Self {
        Self(q)
    }

    /// Initializes a Q factor from the given bandwidth. This bandwidth spans:
    ///
    /// - The interval between -3 dB frequencies, for band-pass filters and notch filters.
    /// - The interval between half gain frequencies, for peaking filters.
    #[must_use]
    pub fn from_bw(bw: unt::Interval) -> Self {
        Self::new(bw.ratio.sqrt() / (bw.ratio - 1.0))
    }

    /// Initializes a Q factor from the "shelf slope", and the filter gain.
    ///
    /// The value S = 1, corresponding to Q = 1 / √2, is the steepest for which the frequency gain
    /// remains monotonic.
    #[must_use]
    pub fn from_slope(slope: f64, vol: unt::Vol) -> Self {
        let a = vol.gain.sqrt();
        Self::new(1.0 / ((a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0).sqrt())
    }
}
