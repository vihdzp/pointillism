//! Implements many effects one can use to modify signals.
//!
//! Effects are structures that wrap around [`Signals`](crate::prelude::Signal) and modify the
//! samples they produce, be they envelope or audio data.
//!
//! The module file implements the most basic structures for transforming a signal, including
//! [`MapSgn`], [`MutSgn`], and [`ModSgn`].

pub mod adsr;
pub mod delay;
pub mod distortion;
pub mod filter;
pub mod freq;
pub mod mix;
pub mod pan;
pub mod trailing;
pub mod vol;

use std::marker::PhantomData;

use crate::prelude::*;

/// Converts a function into one applied pointwise to the entries of a [`Sample`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Pw<S: Sample, F: Map<Input = f64, Output = f64>> {
    /// The function to apply.
    pub func: F,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Sample, F: Map<Input = f64, Output = f64>> Pw<S, F> {
    /// Initializes a new [`Pw`] function.
    pub const fn new(func: F) -> Self {
        Self {
            func,
            phantom: PhantomData,
        }
    }
}

impl<S: Sample, F: Map<Input = f64, Output = f64>> Map for Pw<S, F> {
    type Input = S;
    type Output = S;

    fn eval(&self, x: S) -> S {
        x.map(|y| self.func.eval(*y))
    }
}

/// Maps a signal to another via a specified map.
///
/// This map should send in most cases send the zero [`Sample`] to itself. That ensures that there
/// is no DC offset if the signal stops.
///
/// Note that the map here takes in a sample and outputs a sample. If you instead want to map the
/// floating point values of the sample pointwise, wrap the function in [`Pw`].
#[derive(Clone, Copy, Debug, Default)]
pub struct MapSgn<S: Signal, F: Map<Input = S::Sample>>
where
    F::Output: Sample,
{
    /// The signal being mapped.
    sgn: S,
    /// The map being applied.
    map: F,
}

impl<S: Signal, F: Map<Input = S::Sample>> MapSgn<S, F>
where
    F::Output: Sample,
{
    /// Initializes a generic [`MapSgn`].
    ///
    /// If the signal implements the [`Stop`] or [`Panic`] trait, the function must map zero to
    /// itself, as otherwise this will create a DC offset.
    pub const fn new(sgn: S, map: F) -> Self {
        Self { sgn, map }
    }

    /// Initializes a [`MapSgn`] using the default constructor for the function.
    pub fn new_default(sgn: S) -> Self
    where
        F: Default,
    {
        Self::new(sgn, F::default())
    }

    /// Returns a reference to the original signal.
    pub const fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the original signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// Returns a reference to the map modifying the signal.
    pub const fn map(&self) -> &F {
        &self.map
    }

    /// Returns a mutable reference to the map modifying the signal.
    pub fn map_mut(&mut self) -> &mut F {
        &mut self.map
    }
}

impl<S: Signal, F: Map<Input = S::Sample>> Signal for MapSgn<S, F>
where
    F::Output: Sample,
{
    type Sample = F::Output;

    fn get(&self) -> F::Output {
        self.map().eval(self.sgn.get())
    }
}

impl<S: SignalMut, F: Map<Input = S::Sample>> SignalMut for MapSgn<S, F>
where
    F::Output: Sample,
{
    fn advance(&mut self) {
        self.sgn_mut().advance();
    }

    fn retrigger(&mut self) {
        self.sgn_mut().retrigger();
    }
}

impl<S: Frequency, F: Map<Input = S::Sample>> Frequency for MapSgn<S, F>
where
    F::Output: Sample,
{
    fn freq(&self) -> Freq {
        self.sgn().freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn_mut().freq_mut()
    }
}

impl<S: Base, F: Map<Input = S::Sample>> Base for MapSgn<S, F>
where
    F::Output: Sample,
{
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: Stop, F: Map<Input = S::Sample>> Stop for MapSgn<S, F>
where
    F::Output: Sample,
{
    fn stop(&mut self) {
        self.sgn_mut().stop();
    }
}

impl<S: Done, F: Map<Input = S::Sample>> Done for MapSgn<S, F>
where
    F::Output: Sample,
{
    fn is_done(&self) -> bool {
        self.sgn().is_done()
    }
}

impl<S: Panic, F: Map<Input = S::Sample>> Panic for MapSgn<S, F>
where
    F::Output: Sample,
{
    fn panic(&mut self) {
        self.sgn_mut().panic();
    }
}

/// A [`MapSgn`] taking in a [`Pw`] function.
pub type PwMapSgn<S, F> = MapSgn<S, Pw<<S as Signal>::Sample, F>>;

impl<S: Signal, F: Map<Input = f64, Output = f64>> PwMapSgn<S, F> {
    /// Initializes a [`MapSgn`] from a pointwise function.
    pub const fn new_pw(sgn: S, map: F) -> Self {
        Self::new(sgn, Pw::new(map))
    }

    /// Returns a reference to the pointwise map modifying the signal.
    pub const fn map_pw(&self) -> &F {
        &self.map().func
    }

    /// Returns a mutable reference to the pointwise map modifying the signal.
    pub fn map_pw_mut(&mut self) -> &mut F {
        &mut self.map_mut().func
    }
}

/// Modifies a signal according to a given envelope.
///
/// This signal stops whenever the original signal does. If you instead want a signal that stops
/// when the envelope does, use [`ModSgn`].
#[derive(Clone, Debug)]
pub struct MutSgn<S: Signal, E: Signal<Sample = Env>, F: MutEnv<S>> {
    /// The signal to modify.
    sgn: S,
    /// The envelope modifying the signal.
    env: E,
    /// The function to modify the signal.
    func: F,
}

impl<S: Signal, E: Signal<Sample = Env>, F: MutEnv<S>> MutSgn<S, E, F> {
    /// Initializes a new [`MutSgn`].
    ///
    /// The state of the signal will immediately get updated according to the function and the first
    /// value of the envelope.
    pub fn new(mut sgn: S, env: E, mut func: F) -> Self {
        func.modify_env(&mut sgn, env.get());
        Self { sgn, env, func }
    }

    /// Returns a reference to the original signal.
    pub const fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the original signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// Returns a reference to the envelope controlling the signal.
    pub const fn env(&self) -> &E {
        &self.env
    }

    /// Returns a mutable reference to the envelope controlling the signal.
    pub fn env_mut(&mut self) -> &mut E {
        &mut self.env
    }

    /// Returns a reference to the function modifying the signal.
    pub const fn func(&self) -> &F {
        &self.func
    }

    /// Returns a mutable reference to the function modifying the signal.
    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
    }
}

impl<S: Signal, E: Signal<Sample = Env>, F: MutEnv<S>> Signal for MutSgn<S, E, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn().get()
    }
}

impl<S: SignalMut, E: SignalMut<Sample = Env>, F: MutEnv<S>> SignalMut for MutSgn<S, E, F> {
    fn advance(&mut self) {
        self.sgn.advance();
        self.func.modify_env(&mut self.sgn, self.env.next());
    }

    fn retrigger(&mut self) {
        self.sgn_mut().retrigger();
        self.env_mut().retrigger();
    }
}

impl<S: Frequency, E: SignalMut<Sample = Env>, F: MutEnv<S>> Frequency for MutSgn<S, E, F> {
    fn freq(&self) -> Freq {
        self.sgn().freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn_mut().freq_mut()
    }
}

impl<S: Base, E: SignalMut<Sample = Env>, F: MutEnv<S>> Base for MutSgn<S, E, F> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: SignalMut + Done, E: SignalMut<Sample = Env>, F: MutEnv<S>> Done for MutSgn<S, E, F> {
    fn is_done(&self) -> bool {
        self.sgn().is_done()
    }
}

impl<S: Stop, E: SignalMut<Sample = Env>, F: MutEnv<S>> Stop for MutSgn<S, E, F> {
    fn stop(&mut self) {
        self.sgn_mut().stop();
    }
}

impl<S: Panic, E: SignalMut<Sample = Env>, F: MutEnv<S>> Panic for MutSgn<S, E, F> {
    fn panic(&mut self) {
        self.sgn_mut().panic();
    }
}

/// Modifies a signal according to a given envelope. In contrast to [`MutSgn`], which
/// [`Stops`](Stop) when the original signal does, this signal [`Stops`](Stop) when the envelope
/// does.
///
/// The signal is otherwise unchanged, so if you don't need this functionality, use [`MutSgn`]
/// instead.
#[derive(Clone, Debug)]
pub struct ModSgn<S: SignalMut, E: Stop<Sample = Env>, F: MutEnv<S>> {
    /// Inner data.
    inner: MutSgn<S, E, F>,
}

/// Turns a [`MutSgn`] into a [`ModSgn`]. This changes the functionality of the signal when stopped,
/// as described in the [`ModSgn`] docs.
impl<S: SignalMut, E: Stop<Sample = Env>, F: MutEnv<S>> From<MutSgn<S, E, F>> for ModSgn<S, E, F> {
    fn from(inner: MutSgn<S, E, F>) -> Self {
        Self { inner }
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>, F: MutEnv<S>> ModSgn<S, E, F> {
    /// Initializes a new [`ModSgn`].
    ///
    /// The state of the signal will immediately get updated according to the function and the first
    /// value of the envelope.
    pub fn new(sgn: S, env: E, func: F) -> Self {
        MutSgn::new(sgn, env, func).into()
    }

    /// Returns a reference to the original signal.
    pub const fn sgn(&self) -> &S {
        self.inner.sgn()
    }

    /// Returns a mutable reference to the original signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut()
    }

    /// Returns a reference to the envelope controlling the signal.
    pub const fn env(&self) -> &E {
        self.inner.env()
    }

    /// Returns a mutable reference to the envelope controlling the signal.
    pub fn env_mut(&mut self) -> &mut E {
        self.inner.env_mut()
    }

    /// Returns a reference to the function modifying the signal.
    pub const fn func(&self) -> &F {
        self.inner.func()
    }

    /// Returns a mutable reference to the function modifying the signal.
    pub fn func_mut(&mut self) -> &mut F {
        self.inner.func_mut()
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>, F: MutEnv<S>> Signal for ModSgn<S, E, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        // We need to call it like this or `rust-analyzer` gets tripped up.
        Signal::get(&self.inner)
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>, F: MutEnv<S>> SignalMut for ModSgn<S, E, F> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: Stop<Sample = Env>, F: MutEnv<S>> Frequency for ModSgn<S, E, F> {
    fn freq(&self) -> Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: Stop<Sample = Env>, F: MutEnv<S>> Base for ModSgn<S, E, F> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: SignalMut, E: Stop<Sample = Env> + Done, F: MutEnv<S>> Done for ModSgn<S, E, F> {
    fn is_done(&self) -> bool {
        self.env().is_done()
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>, F: MutEnv<S>> Stop for ModSgn<S, E, F> {
    fn stop(&mut self) {
        self.env_mut().stop();
    }
}

impl<S: SignalMut, E: Stop<Sample = Env> + Panic, F: MutEnv<S>> Panic for ModSgn<S, E, F> {
    fn panic(&mut self) {
        self.env_mut().panic();
    }
}
