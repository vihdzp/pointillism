//! Implements various simple, common, or useful filter designs.
//!
//! TODO: add more first-order filters.
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

use super::{coefficients::DenseStc, DiffEq};
use crate::prelude::*;

/// [`DiffEq`] with small order, backed by two [`DenseStcs`](DenseStc).
pub type LoDiffEq<const T: usize, const U: usize> = DiffEq<DenseStc<T>, DenseStc<U>>;

/// [`DiffEq`] for an order 1 zero.
pub type SingleZero = LoDiffEq<2, 0>;

impl SingleZero {
    /// Initializes a [`SingleZero`] from coefficients normalized to `a0 = 1`.
    #[must_use]
    pub const fn new_normalized(b0: f64, b1: f64) -> Self {
        Self::new_fir(DenseStc::new([b0, b1]))
    }

    /// Initializes a [`SingleZero`] from its unnormalized coefficients.
    #[must_use]
    pub fn new(a0: f64, b0: f64, b1: f64) -> Self {
        Self::new_normalized(b0 / a0, b1 / a0)
    }

    /// Coefficients for a first order zero with 0dB max gain.
    ///
    /// This is a low-pass filter for `a ≤ 0` and a high-pass filter for `a ≥ 0`.
    ///
    /// The high-pass filters are potentially useful, but the low-pass filters only do much at high
    /// frequencies.
    #[must_use]
    pub fn single_zero(a: f64) -> Self {
        let norm = 1.0 / (1.0 + a.abs());
        Self::new_normalized(-norm, norm * a)
    }
}

/// [`DiffEq`] for an order 1 pole.
pub type SinglePole = LoDiffEq<1, 1>;

impl SinglePole {
    /// Initializes a [`SinglePole`] from coefficients normalized to `a0 = 1`.
    #[must_use]
    pub const fn new_normalized(a1: f64, b0: f64) -> Self {
        Self::new_raw(DenseStc::new([b0]), DenseStc::new([a1]))
    }

    /// Initializes a [`SinglePole`] from its unnormalized coefficients.
    #[must_use]
    pub fn new(a0: f64, a1: f64, b0: f64) -> Self {
        Self::new_normalized(a1 / a0, b0 / a0)
    }

    /// Coefficients for a first order pole with 0dB max gain.
    ///
    /// This is a low-pass filter for `a ≤ 0` and a high-pass filter for `a ≥ 0`.
    ///
    /// The low-pass filters are potentially useful, but the high-pass filters only do much at high
    /// frequencies.
    #[must_use]
    pub fn single_pole(a: f64) -> Self {
        let norm = 1.0 - a.abs();
        Self::new_normalized(norm, norm * a)
    }
}

/// [`DiffEq`] for a bilinear (order 1) filter.
pub type Bilinear = LoDiffEq<2, 1>;

/// [`DiffEq`] for a biquadratic (order 2) filter.
pub type Biquad = LoDiffEq<3, 2>;

impl Biquad {
    /// Initializes a [`Biquad`] from coefficients normalized to `a0 = 1`.
    #[must_use]
    pub const fn new_normalized(a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Self {
        Self::new_raw(DenseStc::new([b0, b1, b2]), DenseStc::new([a1, a2]))
    }

    /// Initializes a [`Biquad`] from its unnormalized coefficients.
    #[must_use]
    pub fn new(a0: f64, a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Self {
        Self::new_normalized(a1 / a0, a2 / a0, b0 / a0, b1 / a0, b2 / a0)
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

        Self::new(1.0 + a, -2.0 * wc, 1.0 - a, b0, b1, b0)
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

        Self::new(1.0 + a, -2.0 * wc, 1.0 - a, b0, -b1_neg, b0)
    }

    /// A [band-pass](https://en.wikipedia.org/wiki/Band-pass_filter) filter with a 0 dB gain.
    ///
    /// The frequency controls the peak frequency, while the [`unt::QFactor`] controls the
    /// bandwidth. You can use [`unt::QFactor::from_bw`] to set this bandwidth explicitly.
    #[must_use]
    pub fn band_pass(freq: unt::Freq, q: unt::QFactor) -> Self {
        let (ws, wc) = freq.angular().sin_cos();
        let a = ws / (2.0 * q.0);

        Self::new(1.0 + a, -2.0 * wc, 1.0 - a, a, 0.0, -a)
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

        Self::new(1.0 + a, a1, 1.0 - a, 1.0, a1, 1.0)
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

        Self::new(a0, a1, a2, a2, a1, a0)
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

        Self::new(1.0 + div, a1, 1.0 - div, 1.0 + mul, a1, 1.0 - mul)
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

        Self::new(
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

        Self::new(
            sub + mul,
            2.0 * (prv - nxt_mul),
            sub - mul,
            amp * (add + mul),
            -2.0 * amp * (prv + nxt_mul),
            amp * (add - mul),
        )
    }
}
