//! TODO: missing docs

use crate::{prelude::*, traits::*};
use std::marker::PhantomData;

/// Controls the volume of a signal.
#[derive(Clone, Debug, Default)]
pub struct Volume<S: Signal> {
    /// Inner data.
    inner: eff::PwMapSgn<S, unt::Vol>,
}

impl<S: Signal> Volume<S> {
    /// Initializes a new signal with a given [`unt::Vol`].
    pub const fn new(sgn: S, vol: unt::Vol) -> Self {
        Self {
            inner: eff::PwMapSgn::new_pw(sgn, vol),
        }
    }

    /// Returns a reference to the signal whose volume is modified.
    pub const fn sgn(&self) -> &S {
        self.inner.sgn()
    }

    /// Returns a mutable reference to the signal whose volume is modified.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut()
    }

    /// Volume of the signal.
    pub const fn vol(&self) -> unt::Vol {
        *self.inner.map_pw()
    }

    /// Returns a mutable reference to the volume of the signal.
    pub fn vol_mut(&mut self) -> &mut unt::Vol {
        self.inner.map_pw_mut()
    }
}

impl<S: Signal> Signal for Volume<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner._get()
    }
}

impl<S: SignalMut> SignalMut for Volume<S> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency> Frequency for Volume<S> {
    fn freq(&self) -> unt::Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base> Base for Volume<S> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: Done> Done for Volume<S> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Stop> Stop for Volume<S> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: Panic> Panic for Volume<S> {
    fn panic(&mut self) {
        self.inner.panic();
    }
}

/// The function that applies tremolo to a volume signal.
#[derive(Clone, Copy, Debug)]
pub struct Trem<S: Signal> {
    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Signal> Trem<S> {
    /// Initializes a new [`Trem`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<S: Signal> Default for Trem<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Signal> map::Env<Volume<S>> for Trem<S> {
    fn modify_env(&mut self, sgn: &mut Volume<S>, gain: smp::Env) {
        sgn.vol_mut().gain = gain.0;
    }
}

/// Applies tremolo to a signal.
///
/// Note that "tremolo" here just means a **change** in volume controlled by an envelope. This is
/// more general than the usual meaning of tremolo, this being **oscillation** in volume. For
/// instance, [`AdsrEnv`] is a special case of [`Tremolo`] (technically [`StopTremolo`]).
///
/// This signal stops whenever the original signal does. If you instead want a signal that stops
/// when the envelope does, use [`StopTremolo`].
#[derive(Clone, Debug)]
pub struct Tremolo<S: Signal, E: Signal<Sample = smp::Env>> {
    /// Inner data.
    inner: eff::MutSgn<Volume<S>, E, Trem<S>>,
}

impl<S: Signal, E: Signal<Sample = smp::Env>> Tremolo<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, env: E) -> Self {
        Self {
            // The volume is unimportant, as it immediately gets rewritten.
            inner: eff::MutSgn::new(Volume::new(sgn, unt::Vol::FULL), env, Trem::new()),
        }
    }

    /// Returns a reference to the signal whose volume is modified.
    pub const fn sgn(&self) -> &S {
        self.inner.sgn().sgn()
    }

    /// Returns a mutable reference to the signal whose volume is modified.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut().sgn_mut()
    }

    /// Returns a reference to the envelope controlling the volume.
    pub const fn env(&self) -> &E {
        self.inner.env()
    }

    /// Returns a mutable reference to the envelope controlling the volume.
    pub fn env_mut(&mut self) -> &mut E {
        self.inner.env_mut()
    }
}

impl<S: Signal, E: Signal<Sample = smp::Env>> Signal for Tremolo<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner._get()
    }
}

impl<S: SignalMut, E: SignalMut<Sample = smp::Env>> SignalMut for Tremolo<S, E> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: SignalMut<Sample = smp::Env>> Frequency for Tremolo<S, E> {
    fn freq(&self) -> unt::Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: SignalMut<Sample = smp::Env>> Base for Tremolo<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: SignalMut + Done, E: SignalMut<Sample = smp::Env>> Done for Tremolo<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Stop, E: SignalMut<Sample = smp::Env>> Stop for Tremolo<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: Panic, E: SignalMut<Sample = smp::Env>> Panic for Tremolo<S, E> {
    fn panic(&mut self) {
        self.inner.panic();
    }
}

/// Applies tremolo (change in volume) to a signal according to an envelope. In contrast to
/// [`Tremolo`], which [`Stops`](Stop) when the original signal does, this signal [`Stops`](Stop)
/// when the envelope does.
///
/// The signal is otherwise unchanged, so if you don't need this functionality, use [`Tremolo`]
/// instead.
#[derive(Clone, Debug)]
pub struct StopTremolo<S: SignalMut, E: Stop<Sample = smp::Env>> {
    /// Inner data.
    inner: eff::ModSgn<Volume<S>, E, Trem<S>>,
}

/// Turns a [`Tremolo`] into a [`StopTremolo`]. This changes the functionality of the signal when
/// stopped, as described in the [`StopTremolo`] docs.
impl<S: SignalMut, E: Stop<Sample = smp::Env>> From<Tremolo<S, E>> for StopTremolo<S, E> {
    fn from(value: Tremolo<S, E>) -> Self {
        Self {
            inner: value.inner.into(),
        }
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>> StopTremolo<S, E> {
    /// Initializes a new [`StopTremolo`].
    pub fn new(sgn: S, env: E) -> Self {
        Tremolo::new(sgn, env).into()
    }

    /// Returns a reference to the signal whose volume is modified.
    pub const fn sgn(&self) -> &S {
        self.inner.sgn().sgn()
    }

    /// Returns a mutable reference to the signal whose volume is modified.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut().sgn_mut()
    }

    /// Returns a reference to the envelope controlling the volume.
    pub const fn env(&self) -> &E {
        self.inner.env()
    }

    /// Returns a mutable reference to the envelope controlling the volume.
    pub fn env_mut(&mut self) -> &mut E {
        self.inner.env_mut()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>> Signal for StopTremolo<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner._get()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>> SignalMut for StopTremolo<S, E> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: Stop<Sample = smp::Env>> Frequency for StopTremolo<S, E> {
    fn freq(&self) -> unt::Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: Stop<Sample = smp::Env>> Base for StopTremolo<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env> + Done> Done for StopTremolo<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env>> Stop for StopTremolo<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: SignalMut, E: Stop<Sample = smp::Env> + Panic> Panic for StopTremolo<S, E> {
    fn panic(&mut self) {
        self.inner.panic();
    }
}

/// Gates a signal through an envelope.
///
/// Output will only come through when the envelope is above the threshold.
#[derive(Clone, Debug)]
pub struct Gate<S: Signal, E: Signal<Sample = smp::Env>> {
    /// The gated signal.
    sgn: S,

    /// The envelope for the gating.
    env: E,

    /// The threshold for the gate.
    threshold: f64,
}

impl<S: Signal, E: Signal<Sample = smp::Env>> Gate<S, E> {
    /// Initializes a new gate.
    pub const fn new(sgn: S, env: E, threshold: f64) -> Self {
        Self {
            sgn,
            env,
            threshold,
        }
    }

    /// Returns a reference to the signal.
    pub const fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// Returns a reference to the envelope.
    pub const fn env(&self) -> &E {
        &self.env
    }

    /// Returns a mutable reference to the envelope.
    pub fn env_mut(&mut self) -> &mut E {
        &mut self.env
    }

    /// Returns the threshold.
    pub const fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Returns a mutable reference to the threshold.
    pub fn threshold_mut(&mut self) -> &mut f64 {
        &mut self.threshold
    }
}

impl<S: Signal, E: Signal<Sample = smp::Env>> Signal for Gate<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        if self.env.get().0 >= self.threshold {
            self.sgn.get()
        } else {
            smp::Base::ZERO
        }
    }
}

impl<S: SignalMut, E: SignalMut<Sample = smp::Env>> SignalMut for Gate<S, E> {
    fn advance(&mut self) {
        self.sgn.advance();
        self.env.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.env.retrigger();
    }
}

impl<S: Frequency, E: SignalMut<Sample = smp::Env>> Frequency for Gate<S, E> {
    fn freq(&self) -> unt::Freq {
        self.sgn.freq()
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        self.sgn.freq_mut()
    }
}

impl<S: Base, E: SignalMut<Sample = smp::Env>> Base for Gate<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: Done, E: SignalMut<Sample = smp::Env>> Done for Gate<S, E> {
    fn is_done(&self) -> bool {
        self.sgn().is_done()
    }
}

impl<S: Stop, E: SignalMut<Sample = smp::Env>> Stop for Gate<S, E> {
    fn stop(&mut self) {
        self.sgn_mut().stop();
    }
}

impl<S: Panic, E: SignalMut<Sample = smp::Env>> Panic for Gate<S, E> {
    fn panic(&mut self) {
        self.sgn_mut().panic();
    }
}

// Todo: fade-in/fade-out
