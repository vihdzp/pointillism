//! This file is largely based off the renowned [Audio EQ
//! Cookbook](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html) by Robert
//! Bristow-Johnson.

use std::f64::consts::TAU;

use super::Biquad;
use crate::prelude::*;

/// The [Q factor](https://en.wikipedia.org/wiki/Q_factor) of a filter.
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
    pub fn from_bw(bw: Interval) -> Self {
        Self::new(bw.ratio.sqrt() / (bw.ratio - 1.0))
    }

    /// Initializes a Q factor from the "shelf slope", and the filter gain.
    ///
    /// The value S = 1, corresponding to Q = 1 / √2, is the steepest for which the frequency gain
    /// remains monotonic.
    #[must_use]
    pub fn from_slope(slope: f64, vol: Vol) -> Self {
        let a = vol.gain.sqrt();
        Self::new(1.0 / ((a + 1.0 / a) * (1.0 / slope - 1.0) + 2.0).sqrt())
    }
}

impl Biquad {
    /// Initializes a [`Biquad`] from the explicit normalized coefficients.
    #[must_use]
    pub const fn new_biquad_normalized(a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Self {
        Self::new_normalized([b0, b1, b2], [a1, a2])
    }

    /// Initializes a [`Biquad`] from coefficients, which are then normalized.
    #[must_use]
    pub fn new_biquad(a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Self {
        Self::new_biquad_normalized(a1 / a0, a2 / a0, b0 / a0, b1 / a0, b2 / a0)
    }

    /// A [low-pass](https://en.wikipedia.org/wiki/Low-pass_filter) filter.
    ///
    /// The frequency controls what frequencies are cut off, and the [`QFactor`] controls the
    /// "resonance". Low factors result in a deeper cut, while high factors create a peak at the
    /// filter's frequency.
    #[must_use]
    pub fn low_pass(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = (freq.samples * TAU).sin_cos();
        let a = ws / (2.0 * q.0);
        let b1 = 1.0 - wc;
        let b0 = b1 / 2.0;

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, b0, b1, b0)
    }

    /// A [hi-pass](https://en.wikipedia.org/wiki/High-pass_filter) filter.
    ///
    /// The frequency controls what frequencies are cut off, and the [`QFactor`] controls the
    /// "resonance". Low factors result in a deeper cut, while high factors create a peak at the
    /// filter's frequency.
    #[must_use]
    pub fn hi_pass(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = (freq.samples * TAU).sin_cos();
        let a = ws / (2.0 * q.0);
        let b1_neg = 1.0 + wc;
        let b0 = b1_neg / 2.0;

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, b0, -b1_neg, b0)
    }

    /// A [band-pass](https://en.wikipedia.org/wiki/Band-pass_filter) filter with a 0 dB gain.
    ///
    /// The frequency controls the peak frequency, while the [`QFactor`] controls the bandwidth. You
    /// can use [`QFactor::from_bw`] to set this bandwidth explicitly.
    #[must_use]
    pub fn band_pass(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = (freq.samples * TAU).sin_cos();
        let a = ws / (2.0 * q.0);

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, a, 0.0, -a)
    }

    /// A [notch filter](https://en.wikipedia.org/wiki/Band-stop_filter).
    ///
    /// The frequency controls the removed frequency, while the [`QFactor`] controls the bandwidth.
    /// You can use [`QFactor::from_bw`] to set this bandwidth explicitly.
    #[must_use]
    pub fn notch(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = (freq.samples * TAU).sin_cos();
        let a = ws / (2.0 * q.0);
        let a1 = -2.0 * wc;

        Self::new_biquad(1.0 + a, a1, 1.0 - a, 1.0, a1, 1.0)
    }

    /// An [all-pass](https://en.wikipedia.org/wiki/Band-stop_filter) filter.
    ///
    /// The frequency passed is the frequency at which the phase shift is π / 2, while the
    /// [`QFactor`] controls how steep the change in phase is. High values are steeper.
    #[must_use]
    pub fn all_pass(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = (freq.samples * TAU).sin_cos();
        let a = ws / (2.0 * q.0);
        let a0 = 1.0 + a;
        let a1 = -2.0 * wc;
        let a2 = 1.0 - a;

        Self::new_biquad(a0, a1, a2, a2, a1, a0)
    }

    /// A peaking filter.
    ///
    /// The frequency passed is the peak frequency. The [`Vol`] argument controls the peak gain. The
    /// [`QFactor`] controls the bandwidth of the filter. You can use [`QFactor::from_bw`] to set
    /// this bandwidth explicitly.
    #[must_use]
    pub fn peaking(freq: Freq, _vol: Vol, q: QFactor) -> Self {
        let (ws, wc) = freq.samples.sin_cos();
        let a1 = -2.0 * wc;
        let a = ws / (2.0 * q.0);

        Self::new_biquad(1.0 + a, a1, 1.0 - a, 1.0, a1, 1.0)
    }
}
