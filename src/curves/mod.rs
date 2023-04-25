//! Declares basic curves that may be to generate audio via
//! [`OneshotGen`](crate::prelude::OneshotGen) or
//! [`LoopGen`](crate::prelude::LoopGen).
//!
//! By a curve, we mean any struct implementing
//! [`Map<Input = f32, Output = f32>`](Map).
//!
//! For convenience, we provide four variants of a saw wave: [`Saw`],
//! [`InvSaw`], [`PosSaw`], [`PosInvSaw`]. These vary on whether they take
//! values from `-1.0` to `1.0` or from `0.0` to `1.0`, and whether they go from
//! left to right or right to left.
//!
//! All other of the provided curves, by default, take values from `-1.0` to
//! `1.0`. They can be rescaled via the [`Comp::pos`], [`Comp::sgn`], and
//! [`Comp::neg`] methods.

pub mod buffer;

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
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        crate::pos(x)
    }
}

impl<F: Map<Output = f32>> Comp<F, Pos> {
    /// Composes a function with [`Pos`].
    pub const fn pos(f: F) -> Self {
        Self::new(f, Pos)
    }
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sgn;

impl Sgn {
    /// The [`crate::sgn`] function.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for Sgn {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        crate::sgn(x)
    }
}

impl<F: Map<Output = f32>> Comp<F, Sgn> {
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
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        -x
    }
}

impl<F: Map<Output = f32>> Comp<F, Neg> {
    /// Composes a function with [`Neg`].
    pub const fn neg(f: F) -> Self {
        Self::new(f, Neg)
    }
}

/// A linear map `y = mx + b`.
#[derive(Clone, Copy, Debug)]
pub struct Linear {
    /// Slope of the map.
    pub slope: f32,

    /// y-intercept of the map.
    pub intercept: f32,
}

impl Linear {
    /// Initializes a new linear map.
    #[must_use]
    pub const fn new(slope: f32, intercept: f32) -> Self {
        Self { slope, intercept }
    }

    /// Initializes the linear map that rescales an interval
    /// `[init_lo, init_hi]` to another `[end_lo, end_hi]`.
    #[must_use]
    pub fn rescale(init_lo: f32, init_hi: f32, end_lo: f32, end_hi: f32) -> Self {
        let slope = (end_hi - end_lo) / (init_hi - init_lo);
        Self::new(slope, end_lo - slope * init_lo)
    }
}

impl Map for Linear {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        x * self.slope + self.intercept
    }
}

/// A left-to-right saw wave, taking values from `-1.0` to `1.0`.
///
/// ```txt
/// ⟍
///   ⟍
/// ――――•――――  [DC = 0]
///      ⟍
///        ⟍
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Saw;

impl Saw {
    /// Initializes a new [`Saw`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for Saw {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        crate::sgn(x)
    }
}

/// A right-to-left saw wave, taking values from `1.0` to `-1.0`.
///
/// ```txt
///        ⟋
///      ⟋
/// ――――•――――  [DC = 0]
///   ⟋
/// ⟋
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct InvSaw;

impl InvSaw {
    /// Initializes a new [`InvSaw`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for InvSaw {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        -crate::sgn(x)
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
pub struct PosSaw;

impl PosSaw {
    /// Initializes a new [`PosSaw`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for PosSaw {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        x
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
pub struct PosInvSaw;

impl PosInvSaw {
    /// Initializes a new [`PosInvSaw`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Map for PosInvSaw {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        1.0 - x
    }
}

/// A sine curve.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sin {
    /// Phase of the sine curve, from `0.0` to `1.0`.
    pub phase: f32,
}

impl Sin {
    /// A sine wave with a given phase.
    #[must_use]
    pub const fn new(phase: f32) -> Self {
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

/// Evaluates `sin((x + phase) * τ)`.
#[must_use]
pub fn sin(x: f32, phase: f32) -> f32 {
    ((x + phase) * std::f32::consts::TAU).sin()
}

impl Map for Sin {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        sin(x, self.phase)
    }
}

/// A pulse wave.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug)]
pub struct Pulse {
    /// Shape of the pulse curve, from `0.0` to `1.0`.
    pub shape: f32,
}

impl Pulse {
    /// A pulse wave.
    #[must_use]
    pub const fn new(shape: f32) -> Self {
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

/// Returns `1` if `x < shape`, returns `-1` otherwise.
#[must_use]
pub fn pulse(x: f32, shape: f32) -> f32 {
    if x < shape {
        1.0
    } else {
        -1.0
    }
}

impl Map for Pulse {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        pulse(x, self.shape)
    }
}

/// A curve interpolating between a saw and a triangle.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SawTri {
    /// Position of the peak of the wave.
    pub shape: f32,
}

impl SawTri {
    /// A saw-triangle curve.
    #[must_use]
    pub const fn new(shape: f32) -> Self {
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

/// Interpolates linearly between `-1.0` and `1.0` for `x ≤ shape`, and between
/// `1.0` and `-1.0` for `x ≥ shape`.
#[must_use]
pub fn saw_tri(x: f32, shape: f32) -> f32 {
    /// We must do some size checks to avoid division by 0.
    const EPS: f32 = 1e-7;

    if x < shape {
        if shape.abs() < EPS {
            1.0
        } else {
            2.0 * x / shape - 1.0
        }
    } else if (1.0 - shape).abs() < EPS {
        1.0
    } else {
        2.0 * (1.0 - x) / (1.0 - shape) - 1.0
    }
}

impl Map for SawTri {
    type Input = f32;
    type Output = f32;

    fn eval(&self, x: f32) -> f32 {
        saw_tri(x, self.shape)
    }
}
