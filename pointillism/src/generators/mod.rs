//! Implements generators of all kinds.
//!
//! Generators are structures that generate [`Signals`](crate::prelude::Signal) on their own, be
//! they envelope or audio data.
//!
//! The module file provides the most basic examples of generators, namely generators that read data
//! from a curve. See the [curves module docs](../curves/index.html#terminology) for an explanation
//! on different kinds of curves.

use crate::prelude::*;
use std::marker::PhantomData;

mod buffer;
pub mod poly;
pub mod unison;

pub use buffer::{Chunks, LoopBuf, OnceBuf};
pub use poly::Polyphony;
pub use unison as uni;

/// Converts a plain curve into a sample curve that outputs a signal of the specified type.
#[derive(Clone, Copy, Debug, Default)]
pub struct CurvePlayer<S: smp::Sample, C: Map<Input = unt::Val, Output = f64>> {
    /// The inner plain curve.
    pub curve: C,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: smp::Sample, C: Map<Input = unt::Val, Output = f64>> CurvePlayer<S, C> {
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

impl<S: smp::Sample, C: Map<Input = unt::Val, Output = f64>> Map for CurvePlayer<S, C> {
    type Input = unt::Val;
    type Output = S;

    fn eval(&self, val: unt::Val) -> S {
        S::from_val(self.curve.eval(val))
    }
}

/// Plays a sample curve for a specified [`unt::Time`].
///
/// Initialize with [`Self::new_curve`].
///
/// See also [`gen::LoopCurve`].
#[derive(Clone, Debug)]
pub struct OnceCurve<C: Map<Input = unt::Val>>
where
    C::Output: smp::Sample,
{
    /// The curve we're playing.
    map: C,
    /// How long has the curve played for?
    elapsed: unt::Time,
    /// The time for which the curve is to be played.
    time: unt::Time,
}

impl<C: Map<Input = unt::Val>> OnceCurve<C>
where
    C::Output: smp::Sample,
{
    /// Initializes a new [`gen::OnceCurve`].
    ///
    /// Note that the `map` argument takes in a sample curve. If you wish to build a
    /// [`gen::OnceCurve`] from a plain curve, use [`gen::Once::new`].
    pub const fn new_curve(map: C, time: unt::Time) -> Self {
        Self {
            map,
            elapsed: unt::Time::ZERO,
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

    /// Returns how long the curve has been played for.
    pub const fn elapsed(&self) -> unt::Time {
        self.elapsed
    }

    /// The time for which the curve is to be played.
    pub const fn time(&self) -> unt::Time {
        self.time
    }

    /// How far along the curve are we?
    pub fn val(&self) -> unt::Val {
        unt::Val::new(self.elapsed / self.time)
    }
}

impl<C: Map<Input = unt::Val>> Signal for OnceCurve<C>
where
    C::Output: smp::Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.map.eval(self.val())
    }
}

impl<C: Map<Input = unt::Val>> SignalMut for OnceCurve<C>
where
    C::Output: smp::Sample,
{
    fn advance(&mut self) {
        self.elapsed.advance();
        if self.elapsed > self.time {
            self.elapsed = self.time;
        }
    }

    fn retrigger(&mut self) {
        self.elapsed = unt::Time::ZERO;
    }
}

impl<C: Map<Input = unt::Val>> Base for OnceCurve<C>
where
    C::Output: smp::Sample,
{
    impl_base!();
}

impl<C: Map<Input = unt::Val>> Done for OnceCurve<C>
where
    C::Output: smp::Sample,
{
    fn is_done(&self) -> bool {
        self.elapsed == self.time
    }
}

impl<C: Map<Input = unt::Val>> Stop for OnceCurve<C>
where
    C::Output: smp::Sample,
{
    fn stop(&mut self) {
        self.elapsed = self.time;
    }
}

impl<C: Map<Input = unt::Val>> Panic for OnceCurve<C>
where
    C::Output: smp::Sample,
{
    fn panic(&mut self) {
        self.stop();
    }
}

/// Plays a given curve by reading its output as values of a given sample type.
pub type Once<S, C> = OnceCurve<CurvePlayer<S, C>>;

impl<S: smp::Sample, C: Map<Input = unt::Val, Output = f64>> gen::Once<S, C> {
    /// Initializes a new [`gen::Once`].
    ///
    /// You might need to explicitly specify the type of sample this curve will produce, via
    /// `gen::Once::<S, _>::new`.
    ///
    /// Note that this builds a [`gen::Once`]. In order to build a more general [`gen::OnceCurve`],
    /// use [`gen::OnceCurve::new_curve`].
    pub const fn new(curve: C, time: unt::Time) -> Self {
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
/// See also [`gen::OnceCurve`].
#[derive(Clone, Debug, Default)]
pub struct LoopCurve<C: Map<Input = unt::Val>>
where
    C::Output: smp::Sample,
{
    /// The curve being played.
    map: C,
    /// How far along the curve we are?
    val: unt::Val,
    /// The frequency at which the curve is played.
    freq: unt::Freq,
}

impl<C: Map<Input = unt::Val>> LoopCurve<C>
where
    C::Output: smp::Sample,
{
    /// Initializes a new [`gen::LoopCurve`] with a given phase.
    pub const fn new_curve_phase(map: C, freq: unt::Freq, phase: unt::Val) -> Self {
        Self {
            map,
            freq,
            val: phase,
        }
    }

    /// Initializes a new [`gen::LoopCurve`].
    ///
    /// Note that the `map` argument takes in a sample curve. If you wish to build a
    /// [`gen::LoopCurve`] from a plain curve, use [`gen::Loop::new`].
    pub const fn new_curve(map: C, freq: unt::Freq) -> Self {
        Self::new_curve_phase(map, freq, unt::Val::ZERO)
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
    pub const fn val(&self) -> unt::Val {
        self.val
    }

    /// Returns a mutable reference to how far along the curve we are.
    pub fn val_mut(&mut self) -> &mut unt::Val {
        &mut self.val
    }

    /// The frequency at which the curve is played.
    pub const fn freq(&self) -> unt::Freq {
        self.freq
    }

    /// A mutable reference to the frequency at which this curve is played.
    pub fn freq_mut(&mut self) -> &mut unt::Freq {
        &mut self.freq
    }
}

impl<C: Map<Input = unt::Val>> Signal for LoopCurve<C>
where
    C::Output: smp::Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.map.eval(self.val())
    }
}

impl<C: Map<Input = unt::Val>> SignalMut for LoopCurve<C>
where
    C::Output: smp::Sample,
{
    fn advance(&mut self) {
        self.val.advance_freq(self.freq());
    }

    fn retrigger(&mut self) {
        self.val = unt::Val::ZERO;
    }
}

impl<C: Map<Input = unt::Val>> Frequency for LoopCurve<C>
where
    C::Output: smp::Sample,
{
    fn freq(&self) -> unt::Freq {
        self.freq
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        &mut self.freq
    }
}

impl<C: Map<Input = unt::Val>> Base for LoopCurve<C>
where
    C::Output: smp::Sample,
{
    impl_base!();
}

/// Loops a given curve by reading its output as values of a given sample type.
pub type Loop<S, C> = LoopCurve<CurvePlayer<S, C>>;

impl<S: smp::Sample, C: Map<Input = unt::Val, Output = f64>> gen::Loop<S, C> {
    /// Initializes a new [`gen::Loop`] with a given phase.
    pub const fn new_phase(curve: C, freq: unt::Freq, phase: unt::Val) -> Self {
        Self::new_curve_phase(CurvePlayer::new(curve), freq, phase)
    }

    /// Initializes a new [`gen::Loop`] with a random phase.
    pub fn new_rand_phase(curve: C, freq: unt::Freq) -> Self {
        use rand::Rng;
        Self::new_phase(curve, freq, rand::thread_rng().gen())
    }

    /// Initializes a new [`gen::Loop`].
    ///
    /// You might need to explicitly specify the type of sample this curve will produce, via
    /// `gen::Loop::<S, _>::new`.
    ///
    /// Note that this builds a [`gen::Loop`]. In order to build a more general [`gen::LoopCurve`],
    /// use `gen::LoopCurve::new_curve`.
    pub const fn new(curve: C, freq: unt::Freq) -> Self {
        Self::new_phase(curve, freq, unt::Val::ZERO)
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
pub struct NoiseGen<S: smp::Sample> {
    /// The current random value.
    current: S,
}

impl<S: smp::Sample> Default for NoiseGen<S> {
    fn default() -> Self {
        Self { current: S::rand() }
    }
}

impl<S: smp::Sample> NoiseGen<S> {
    /// Initializes a new [`NoiseGen`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: smp::Sample> Signal for NoiseGen<S> {
    type Sample = S;

    fn get(&self) -> Self::Sample {
        self.current
    }
}

impl<S: smp::Sample> SignalMut for NoiseGen<S> {
    fn advance(&mut self) {
        self.retrigger();
    }

    fn retrigger(&mut self) {
        self.current = S::rand();
    }
}

impl<S: smp::Sample> Base for NoiseGen<S> {
    impl_base!();
}

/// Turns a function into a generator.
///
/// Note that retriggering this signal won't do anything.
pub struct Func<A: Audio, F: FnMut() -> A> {
    /// The function generating the samples.
    func: F,
    /// The last output.
    val: A,
}

impl<A: Audio, F: FnMut() -> A> Func<A, F> {
    /// Initializes a new signal from a function.
    pub fn new(mut func: F) -> Self {
        let val = (func)();
        Self { func, val }
    }
}

impl<A: Audio, F: FnMut() -> A> Signal for Func<A, F> {
    type Sample = A;

    fn get(&self) -> Self::Sample {
        self.val
    }
}

impl<A: Audio, F: FnMut() -> A> SignalMut for Func<A, F> {
    fn advance(&mut self) {
        self.val = (self.func)();
    }

    fn retrigger(&mut self) {
        // No-op.
    }
}

/// Turns a function, which takes in a time stamp, into a generator.
///
/// Note that retriggering this signal won't do anything.
pub struct TimeFunc<A: Audio, F: FnMut(unt::Time) -> A> {
    /// The function generating the samples.
    func: F,
    /// The last output.
    val: A,
    /// Elapsed time.
    time: unt::Time,
}

impl<A: Audio, F: FnMut(unt::Time) -> A> TimeFunc<A, F> {
    /// Initializes a new signal from a function.
    pub fn new(mut func: F) -> Self {
        let val = func(unt::Time::ZERO);
        Self {
            func,
            val,
            time: unt::Time::ZERO,
        }
    }
}

impl<A: Audio, F: FnMut(unt::Time) -> A> Signal for TimeFunc<A, F> {
    type Sample = A;

    fn get(&self) -> Self::Sample {
        self.val
    }
}

impl<A: Audio, F: FnMut(unt::Time) -> A> SignalMut for TimeFunc<A, F> {
    fn advance(&mut self) {
        self.time.advance();
        self.val = (self.func)(self.time);
    }

    fn retrigger(&mut self) {
        // No-op.
    }
}
