//! Defines different kinds of signal distortion.
//!
//! No new signal structs are defined in this file. Instead, we define new initiailzations for
//! [`eff::PwMapSgn`].

use crate::prelude::*;

/// Infinite clipping distortion.
///
/// Maps positive values to `1.0`, negative values to `-1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct InfClip;

impl Map for InfClip {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        x.signum()
    }
}

impl<S: Signal> eff::PwMapSgn<S, InfClip> {
    /// Applies [`InfClip`] distortion to a signal.
    pub const fn inf_clip(sgn: S) -> Self {
        Self::new_pw(sgn, InfClip)
    }
}

/// Clipping distortion.
///
/// Clamps all values between `-threshold` and `threshold`, and normalizes.
#[derive(Clone, Copy, Debug)]
pub struct Clip {
    /// The threshold for clipping.
    pub threshold: f64,
}

impl Clip {
    /// Initializes a new [`Clip`] struct.
    #[must_use]
    pub const fn new(threshold: f64) -> Self {
        Self { threshold }
    }
}

impl Default for Clip {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Map for Clip {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        x.clamp(-self.threshold, self.threshold) / self.threshold
    }
}

impl<S: Signal> eff::PwMapSgn<S, Clip> {
    /// Applies [`Clip`] distortion to a signal.
    pub const fn clip(sgn: S, threshold: f64) -> Self {
        Self::new_pw(sgn, Clip::new(threshold))
    }
}

/// Arctangent distortion.
///
/// Applies the function `tan⁻¹(shape * x)` to the input signal and normalizes.
#[derive(Clone, Copy, Debug)]
pub struct Atan {
    /// The shape of the distortion. Typically larger than `1.0`.
    pub shape: f64,
}

impl Atan {
    /// Initializes a new [`Atan`] struct.
    #[must_use]
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }
}

impl Default for Atan {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Map for Atan {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        (self.shape * x).atan() / std::f64::consts::FRAC_PI_2
    }
}

impl<S: Signal> eff::PwMapSgn<S, Atan> {
    /// Applies [`Atan`] distortion to a signal.
    pub const fn atan(sgn: S, shape: f64) -> Self {
        Self::new_pw(sgn, Atan::new(shape))
    }
}

/// The function `x^n`, renormalized for even exponents.
#[derive(Clone, Copy, Debug)]
pub struct Pow {
    /// Exponent to raise a number to.
    pub exponent: u16,
}

impl Pow {
    /// Initializes a new [`Pow`].
    #[must_use]
    pub const fn new(exponent: u16) -> Self {
        Self { exponent }
    }

    /// No distortion.
    #[must_use]
    pub const fn linear() -> Self {
        Self::new(1)
    }

    /// Cubic distortion.
    #[must_use]
    pub const fn cubic() -> Self {
        Self::new(3)
    }
}

impl Default for Pow {
    fn default() -> Self {
        Self::linear()
    }
}

impl Map for Pow {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        let res = x.powi(i32::from(self.exponent));

        if self.exponent % 2 == 0 {
            map::sgn(res)
        } else {
            res
        }
    }
}

impl<S: Signal> eff::PwMapSgn<S, Pow> {
    /// Applies [`Pow`] distortion to a signal.
    pub const fn pow(sgn: S, exponent: u16) -> Self {
        Self::new_pw(sgn, Pow::new(exponent))
    }

    /// Cubic distortion.
    pub const fn cubic(sgn: S) -> Self {
        Self::pow(sgn, 3)
    }
}

// Todo: bitcrusher effect.
