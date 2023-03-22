//! Different kinds of signal distortion.

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

/// Applies [`InfClip`] distortion to a signal.
pub type InfClipping<S> = PointwiseMapSgn<S, InfClip>;

impl<S: Signal> InfClipping<S> {
    /// Initializes a new [`InfClipping`].
    pub const fn new(sgn: S) -> Self {
        Self::new_pointwise(sgn, InfClip)
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

/// Applies [`Clip`] distortion to a signal.
pub type Clipping<S> = PointwiseMapSgn<S, Clip>;

impl<S: Signal> Clipping<S> {
    /// Initializes a new [`Clipping`].
    pub const fn new(sgn: S, threshold: f64) -> Self {
        Self::new_pointwise(sgn, Clip::new(threshold))
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

/// Applies [`Atan`] distortion to a signal.
pub type Arctangent<S> = PointwiseMapSgn<S, Atan>;

impl<S: Signal> Arctangent<S> {
    /// Initializes a new [`Arctangent`].
    pub const fn new(sgn: S, shape: f64) -> Self {
        Self::new_pointwise(sgn, Atan::new(shape))
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
            crate::sgn(res)
        } else {
            res
        }
    }
}

/// Applies [`Pow`] distortion to a signal.
pub type Power<S> = PointwiseMapSgn<S, Pow>;

impl<S: Signal> Power<S> {
    /// Initializes a new [`Power`].
    pub const fn new(sgn: S, exponent: u16) -> Self {
        Self::new_pointwise(sgn, Pow::new(exponent))
    }

    /// No distortion.
    pub const fn linear(sgn: S) -> Self {
        Self::new(sgn, 1)
    }

    /// Cubic distortion.
    pub const fn cubic(sgn: S) -> Self {
        Self::new(sgn, 3)
    }
}

// Todo: bitcrusher effect.
