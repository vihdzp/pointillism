//! Implements various simple, common, or useful filter designs.
//!
//! ## Sources
//!
//! Info on first-order designs retrieved from [First Order Digital Filters--An Audio
//! Cookbook](http://freeverb3vst.osdn.jp/doc/AN11.pdf) by Christopher Moore.
//!
//! Biquadratic filter designs are adapted from the renowned [Audio EQ
//! Cookbook](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html) by Robert
//! Bristow-Johnson.

use std::f64::consts::TAU;

use super::{Biquad, Coefficients};
use crate::prelude::*;

impl Coefficients<0, 0> {
    /// Coefficients for the filter that returns nothing, no matter the output.
    pub const fn zero() -> Self {
        Self::new_fir([])
    }
}

impl Default for Coefficients<0, 0> {
    fn default() -> Self {
        Self::zero()
    }
}

impl Coefficients<1, 0> {
    /// Coefficients for the filter that simply modifies the signal volume.
    pub const fn gain(vol: Vol) -> Self {
        Self::new_fir([vol.gain])
    }

    /// Coefficients for the filter that returns the original signal, unaltered.
    pub const fn trivial() -> Self {
        Self::gain(Vol::FULL)
    }
}

impl Default for Coefficients<1, 0> {
    fn default() -> Self {
        Self::trivial()
    }
}

impl Coefficients<2, 0> {
    /// Coefficients for a first order zero with 0dB max gain.
    ///
    /// This is a low-pass filter for `a ≤ 0` and a high-pass filter for `a ≥ 0`.
    ///
    /// The high-pass filters are potentially useful, but the low-pass filters only do much at high
    /// frequencies.
    pub fn single_zero(a: f64) -> Self {
        let norm = 1.0 / (1.0 + a.abs());
        Self::new_fir([-norm, norm * a])
    }
}

impl Coefficients<1, 1> {
    /// Coefficients for a first order pole with 0dB max gain.
    ///
    /// This is a low-pass filter for `a ≤ 0` and a high-pass filter for `a ≥ 0`.
    ///
    /// The low-pass filters are potentially useful, but the high-pass filters only do much at high
    /// frequencies.
    pub fn single_pole(a: f64) -> Self {
        let norm = 1.0 - a.abs();
        Self::new_normalized([norm], [norm * a])
    }
}

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
        Self::new([b0, b1, b2], [a0, a1, a2])
    }

    /// A [low-pass](https://en.wikipedia.org/wiki/Low-pass_filter) filter.
    ///
    /// The frequency controls what frequencies are cut off, and the [`QFactor`] controls the
    /// "resonance". Low factors result in a deeper cut, while high factors create a peak at the
    /// filter's frequency.
    #[must_use]
    pub fn low_pass(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
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
        let (ws, wc) = freq.angular().sin_cos();
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
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, a, 0.0, -a)
    }

    /// A [notch filter](https://en.wikipedia.org/wiki/Band-stop_filter).
    ///
    /// The frequency controls the removed frequency, while the [`QFactor`] controls the bandwidth.
    /// You can use [`QFactor::from_bw`] to set this bandwidth explicitly.
    #[must_use]
    pub fn notch(freq: Freq, q: QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
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
        let (ws, wc) = freq.angular().sin_cos();
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
    pub fn peaking(freq: Freq, vol: Vol, q: QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a1 = -2.0 * wc;

        let amp = vol.gain.sqrt();
        let a = ws / (2.0 * q.0);
        let axa = a * amp;
        let ada = a / amp;

        Self::new_biquad(1.0 + ada, a1, 1.0 - ada, 1.0 + axa, a1, 1.0 - axa)
    }

    /// A low shelf filter.
    ///
    /// The frequency passed is the corner frequency. The [`Vol`] argument controls the shelf gain.
    /// The [`QFactor`] controls the bandwidth of the filter. You can use [`QFactor::from_bw`] to
    /// set this bandwidth explicitly.
    #[must_use]
    pub fn low_shelf(freq: Freq, vol: Vol, q: QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();

        let amp = vol.gain.sqrt();
        let amp_sqrt = amp.sqrt();
        let axa = amp_sqrt * ws / q.0;

        let ap1 = amp + 1.0;
        let am1 = amp - 1.0;
        let ap1w = ap1 * wc;
        let am1w = am1 * wc;

        let apa = ap1 + am1w;
        let ama = ap1 - am1w;

        Self::new_biquad(
            apa + axa,
            -2.0 * (am1 + ap1w),
            apa - axa,
            amp * (ama + axa),
            2.0 * amp * ama,
            amp * (ama - axa),
        )
    }

    /// A high shelf filter.
    ///
    /// The frequency passed is the corner frequency. The [`Vol`] argument controls the shelf gain.
    /// The [`QFactor`] controls the bandwidth of the filter. You can use [`QFactor::from_bw`] to
    /// set this bandwidth explicitly.
    #[must_use]
    pub fn hi_shelf(freq: Freq, vol: Vol, q: QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();

        let amp = vol.gain.sqrt();
        let amp_sqrt = amp.sqrt();
        let axa = amp_sqrt * ws / q.0;

        let ap1 = amp + 1.0;
        let am1 = amp - 1.0;
        let ap1w = ap1 * wc;
        let am1w = am1 * wc;

        let apa = ap1 + am1w;
        let ama = ap1 - am1w;

        Self::new_biquad(
            ama + axa,
            2.0 * (am1 - ap1w),
            ama - axa,
            amp * (apa + axa),
            -2.0 * amp * apa,
            amp * (apa - axa),
        )
    }
}
