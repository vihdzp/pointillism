//! Implements various simple, common, or useful filter designs.
//!
//! ## Sources
//!
//! Info on first-order designs retrieved from [First Order Digital Filters--An Audio
//! Cookbook](https://web.archive.org/web/20230711111226/http://freeverb3vst.osdn.jp/doc/AN11.pdf)
//! by Christopher Moore.
//!
//! Biquadratic filter designs are adapted from the renowned [Audio EQ
//! Cookbook](https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html) by Robert
//! Bristow-Johnson.

use super::Coefficients;
use crate::prelude::*;

impl<const T: usize, const U: usize> Coefficients<T, U> {
    /// Zero all coefficients of the filter.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new_normalized([0.0; T], [0.0; U])
    }
}

impl Default for Coefficients<0, 0> {
    fn default() -> Self {
        Self::zero()
    }
}

impl Coefficients<1, 0> {
    /// Coefficients for the filter that simply modifies the signal volume.
    #[must_use]
    pub const fn gain(vol: unt::Vol) -> Self {
        Self::new_fir([vol.gain])
    }

    /// Coefficients for the filter that returns the original signal, unaltered.
    #[must_use]
    pub const fn trivial() -> Self {
        Self::gain(unt::Vol::FULL)
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
    #[must_use]
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
    #[must_use]
    pub fn single_pole(a: f64) -> Self {
        let norm = 1.0 - a.abs();
        Self::new_normalized([norm], [norm * a])
    }
}

/// [`Coefficients`] for a biquadratic (order 2) filter.
pub type Biquad = Coefficients<3, 2>;

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
    /// The frequency controls what frequencies are cut off, and the [`unt::QFactor`] controls the
    /// "resonance". Low factors result in a deeper cut, while high factors create a peak at the
    /// filter's frequency.
    #[must_use]
    pub fn low_pass(freq: unt::Freq, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);
        let b1 = 1.0 - wc;
        let b0 = b1 / 2.0;

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, b0, b1, b0)
    }

    /// A [hi-pass](https://en.wikipedia.org/wiki/High-pass_filter) filter.
    ///
    /// The frequency controls what frequencies are cut off, and the [`unt::QFactor`] controls the
    /// "resonance". Low factors result in a deeper cut, while high factors create a peak at the
    /// filter's frequency.
    #[must_use]
    pub fn hi_pass(freq: unt::Freq, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);
        let b1_neg = 1.0 + wc;
        let b0 = b1_neg / 2.0;

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, b0, -b1_neg, b0)
    }

    /// A [band-pass](https://en.wikipedia.org/wiki/Band-pass_filter) filter with a 0 dB gain.
    ///
    /// The frequency controls the peak frequency, while the [`unt::QFactor`] controls the
    /// bandwidth. You can use [`unt::QFactor::from_bw`] to set this bandwidth explicitly.
    #[must_use]
    pub fn band_pass(freq: unt::Freq, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);

        Self::new_biquad(1.0 + a, -2.0 * wc, 1.0 - a, a, 0.0, -a)
    }

    /// A [notch filter](https://en.wikipedia.org/wiki/Band-stop_filter).
    ///
    /// The frequency controls the removed frequency, while the [`unt::QFactor`] controls the
    /// bandwidth. You can use [`unt::QFactor::from_bw`] to set this bandwidth explicitly.
    #[must_use]
    pub fn notch(freq: unt::Freq, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);
        let a1 = -2.0 * wc;

        Self::new_biquad(1.0 + a, a1, 1.0 - a, 1.0, a1, 1.0)
    }

    /// An [all-pass](https://en.wikipedia.org/wiki/Band-stop_filter) filter.
    ///
    /// The frequency passed is the frequency at which the phase shift is π / 2, while the
    /// [`unt::QFactor`] controls how steep the change in phase is. High values are steeper.
    #[must_use]
    pub fn all_pass(freq: unt::Freq, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);
        let a0 = 1.0 + a;
        let a1 = -2.0 * wc;
        let a2 = 1.0 - a;

        Self::new_biquad(a0, a1, a2, a2, a1, a0)
    }

    /// A peaking filter.
    ///
    /// The frequency passed is the peak frequency. The [`unt::Vol`] argument controls the peak
    /// gain. The [`unt::QFactor`] controls the bandwidth of the filter. You can use
    /// [`unt::QFactor::from_bw`] to set this bandwidth explicitly.
    ///
    /// ## Panics
    ///
    /// Panics if the volume is less or equal to zero.
    ///
    /// If you want to remove a frequency, use a [`notch`](Self::notch) filter instead.
    #[must_use]
    pub fn peaking(freq: unt::Freq, vol: unt::Vol, q: unt::QFactor) -> Self {
        assert!(vol.gain > 0.0);

        let (ws, wc) = freq.angular().sin_cos();
        let a1 = -2.0 * wc;

        let amp = vol.gain.sqrt();
        let a = ws / (2.0 * q.0);
        let mul = a * amp;
        let div = a / amp;

        Self::new_biquad(1.0 + div, a1, 1.0 - div, 1.0 + mul, a1, 1.0 - mul)
    }

    /// A low shelf filter.
    ///
    /// The frequency passed is the corner frequency. The [`unt::Vol`] argument controls the shelf
    /// gain. The [`unt::QFactor`] controls the bandwidth of the filter. You can use
    /// [`unt::QFactor::from_bw`] to set this bandwidth explicitly.
    ///
    /// Very low volumes will result in more higher frequencies getting cut off. When the volume is
    /// zero, the signal will also be zero.
    #[must_use]
    pub fn low_shelf(freq: unt::Freq, vol: unt::Vol, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();

        let amp = vol.gain.sqrt();
        let amp_sqrt = amp.sqrt();
        let mul = amp_sqrt * ws / q.0;

        let nxt = amp + 1.0;
        let prv = amp - 1.0;
        let nxt_mul = nxt * wc;
        let prv_mul = prv * wc;

        let add = nxt + prv_mul;
        let sub = nxt - prv_mul;

        Self::new_biquad(
            add + mul,
            -2.0 * (prv + nxt_mul),
            add - mul,
            amp * (sub + mul),
            2.0 * amp * (prv - nxt_mul),
            amp * (sub - mul),
        )
    }

    /// A high shelf filter.
    ///
    /// The frequency passed is the corner frequency. The [`unt::Vol`] argument controls the shelf
    /// gain. The [`unt::QFactor`] controls the bandwidth of the filter. You can use
    /// [`unt::QFactor::from_bw`] to set this bandwidth explicitly.
    ///
    /// Very low volumes will result in more lower frequencies getting cut off. When the volume is
    /// zero, the signal will also be zero.
    #[must_use]
    pub fn hi_shelf(freq: unt::Freq, vol: unt::Vol, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();

        let amp = vol.gain.sqrt();
        let amp_sqrt = amp.sqrt();
        let mul = amp_sqrt * ws / q.0;

        let nxt = amp + 1.0;
        let prv = amp - 1.0;
        let nxt_mul = nxt * wc;
        let prv_mul = prv * wc;

        let add = nxt + prv_mul;
        let sub = nxt - prv_mul;

        Self::new_biquad(
            sub + mul,
            2.0 * (prv - nxt_mul),
            sub - mul,
            amp * (add + mul),
            -2.0 * amp * (prv + nxt_mul),
            amp * (add - mul),
        )
    }
}
