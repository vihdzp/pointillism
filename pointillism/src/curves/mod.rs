//! Declares basic curves that may be to generate audio via [`gen::Once`] or [`gen::Loop`].
//!
//! All of the provided curves, by default, take values from `-1.0` to `1.0`. They can be rescaled
//! via the [`map::Comp::pos`], [`map::Comp::sgn`], and [`map::Comp::neg`] methods.
//!
//! The exception to this rule are saw waves. Due to their prevalence, we provide four variants:
//! [`Saw`], [`InvSaw`], [`PosSaw`], [`PosInvSaw`]. These vary on whether they take values from
//! `-1.0` to `1.0` or from `0.0` to `1.0`, and whether they go from left to right or right to left.
//!
//! ## Terminology
//!
//! We distinguish between two kinds of curves. The most basic curves like [`Sin`], [`SawTri`],
//! [`Pulse`], etc.) are all examples of **(plain) curves**, meaning types implementing [`map::Map`]
//! where the input is [`unt::Val`] both the input and output are `f64`.
//!
//! On the other hand, **sample curves**, are types implementing [`map::Map`] where the input is
//! [`unt::Val`] and the output is a [`smp::Sample`]. One can create a sample curve from a plain
//! curve by using [`CurvePlayer`](gen::CurvePlayer).

use crate::prelude::*;

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

impl map::Map for Saw {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        map::sgn(x.inner())
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

impl map::Map for InvSaw {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        -map::sgn(x.inner())
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

impl map::Map for PosSaw {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
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

impl map::Map for PosInvSaw {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        1.0 - x.inner()
    }
}

/// A sine curve.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sin;

impl map::Map for Sin {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
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

impl map::Map for Cos {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
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

impl map::Map for Sq {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: Self::Input) -> f64 {
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

impl map::Map for Pulse {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        pulse(x.inner(), self.shape)
    }
}

/// Interpolates linearly between `-1.0` and `1.0` for `x ≤ shape`, and between `1.0` and `-1.0` for
/// `x ≥ shape`.
#[must_use]
pub fn saw_tri(x: f64, shape: unt::Val) -> f64 {
    /// We must do some size checks to avoid division by 0.
    const EPS: f64 = 1e-7;

    let shape = shape.inner();
    if x < shape {
        if shape < EPS {
            1.0
        } else {
            buf::int::linear(-1.0, 1.0, unt::Val::new(x / shape))
        }
    } else if 1.0 - shape < EPS {
        1.0
    } else {
        buf::int::linear(-1.0, 1.0, unt::Val::new((1.0 - x) / (1.0 - shape)))
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

impl map::Map for Tri {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        saw_tri(x.inner(), unt::Val::HALF)
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
    pub shape: unt::Val,
}

impl SawTri {
    /// A saw-triangle curve.
    #[must_use]
    pub const fn new(shape: unt::Val) -> Self {
        Self { shape }
    }

    /// A (left to right) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using [`Saw`] instead.
    #[must_use]
    pub const fn saw() -> Self {
        Self::new(unt::Val::ONE)
    }

    /// A triangle wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using [`Tri`] instead.
    #[must_use]
    pub const fn tri() -> Self {
        Self::new(unt::Val::HALF)
    }

    /// A (right to left) saw wave.
    ///
    /// Unless you want to merge between other triangle waves, consider using [`InvSaw`] instead.
    #[must_use]
    pub const fn inv_saw() -> Self {
        Self::new(unt::Val::ZERO)
    }
}

impl Default for SawTri {
    fn default() -> Self {
        Self::tri()
    }
}

impl map::Map for SawTri {
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        saw_tri(x.inner(), self.shape)
    }
}

/// Linearly morph between two curves.
///
/// Take note of [phase cancellation](https://en.wikipedia.org/wiki/Wave_interference)! Adding two
/// waves won't always result in an "average" sound.
pub struct Morph<
    C: map::Map<Input = unt::Val, Output = f64>,
    D: map::Map<Input = unt::Val, Output = f64>,
> {
    /// The first curve.
    pub fst: C,
    /// The second curve.
    pub snd: D,
    /// The morph amount.
    pub morph: unt::Val,
}

impl<C: map::Map<Input = unt::Val, Output = f64>, D: map::Map<Input = unt::Val, Output = f64>>
    Morph<C, D>
{
    /// Morphs between two curves.
    pub const fn new(fst: C, snd: D, morph: unt::Val) -> Self {
        Self { fst, snd, morph }
    }

    /// A morph that starts at the first curve.
    pub const fn fst(fst: C, snd: D) -> Self {
        Self::new(fst, snd, unt::Val::ZERO)
    }

    /// A morph that's halfway between both curves.
    pub const fn half(fst: C, snd: D) -> Self {
        Self::new(fst, snd, unt::Val::HALF)
    }

    /// A morph that starts at the second curve.
    pub const fn snd(fst: C, snd: D) -> Self {
        Self::new(fst, snd, unt::Val::ONE)
    }
}

impl<C: map::Map<Input = unt::Val, Output = f64>, D: map::Map<Input = unt::Val, Output = f64>>
    map::Map for Morph<C, D>
{
    type Input = unt::Val;
    type Output = f64;

    fn eval(&self, x: unt::Val) -> f64 {
        buf::int::linear(self.fst.eval(x), self.snd.eval(x), self.morph)
    }
}
