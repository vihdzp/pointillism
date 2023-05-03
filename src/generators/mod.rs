//! Implements generators of all kinds.
//!
//! Generators are structures that generate [`Signals`](crate::prelude::Signal) on their own, be
//! they envelope or audio data.
//!
//! The module file provides the most basic examples of generators, namely generators that read data
//! from a curve. See the [curves module docs](../curves/index.html#terminology) for an explanation
//! on different kinds of curves.

use std::marker::PhantomData;

pub use crate::prelude::*;

pub mod poly;
pub mod sequence;

/// Converts a plain curve into a sample curve that outputs a signal of the specified type.
#[derive(Clone, Copy, Debug, Default)]
pub struct CurvePlayer<S: Sample, C: Map<Input = f64, Output = f64>> {
    /// The inner plain curve.
    pub curve: C,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> CurvePlayer<S, C> {
    /// Initializes a new [`CurvePlayer`].
    ///
    /// You might need to explicitly specify the type of sample you want to play the curve as, via
    /// `CurvePlayer::new::<S>`.
    pub const fn new(curve: C) -> Self {
        Self {
            curve,
            phantom: PhantomData,
        }
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Map for CurvePlayer<S, C> {
    type Input = f64;
    type Output = S;

    fn eval(&self, val: f64) -> S {
        S::from_val(self.curve.eval(val))
    }
}

/// Plays a sample curve at a specified speed, until it reaches the right endpoint.
///
/// See also [`LoopCurveGen`].
#[derive(Clone, Debug)]
pub struct OnceCurveGen<C: Map<Input = f64>>
where
    C::Output: Sample,
{
    /// The sample curve used to generate the samples.
    pub map: C,

    /// The time for which the curve is played.
    pub time: Time,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to play.
    val: f64,
}

impl<C: Map<Input = f64>> OnceCurveGen<C>
where
    C::Output: Sample,
{
    /// Initializes a new [`OnceCurveGen`].
    ///
    /// Note that the `map` argument takes in a sample curve. If you wish to build a
    /// [`OnceCurveGen`] from a plain curve, use [`OnceGen::new`].
    pub const fn new_curve(map: C, time: Time) -> Self {
        Self {
            map,
            time,
            val: 0.0,
        }
    }

    /// A reference to the curve being played.
    pub const fn map(&self) -> &C {
        &self.map
    }

    /// A mutable reference to the curve being played.
    pub fn map_mut(&mut self) -> &mut C {
        &mut self.map
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along the curve we're
    /// currently reading.
    pub const fn val(&self) -> f64 {
        self.val
    }
}

impl<C: Map<Input = f64>> Signal for OnceCurveGen<C>
where
    C::Output: Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.map.eval(self.val)
    }

    fn advance(&mut self) {
        self.val += 1.0 / self.time.frames();
        self.val = self.val.min(1.0);
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

impl<C: Map<Input = f64>> Base for OnceCurveGen<C>
where
    C::Output: Sample,
{
    impl_base!();
}

impl<C: Map<Input = f64>> Done for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn is_done(&self) -> bool {
        self.val >= 1.0
    }
}

impl<C: Map<Input = f64>> Stop for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn stop(&mut self) {
        self.val = 1.0;
    }
}

impl<C: Map<Input = f64>> Panic for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn panic(&mut self) {
        self.stop();
    }
}

/// Plays a given curve by reading its output as values of a given sample type.
pub type OnceGen<S, C> = OnceCurveGen<CurvePlayer<S, C>>;

impl<S: Sample, C: Map<Input = f64, Output = f64>> OnceGen<S, C> {
    /// Initializes a new [`OnceGen`].
    ///
    /// You might need to explicitly specify the type of sample this curve will produce, via
    /// `OnceGen::<S, _>::new`.
    ///
    /// Note that this builds a [`OnceGen`]. In order to build a more general [`OnceCurveGen`], use
    /// `OnceCurveGen::new_curve`.
    pub const fn new(curve: C, time: Time) -> Self {
        Self::new_curve(CurvePlayer::new(curve), time)
    }

    /// A reference to the curve being played.
    pub const fn curve(&self) -> &C {
        &self.map().curve
    }

    /// A mutable reference to the curve being played.
    pub fn curve_mut(&mut self) -> &mut C {
        &mut self.map_mut().curve
    }
}

/// Loops a curve at a specified frequency.
///
/// See also [`OnceCurveGen`].
#[derive(Clone, Debug, Default)]
pub struct LoopCurveGen<C: Map<Input = f64>>
where
    C::Output: Sample,
{
    /// The map used to generate the samples.
    pub map: C,

    /// The frequency at which the curve is played.
    pub freq: Freq,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to play.
    val: f64,
}

impl<C: Map<Input = f64>> LoopCurveGen<C>
where
    C::Output: Sample,
{
    /// Initializes a new [`LoopCurveGen`].
    ///
    /// Note that the `map` argument takes in a sample curve. If you wish to build a
    /// [`LoopCurveGen`] from a plain curve, use [`LoopGen::new`].
    pub const fn new_curve(map: C, freq: Freq) -> Self {
        Self {
            map,
            freq,
            val: 0.0,
        }
    }

    /// Initializes a new [`LoopCurveGen`] with a given phase.
    pub fn new_phase(map: C, freq: Freq, phase: f64) -> Self {
        Self {
            map,
            freq,
            val: phase.rem_euclid(1.0),
        }
    }

    /// A reference to the curve being played.
    pub const fn map(&self) -> &C {
        &self.map
    }

    /// A mutable reference to the curve being played.
    pub fn map_mut(&mut self) -> &mut C {
        &mut self.map
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along the curve we're
    /// currently reading.
    pub fn val(&self) -> f64 {
        self.val
    }
}

impl<C: Map<Input = f64>> Signal for LoopCurveGen<C>
where
    C::Output: Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.map.eval(self.val)
    }

    fn advance(&mut self) {
        self.val += self.freq.hz() / crate::SAMPLE_RATE_F64;
        self.val %= 1.0;
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

impl<C: Map<Input = f64>> Frequency for LoopCurveGen<C>
where
    C::Output: Sample,
{
    fn freq(&self) -> Freq {
        self.freq
    }

    fn freq_mut(&mut self) -> &mut Freq {
        &mut self.freq
    }
}

impl<C: Map<Input = f64>> Base for LoopCurveGen<C>
where
    C::Output: Sample,
{
    impl_base!();
}

/// Loops a given curve by reading its output as values of a given sample type.
pub type LoopGen<S, C> = LoopCurveGen<CurvePlayer<S, C>>;

impl<S: Sample, C: Map<Input = f64, Output = f64>> LoopGen<S, C> {
    /// Initializes a new [`LoopGen`].
    ///
    /// You might need to explicitly specify the type of sample this curve will produce, via
    /// `LoopGen::<S, _>::new`.
    ///
    /// Note that this builds a [`LoopGen`]. In order to build a more general [`LoopCurveGen`], use
    /// `LoopCurveGen::new_curve`.
    pub const fn new(curve: C, freq: Freq) -> Self {
        Self::new_curve(CurvePlayer::new(curve), freq)
    }

    /// A reference to the curve being played.
    pub const fn curve(&self) -> &C {
        &self.map().curve
    }

    /// A mutable reference to the curve being played.
    pub fn curve_mut(&mut self) -> &mut C {
        &mut self.map_mut().curve
    }
}

/// Generates random data.
#[derive(Clone, Copy, Debug)]
pub struct NoiseGen<S: Sample> {
    /// The current random value.
    current: S,
}

impl<S: Sample> Default for NoiseGen<S> {
    fn default() -> Self {
        Self { current: S::rand() }
    }
}

impl<S: Sample> NoiseGen<S> {
    /// Initializes a new [`NoiseGen`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Sample> Signal for NoiseGen<S> {
    type Sample = S;

    fn get(&self) -> Self::Sample {
        self.current
    }

    fn advance(&mut self) {
        self.current = S::rand();
    }

    fn retrigger(&mut self) {
        self.advance();
    }
}

impl<S: Sample> Base for NoiseGen<S> {
    impl_base!();
}
