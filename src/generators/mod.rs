//! Implements generators of all kinds.
//!
//! Generators are structures that generate [`Signals`](crate::prelude::Signal) on their own, be
//! they envelope or audio data.
//!
//! The module file provides the most basic examples of generators, namely generators that read data
//! from a curve. See the [curves module docs](../curves/index.html#terminology) for an explanation
//! on different kinds of curves.

use std::{fmt::Display, marker::PhantomData};

use rand::{distributions::Standard, prelude::Distribution};

pub use crate::prelude::*;

pub mod poly;
pub mod sequence;
pub mod unison;

/// A value between `0.0` and `1.0` showing how far along a curve we are.
///
/// The methods of this type abstract away some basic logic used in other structs such as
/// [`LoopCurveGen`] and [`OnceCurveGen`].
///
/// This is also the input type for a map. This gives us a guarantee that the values are in this
/// specific range.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Val(f64);

impl Val {
    /// The zero value.
    pub const ZERO: Self = Val(0.0);

    /// The one value.
    pub const ONE: Self = Val(1.0);

    /// Initializes a [`Val`].
    ///
    /// The inner value must be between `0.0` and `1.0`.
    ///
    /// ## Panics
    ///
    /// Panics if the passed value isn't between `0.0` and `1.0`.
    #[must_use]
    pub fn new(val: f64) -> Self {
        assert!((0.0..=1.0).contains(&val));
        Self(val)
    }

    /// Initializes a [`Val`] without checking the range conditions.
    ///
    /// ## Safety
    ///
    /// This method is safe. However, it may lead to panicking and other unexpected behavior.
    #[must_use]
    pub const fn new_unchecked(val: f64) -> Self {
        Self(val)
    }

    /// Returns the inner value.
    #[must_use]
    pub const fn val(&self) -> f64 {
        self.0
    }

    /// Resets the value to `0.0`.
    pub fn retrigger(&mut self) {
        self.0 = 0.0;
    }

    /// Whether the value is equal to `1.0`.
    #[must_use]
    pub fn is_done(&self) -> bool {
        // We avoid clippy complaints by using `>=`.
        self.0 >= 1.0
    }

    /// Sets the value to `1.0`.
    pub fn stop(&mut self) {
        self.0 = 1.0;
    }

    /// Advances the inner value in order to play a wave with the specified frequency.
    pub fn advance_freq(&mut self, freq: RawFreq) {
        self.0 = (self.0 + freq.hz() / SAMPLE_RATE_F64) % 1.0;
    }

    /// Advances the inner value in order to play a wave for the specified duration.
    pub fn advance_time(&mut self, time: Time) {
        self.0 += 1.0 / time.frames();
        self.0 = self.0.min(1.0);
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Distribution<Val> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Val {
        Val(rng.gen())
    }
}

/// Converts a plain curve into a sample curve that outputs a signal of the specified type.
#[derive(Clone, Copy, Debug, Default)]
pub struct CurvePlayer<S: Sample, C: Map<Input = Val, Output = f64>> {
    /// The inner plain curve.
    pub curve: C,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Sample, C: Map<Input = Val, Output = f64>> CurvePlayer<S, C> {
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

impl<S: Sample, C: Map<Input = Val, Output = f64>> Map for CurvePlayer<S, C> {
    type Input = Val;
    type Output = S;

    fn eval(&self, val: Val) -> S {
        S::from_val(self.curve.eval(val))
    }
}

/// Plays a sample curve at a specified speed, until it reaches the right endpoint.
///
/// Initialize with [`Self::new_curve`].
///
/// See also [`LoopCurveGen`].
#[derive(Clone, Debug)]
pub struct OnceCurveGen<C: Map<Input = Val>>
where
    C::Output: Sample,
{
    /// The curve we're playing.
    map: C,

    /// How far along the curve we are.
    val: Val,

    /// The time for which the curve is played.
    time: Time,
}

impl<C: Map<Input = Val>> OnceCurveGen<C>
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
            val: Val::ZERO,
            time,
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

    /// Returns how far along the curve we are.
    pub const fn val(&self) -> Val {
        self.val
    }

    /// The time for which the curve is played.
    pub const fn time(&self) -> Time {
        self.time
    }

    /// A mutable reference to the time for which this curve is played.
    pub fn time_mut(&mut self) -> &mut Time {
        &mut self.time
    }
}

impl<C: Map<Input = Val>> Signal for OnceCurveGen<C>
where
    C::Output: Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.map.eval(self.val)
    }
}

impl<C: Map<Input = Val>> SignalMut for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn advance(&mut self) {
        self.val.advance_time(self.time());
    }

    fn retrigger(&mut self) {
        self.val.retrigger();
    }
}

impl<C: Map<Input = Val>> Base for OnceCurveGen<C>
where
    C::Output: Sample,
{
    impl_base!();
}

impl<C: Map<Input = Val>> Done for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn is_done(&self) -> bool {
        self.val().is_done()
    }
}

impl<C: Map<Input = Val>> Stop for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn stop(&mut self) {
        self.val.stop();
    }
}

impl<C: Map<Input = Val>> Panic for OnceCurveGen<C>
where
    C::Output: Sample,
{
    fn panic(&mut self) {
        self.stop();
    }
}

/// Plays a given curve by reading its output as values of a given sample type.
pub type OnceGen<S, C> = OnceCurveGen<CurvePlayer<S, C>>;

impl<S: Sample, C: Map<Input = Val, Output = f64>> OnceGen<S, C> {
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
/// Initialize with [`Self::new_curve`].
///
/// See also [`OnceCurveGen`].
#[derive(Clone, Debug, Default)]
pub struct LoopCurveGen<C: Map<Input = Val>>
where
    C::Output: Sample,
{
    /// The curve being played.
    map: C,

    /// How far along the curve we are.
    val: Val,

    /// The frequency at which the curve is played.
    freq: RawFreq,
}

impl<C: Map<Input = Val>> LoopCurveGen<C>
where
    C::Output: Sample,
{
    /// Initializes a new [`LoopCurveGen`] with a given phase.
    pub const fn new_curve_phase(map: C, freq: RawFreq, phase: Val) -> Self {
        Self {
            map,
            freq,
            val: phase,
        }
    }

    /// Initializes a new [`LoopCurveGen`].
    ///
    /// Note that the `map` argument takes in a sample curve. If you wish to build a
    /// [`LoopCurveGen`] from a plain curve, use [`LoopGen::new`].
    pub const fn new_curve(map: C, freq: RawFreq) -> Self {
        Self::new_curve_phase(map, freq, Val::ZERO)
    }

    /// A reference to the curve being played.
    pub const fn map(&self) -> &C {
        &self.map
    }

    /// A mutable reference to the curve being played.
    pub fn map_mut(&mut self) -> &mut C {
        &mut self.map
    }

    /// Returns how far along the curve we are.
    pub const fn val(&self) -> Val {
        self.val
    }

    /// Returns a mutable reference to how far along the curve we are.
    pub fn val_mut(&mut self) -> &mut Val {
        &mut self.val
    }

    /// The frequency at which the curve is played.
    pub const fn freq(&self) -> RawFreq {
        self.freq
    }

    /// A mutable reference to the frequency at which this curve is played.
    pub fn time_mut(&mut self) -> &mut RawFreq {
        &mut self.freq
    }
}

impl<C: Map<Input = Val>> Signal for LoopCurveGen<C>
where
    C::Output: Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.map.eval(self.val())
    }
}

impl<C: Map<Input = Val>> SignalMut for LoopCurveGen<C>
where
    C::Output: Sample,
{
    fn advance(&mut self) {
        self.val.advance_freq(self.freq());
    }

    fn retrigger(&mut self) {
        self.val.retrigger();
    }
}

impl<C: Map<Input = Val>> Frequency for LoopCurveGen<C>
where
    C::Output: Sample,
{
    fn freq(&self) -> RawFreq {
        self.freq
    }

    fn freq_mut(&mut self) -> &mut RawFreq {
        &mut self.freq
    }
}

impl<C: Map<Input = Val>> Base for LoopCurveGen<C>
where
    C::Output: Sample,
{
    impl_base!();
}

/// Loops a given curve by reading its output as values of a given sample type.
pub type LoopGen<S, C> = LoopCurveGen<CurvePlayer<S, C>>;

impl<S: Sample, C: Map<Input = Val, Output = f64>> LoopGen<S, C> {
    /// Initializes a new [`LoopGen`] with a given phase.
    pub const fn new_phase(curve: C, freq: RawFreq, phase: Val) -> Self {
        Self::new_curve_phase(CurvePlayer::new(curve), freq, phase)
    }

    /// Initializes a new [`LoopGen`].
    ///
    /// You might need to explicitly specify the type of sample this curve will produce, via
    /// `LoopGen::<S, _>::new`.
    ///
    /// Note that this builds a [`LoopGen`]. In order to build a more general [`LoopCurveGen`], use
    /// `LoopCurveGen::new_curve`.
    pub const fn new(curve: C, freq: RawFreq) -> Self {
        Self::new_phase(curve, freq, Val::ZERO)
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
}

impl<S: Sample> SignalMut for NoiseGen<S> {
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
