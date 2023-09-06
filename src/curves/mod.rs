//! Declares basic curves that may be to generate audio via [`OnceGen`](crate::prelude::OnceGen) or
//! [`LoopGen`](crate::prelude::LoopGen).
//!
//! All of the provided curves, by default, take values from `-1.0` to `1.0`. They can be rescaled
//! via the [`Comp::pos`], [`Comp::sgn`], and [`Comp::neg`] methods.
//!
//! The exception to this rule are saw waves. Due to their prevalence, we provide four variants:
//! [`Saw`], [`InvSaw`], [`PosSaw`], [`PosInvSaw`]. These vary on whether they take values from
//! `-1.0` to `1.0` or from `0.0` to `1.0`, and whether they go from left to right or right to left.
//!
//! ## Terminology
//!
//! We distinguish between two kinds of curves. The most basic curves like [`Sin`], [`SawTri`],
//! [`Pulse`], etc.) are all examples of **(plain) curves**, meaning types implementing [`Map`]
//! where the input is [`Val`] both the input and output are `f64`.
//!
//! On the other hand, **sample curves**, are types implementing [`Map`] where the input is [`Val`]
//! and the output is a [`Sample`](crate::prelude::Sample). One can create a sample curve from a
//! plain curve by using [`CurvePlayer`](crate::prelude::CurvePlayer).

pub mod interpolate;

#[cfg(feature = "hound")]
pub mod buffer;

use crate::{
    map::{Comp, Map},
    prelude::Val,
};

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pos;

impl Map for Pos {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        crate::pos(x)
    }
}

impl<F: Map<Output = f64>> Comp<F, Pos> {
    /// Composes a function with [`Pos`].
    pub const fn pos(f: F) -> Self {
        Self::new(f, Pos)
    }
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sgn;

impl Map for Sgn {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        crate::sgn(x)
    }
}

impl<F: Map<Output = f64>> Comp<F, Sgn> {
    /// Composes a function with [`Sgn`].
    pub const fn sgn(f: F) -> Self {
        Self::new(f, Sgn)
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

impl<F: Map<Output = f64>> Comp<F, Neg> {
    /// Composes a function with [`Neg`].
    pub const fn neg(f: F) -> Self {
        Self::new(f, Neg)
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

    /// Initializes the linear map that rescales an interval `[init_lo, init_hi]` to another
    /// `[end_lo, end_hi]`.
    #[must_use]
    pub fn rescale(init_lo: f64, init_hi: f64, end_lo: f64, end_hi: f64) -> Self {
        let slope = (end_hi - end_lo) / (init_hi - init_lo);
        Self::new(slope, end_lo - slope * init_lo)
    }

    /// Initializes the linear map that rescales the unit interval `[0.0, 1.0]` to another `[lo,
    /// hi]`.
    #[must_use]
    pub fn rescale_unit(lo: f64, hi: f64) -> Self {
        Self::rescale(0.0, 1.0, lo, hi)
    }

    /// Initializes the linear map that rescales the interval `[-1.0, 1.0]` to another `[lo, hi]`.
    #[must_use]
    pub fn rescale_sgn(lo: f64, hi: f64) -> Self {
        Self::rescale(-1.0, 1.0, lo, hi)
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
/// ```txt
///        ⟋
///      ⟋
/// ――――•――――  [DC = 0]
///   ⟋
/// ⟋
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Saw;

impl Map for Saw {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        crate::sgn(x.inner())
    }
}

/// A right-to-left saw wave, taking values from `1.0` to `-1.0`.
///
/// ```txt
/// ⟍
///   ⟍
/// ――――•――――  [DC = 0]
///      ⟍
///        ⟍
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct InvSaw;

impl Map for InvSaw {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        -crate::sgn(x.inner())
    }
}

/// A left-to-right saw wave, taking values from `0.0` to `1.0`.
///
/// ```txt
///         ⟋
///       ⟋
///     ⟋
///   ⟋
/// •――――――――  [DC = 0]
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct PosSaw;

impl Map for PosSaw {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        x.inner()
    }
}

/// A right-to-left saw wave, taking values from `1.0` to `0.0`.
///
/// ```txt
/// ⟍
///   ⟍
///     ⟍
///       ⟍
/// ――――――――•  [DC = 0]
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct PosInvSaw;

impl Map for PosInvSaw {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        1.0 - x.inner()
    }
}

/// A sine curve.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sin;

impl Map for Sin {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        (x.inner() * std::f64::consts::TAU).sin()
    }
}

/// A cosine curve.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cos;

impl Cos {
    /// A cosine wave.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for Cos {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        (x.inner() * std::f64::consts::TAU).cos()
    }
}

/// Returns `-1` if `x < shape`, returns `1` otherwise.
#[must_use]
pub fn pulse(x: f64, shape: f64) -> f64 {
    if x < shape {
        -1.0
    } else {
        1.0
    }
}

/// A square wave.
///
/// Takes on values between `-1.0` and `1.0`.
///
/// ```txt
///     •――――
///     |
/// ―――――――――  [DC = 0]
///     |
/// ――――•
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Sq;

impl Map for Sq {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Self::Input) -> Self::Output {
        pulse(x.inner(), 0.5)
    }
}

/// A pulse wave with a given shape.
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
    ///
    /// Unless you want to merge between other pulse waves, consider using [`Sq`] instead.
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
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        pulse(x.inner(), self.shape)
    }
}

/// Interpolates linearly between `-1.0` and `1.0` for `x ≤ shape`, and between `1.0` and `-1.0` for
/// `x ≥ shape`.
#[must_use]
pub fn saw_tri(x: f64, shape: f64) -> f64 {
    /// We must do some size checks to avoid division by 0.
    const EPS: f64 = 1e-7;

    if x < shape {
        if shape.abs() < EPS {
            1.0
        } else {
            interpolate::linear(-1.0, 1.0, Val::new(x / shape))
        }
    } else if (1.0 - shape).abs() < EPS {
        1.0
    } else {
        interpolate::linear(-1.0, 1.0, Val::new((1.0 - x) / (1.0 - shape)))
    }
}

/// A triangle wave.
///
/// Takes on values between `-1.0` and `1.0`.
///
/// ```txt
///        ⟋⟍
///      ⟋    ⟍
/// ――――•―――――――•―― [DC = 0]
///   ⟋          ⟍
/// ⟋              ⟍
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Tri;

impl Map for Tri {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        saw_tri(x.inner(), 0.5)
    }
}

/// A curve interpolating between a saw and a triangle.
///
/// Takes on values between `-1.0` and `1.0`. The `shape` corresponds to the x-coordinate of its
/// peak. For instance, a saw-tri with shape &approx; 2/3:
///
/// ```txt
///        ⟋\
///      ⟋   \
/// ――――•―――――•―― [DC = 0]
///   ⟋        \
/// ⟋           \
/// ```
#[derive(Clone, Copy, Debug)]
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
    /// Unless you want to merge between other triangle waves, consider using [`Saw`] instead.
    #[must_use]
    pub const fn saw() -> Self {
        Self::new(1.0)
    }

    /// A triangle wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using [`Tri`] instead.
    #[must_use]
    pub const fn tri() -> Self {
        Self::new(0.5)
    }

    /// A (right to left) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using [`InvSaw`] instead.
    #[must_use]
    pub const fn inv_saw() -> Self {
        Self::new(0.0)
    }
}

impl Default for SawTri {
    fn default() -> Self {
        Self::tri()
    }
}

impl Map for SawTri {
    type Input = Val;
    type Output = f64;

    fn eval(&self, x: Val) -> f64 {
        saw_tri(x.inner(), self.shape)
    }
}
