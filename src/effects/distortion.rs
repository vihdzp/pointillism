//! Different kinds of signal distortion.

use crate::{prelude::Signal, signal::PointwiseMapSgn, Map};

/// Infinite clipping distortion.
///
/// Maps positive values to `1.0`, negative values to `-1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct InfClip;

impl Map<f64, f64> for InfClip {
    fn eval(&self, x: f64) -> f64 {
        x.signum()
    }
}

/// Applies [`InfClip`] distortion to a signal.
pub type InfClipping<S> = PointwiseMapSgn<S, InfClip>;

impl<S: Signal> InfClipping<S> {
    /// Initializes a new [`InfClipping`].
    pub fn new(sgn: S) -> Self {
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
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }
}

impl Default for Clip {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Map<f64, f64> for Clip {
    fn eval(&self, x: f64) -> f64 {
        x.clamp(-self.threshold, self.threshold) / self.threshold
    }
}

/// Applies [`Clip`] distortion to a signal.
pub type Clipping<S> = PointwiseMapSgn<S, Clip>;

impl<S: Signal> Clipping<S> {
    /// Initializes a new [`Clipping`].
    pub fn new(sgn: S, threshold: f64) -> Self {
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
    pub fn new(shape: f64) -> Self {
        Self { shape }
    }
}

impl Default for Atan {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Map<f64, f64> for Atan {
    fn eval(&self, x: f64) -> f64 {
        (self.shape * x).atan() / std::f64::consts::FRAC_PI_2
    }
}

/// Applies [`Atan`] distortion to a signal.
pub type Arctangent<S> = PointwiseMapSgn<S, Atan>;

impl<S: Signal> Arctangent<S> {
    /// Initializes a new [`Arctangent`].
    pub fn new(sgn: S, shape: f64) -> Self {
        Self::new_pointwise(sgn, Atan::new(shape))
    }
}

// Todo: bitcrusher effect.
