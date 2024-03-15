//! Implements many effects one can use to modify signals.
//!
//! Effects are structures that wrap around [`Signals`](crate::prelude::Signal) and modify the
//! samples they produce, be they envelope or audio data.
//!
//! The module file implements the most basic structures for transforming a signal, including
//! [`MapSgn`], [`MutSgn`], and [`ModSgn`].

pub mod delay;
pub mod distortion;
pub mod envelopes;
pub mod filter;
mod freq;
mod trailing;
mod vol;

pub use delay as dly;
pub use distortion as dst;
pub use envelopes as env;
pub use filter as flt;
pub mod pan;

pub use freq::{Vib, Vibrato};
pub use trailing::{Stopping, Trailing};
pub use vol::{Gate, StopTremolo, Trem, Tremolo, Volume};

use crate::prelude::*;

/// Maps a signal to another via a specified map.
///
/// This map should send in most cases send the zero [`Sample`] to itself. That ensures that there
/// is no DC offset if the signal stops.
///
/// Note that the map here takes in a sample and outputs a sample. If you instead want to map the
/// floating point values of the sample pointwise, wrap the function in [`map::Pw`].
#[derive(Clone, Copy, Debug, Default)]
pub struct MapSgn<S: Signal, F: map::Map<Input = S::Sample>>
where
    F::Output: smp::Sample,
{
    /// The signal being mapped.
    sgn: S,
    /// The map being applied.
    map: F,
}

impl<S: Signal, F: map::Map<Input = S::Sample>> MapSgn<S, F>
where
    F::Output: smp::Sample,
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

impl<S: Signal, F: map::Map<Input = S::Sample>> Signal for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    type Sample = F::Output;

    fn get(&self) -> F::Output {
        self.map().eval(self.sgn.get())
    }
}

impl<S: SignalMut, F: map::Map<Input = S::Sample>> SignalMut for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    fn advance(&mut self) {
        self.sgn_mut().advance();
    }

    fn retrigger(&mut self) {
        self.sgn_mut().retrigger();
    }
}

impl<S: Frequency, F: map::Map<Input = S::Sample>> Frequency for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    fn freq(&self) -> unt::Freq {
        self.sgn().freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.sgn_mut().freq_mut()
    }
}

impl<S: Base, F: map::Map<Input = S::Sample>> Base for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: Stop, F: map::Map<Input = S::Sample>> Stop for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    fn stop(&mut self) {
        self.sgn_mut().stop();
    }
}

impl<S: Done, F: map::Map<Input = S::Sample>> Done for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    fn is_done(&self) -> bool {
        self.sgn().is_done()
    }
}

impl<S: Panic, F: map::Map<Input = S::Sample>> Panic for MapSgn<S, F>
where
    F::Output: smp::Sample,
{
    fn panic(&mut self) {
        self.sgn_mut().panic();
    }
}

/// A [`MapSgn`] taking in a [`map::Pw`] function.
pub type PwMapSgn<S, F> = MapSgn<S, map::Pw<<S as Signal>::Sample, F>>;

impl<S: Signal, F: map::Map<Input = f64, Output = f64>> PwMapSgn<S, F> {
    /// Initializes a [`MapSgn`] from a pointwise function.
    pub const fn new_pw(sgn: S, map: F) -> Self {
        Self::new(sgn, map::Pw::new(map))
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
///
/// ## Example
///
/// TODO.
#[derive(Clone, Debug)]
pub struct MutSgn<S: Signal, E: Signal<Sample = smp::Env>, F: map::Env<S>> {
    /// The signal to modify.
    sgn: S,
    /// The envelope modifying the signal.
    env: E,
    /// The function to modify the signal.
    func: F,
}

impl<S: Signal, E: Signal<Sample = smp::Env>, F: map::Env<S>> MutSgn<S, E, F> {
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

impl<S: Signal, E: Signal<Sample = smp::Env>, F: map::Env<S>> Signal for MutSgn<S, E, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn().get()
    }
}

impl<S: SignalMut, E: SignalMut<Sample = smp::Env>, F: map::Env<S>> SignalMut for MutSgn<S, E, F> {
    fn advance(&mut self) {
        self.sgn.advance();
        self.func.modify_env(&mut self.sgn, self.env.next());
    }

    fn retrigger(&mut self) {
        self.sgn_mut().retrigger();
        self.env_mut().retrigger();
    }
}

impl<S: Frequency, E: SignalMut<Sample = smp::Env>, F: map::Env<S>> Frequency for MutSgn<S, E, F> {
    fn freq(&self) -> unt::Freq {
        self.sgn().freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.sgn_mut().freq_mut()
    }
}

impl<S: Base, E: SignalMut<Sample = smp::Env>, F: map::Env<S>> Base for MutSgn<S, E, F> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: SignalMut + Done, E: SignalMut<Sample = smp::Env>, F: map::Env<S>> Done
    for MutSgn<S, E, F>
{
    fn is_done(&self) -> bool {
        self.sgn().is_done()
    }
}

impl<S: Stop, E: SignalMut<Sample = smp::Env>, F: map::Env<S>> Stop for MutSgn<S, E, F> {
    fn stop(&mut self) {
        self.sgn_mut().stop();
    }
}

impl<S: Panic, E: SignalMut<Sample = smp::Env>, F: map::Env<S>> Panic for MutSgn<S, E, F> {
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
pub struct ModSgn<S: SignalMut, E: Stop<Sample = smp::Env>, F: map::Env<S>> {
    /// Inner data.
    inner: MutSgn<S, E, F>,
}

/// Turns a [`MutSgn`] into a [`ModSgn`]. This changes the functionality of the signal when stopped,
/// as described in the [`ModSgn`] docs.
impl<S: SignalMut, E: Stop<Sample = smp::Env>, F: map::Env<S>> From<MutSgn<S, E, F>>
    for ModSgn<S, E, F>
{
    fn from(inner: MutSgn<S, E, F>) -> Self {
        Self { inner }
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>, F: map::Env<S>> ModSgn<S, E, F> {
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

impl<S: SignalMut, E: Stop<Sample = smp::Env>, F: map::Env<S>> Signal for ModSgn<S, E, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner._get()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>, F: map::Env<S>> SignalMut for ModSgn<S, E, F> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: Stop<Sample = smp::Env>, F: map::Env<S>> Frequency for ModSgn<S, E, F> {
    fn freq(&self) -> unt::Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: Stop<Sample = smp::Env>, F: map::Env<S>> Base for ModSgn<S, E, F> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env> + Done, F: map::Env<S>> Done for ModSgn<S, E, F> {
    fn is_done(&self) -> bool {
        self.env().is_done()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>, F: map::Env<S>> Stop for ModSgn<S, E, F> {
    fn stop(&mut self) {
        self.env_mut().stop();
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env> + Panic, F: map::Env<S>> Panic for ModSgn<S, E, F> {
    fn panic(&mut self) {
        self.env_mut().panic();
    }
}
