//! Declares basic curves that may be used in envelopes via
//! [`CurveEnv`](crate::generators::CurveEnv), or to generate audio
//! [`CurveGen`](crate::generators::CurveGen).

use crate::Map;

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ToPos<C: Map<f64, f64>>(pub C);

impl<C: Map<f64, f64>> Map<f64, f64> for ToPos<C> {
    fn eval(&self, x: f64) -> f64 {
        crate::to_pos(self.0.eval(x))
    }
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ToSgn<C: Map<f64, f64>>(pub C);

impl<C: Map<f64, f64>> Map<f64, f64> for ToSgn<C> {
    fn eval(&self, x: f64) -> f64 {
        crate::to_sgn(self.0.eval(x))
    }
}

/// A sine curve.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sin {
    /// Phase of the sine curve, from `0.0` to `1.0`.
    pub phase: f64,
}

impl Sin {
    /// A sine wave with a given phase.
    pub const fn new(phase: f64) -> Self {
        Self { phase }
    }

    /// A sine wave.
    #[allow(clippy::self_named_constructors)]
    pub const fn sin() -> Self {
        Self::new(0.0)
    }

    /// A cosine wave.
    pub const fn cos() -> Self {
        Self::new(0.25)
    }
}

impl Map<f64, f64> for Sin {
    fn eval(&self, x: f64) -> f64 {
        ((x + self.phase) * std::f64::consts::TAU).sin()
    }
}

/// A pulse wave.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug)]
pub struct Pulse {
    /// Shape of the pulse curve, from `0.0` to `1.0`.
    pub shape: f64,
}

impl Pulse {
    /// A pulse wave.
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }

    /// A square wave.
    pub const fn sq() -> Self {
        Self::new(0.5)
    }
}

impl Default for Pulse {
    fn default() -> Self {
        Self::sq()
    }
}

impl Map<f64, f64> for Pulse {
    fn eval(&self, x: f64) -> f64 {
        if x < self.shape {
            1.0
        } else {
            -1.0
        }
    }
}

/// A left-to-right saw wave, taking values from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Saw;

impl Map<f64, f64> for Saw {
    fn eval(&self, x: f64) -> f64 {
        x
    }
}

/// A right-to-left saw wave, taking values from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct InvSaw;

impl Map<f64, f64> for InvSaw {
    fn eval(&self, x: f64) -> f64 {
        -x
    }
}

/// A left-to-right saw wave, taking values from `0.0` to `1.0`.
pub type PosSaw = ToPos<Saw>;

/// A right-to-left saw wave, taking values from `0.0` to `1.0`.
pub type PosInvSaw = ToPos<InvSaw>;

/// A curve interpolating between a saw and a triangle.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SawTri {
    /// Position of the peak of the wave.
    pub shape: f64,
}

impl SawTri {
    /// A saw-triangle curve.
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }

    /// A (left to right) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using
    /// [`Saw`] instead.
    pub const fn saw() -> Self {
        Self::new(1.0)
    }

    /// A triangle wave.
    pub const fn tri() -> Self {
        Self::new(0.5)
    }

    /// A (right to left) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using
    /// [`InvSaw`] instead.
    pub const fn inv_saw() -> Self {
        Self::new(0.0)
    }
}

impl Map<f64, f64> for SawTri {
    fn eval(&self, x: f64) -> f64 {
        // We must do some size checks to avoid division by 0.
        if x < self.shape {
            if self.shape.abs() < 1e-7 {
                1.0
            } else {
                2.0 * x / self.shape - 1.0
            }
        } else if (1.0 - self.shape).abs() < 1e-7 {
            1.0
        } else {
            2.0 * (1.0 - x) / (1.0 - self.shape) - 1.0
        }
    }
}
