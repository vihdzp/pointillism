//! Declares basic curves that may be used in envelopes via
//! [`CurveEnv`](crate::prelude::CurveEnv), or to generate audio
//! [`CurveGen`](crate::prelude::CurveGen).
//!
//! By a curve, we mean any struct implementing 
//! `Map<Input = f64, Output = f64>`.

use crate::map::{Comp, Map};

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pos;

impl Pos {
    /// The [`crate::pos`] function.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for Pos {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        crate::pos(x)
    }
}

/// Composes a function with [`Pos`]
pub type PosComp<F> = Comp<F, Pos>;

impl<F: Map<Output = f64>> PosComp<F> {
    /// Initializes a new [`PosComp`].
    pub const fn new_pos(f: F) -> Self {
        Self::new_generic(f, Pos)
    }
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sgn;

impl Sgn {
    /// The [`crate::sgn`] function.
    ///
    /// Note that [`Saw`] is an alias for [`Sgn`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for Sgn {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        crate::sgn(x)
    }
}

/// Composes a function with [`Sgn`]
pub type SgnComp<F> = Comp<F, Sgn>;

impl<F: Map<Output = f64>> SgnComp<F> {
    /// Initializes a new [`SgnComp`].
    pub const fn new_sgn(f: F) -> Self {
        Self::new_generic(f, Sgn)
    }
}

/// Negates a floating point value.
#[derive(Clone, Copy, Debug, Default)]
pub struct Neg;

impl Neg {
    /// The negation function.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for Neg {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        -x
    }
}

/// Composes a function with [`Neg`]
pub type NegComp<F> = Comp<F, Neg>;

impl<F: Map<Output = f64>> NegComp<F> {
    /// Initializes a new [`NegComp`].
    pub const fn new_neg(f: F) -> Self {
        Self::new_generic(f, Neg)
    }
}

/// A linear map `y = mx + b`.
#[derive(Clone, Copy, Debug)]
pub struct Linear {
    /// Slope of the map.
    pub slope: f64,

    /// y-intercept of the map.
    pub intercept: f64,
}

impl Linear {
    /// Initializes a new linear map.
    #[must_use]
    pub const fn new(slope: f64, intercept: f64) -> Self {
        Self { slope, intercept }
    }

    /// Initializes the linear map that rescales an interval to another.
    #[must_use]
    pub fn rescale(init_lo: f64, init_hi: f64, end_lo: f64, end_hi: f64) -> Self {
        let slope = (end_hi - end_lo) / (init_hi - init_lo);
        Self::new(slope, end_lo - slope * init_lo)
    }
}

impl Map for Linear {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        x * self.slope + self.intercept
    }
}

/// A left-to-right saw wave, taking values from `-1.0` to `1.0`.
///
/// Note that this is a type alias for [`Sgn`].
pub type Saw = Sgn;

/// A right-to-left saw wave, taking values from `-1.0` to `1.0`.
pub type InvSaw = NegComp<Saw>;

impl InvSaw {
    /// Initializes a new [`InvSaw`].
    #[must_use]
    pub const fn new() -> Self {
        Self::new_neg(Saw::new())
    }
}

/// A left-to-right saw wave, taking values from `0.0` to `1.0`.
///
/// Note that this is a type alias for [`Id`](crate::Id).
pub type PosSaw = crate::Id<f64>;

/// A right-to-left saw wave, taking values from `0.0` to `1.0`.
pub type PosInvSaw = PosComp<InvSaw>;

impl PosInvSaw {
    /// Initializes a new [`PosInvSaw`].
    #[must_use]
    pub const fn new() -> Self {
        Self::new_pos(InvSaw::new())
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
    #[must_use]
    pub const fn new(phase: f64) -> Self {
        Self { phase }
    }

    /// A sine wave.
    #[allow(clippy::self_named_constructors)]
    #[must_use]
    pub const fn sin() -> Self {
        Self::new(0.0)
    }

    /// A cosine wave.
    #[must_use]
    pub const fn cos() -> Self {
        Self::new(0.25)
    }
}

impl Map for Sin {
    type Input = f64;
    type Output = f64;

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
    #[must_use]
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }

    /// A square wave.
    #[must_use]
    pub const fn sq() -> Self {
        Self::new(0.5)
    }
}

impl Default for Pulse {
    fn default() -> Self {
        Self::sq()
    }
}

impl Map for Pulse {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        if x < self.shape {
            1.0
        } else {
            -1.0
        }
    }
}

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
    #[must_use]
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }

    /// A (left to right) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using
    /// [`Saw`] instead.
    #[must_use]
    pub const fn saw() -> Self {
        Self::new(1.0)
    }

    /// A triangle wave.
    #[must_use]
    pub const fn tri() -> Self {
        Self::new(0.5)
    }

    /// A (right to left) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using
    /// [`InvSaw`] instead.
    #[must_use]
    pub const fn inv_saw() -> Self {
        Self::new(0.0)
    }
}

impl Map for SawTri {
    type Input = f64;
    type Output = f64;

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
