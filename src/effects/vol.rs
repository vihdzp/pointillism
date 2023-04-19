//! Structures for changing the volume of an audio signal.

use std::marker::PhantomData;

use crate::prelude::*;

/// Represents the gain of some signal.
///
/// This also implements the [`Map`] trait, thus doubling as a function that
/// multiplies the volume of a signal.
#[derive(Clone, Copy, Debug)]
pub struct Vol {
    /// Gain factor.
    pub gain: f64,
}

impl Vol {
    /// Initializes a new volume variable.
    #[must_use]
    pub const fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Silence.
    pub const ZERO: Self = Self::new(0.0);

    /// Full volume.
    pub const FULL: Self = Self::new(1.0);

    /// Gain measured in decibels.
    #[must_use]
    pub fn new_db(db: f64) -> Self {
        Self::new(10f64.powf(db / 20.0))
    }

    /// The gain in decibels.
    #[must_use]
    pub fn db(&self) -> f64 {
        20.0 * self.gain.log10()
    }
}

impl Default for Vol {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Map for Vol {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        x * self.gain
    }
}

/// Controls the volume of a signal.
#[derive(Clone, Debug, Default)]
pub struct Volume<S: Signal> {
    /// Inner data.
    inner: PwMapSgn<S, Vol>,
}

impl<S: Signal> Volume<S> {
    /// Initializes a new signal with a given [`Vol`].
    pub const fn new(sgn: S, vol: Vol) -> Self {
        Self {
            inner: PwMapSgn::new_pw(sgn, vol),
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
    pub const fn vol(&self) -> Vol {
        *self.inner.map_pw()
    }

    /// Returns a mutable reference to the volume of the signal.
    pub fn vol_mut(&mut self) -> &mut Vol {
        self.inner.map_pw_mut()
    }
}

impl<S: Signal> Signal for Volume<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner.get()
    }

    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency> Frequency for Volume<S> {
    fn freq(&self) -> Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
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
    /// Dummy variable.
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

impl<S: Signal> Mut<Volume<S>, f64> for Trem<S> {
    fn modify(&mut self, sgn: &mut Volume<S>, gain: f64) {
        sgn.vol_mut().gain = gain;
    }
}

/// Applies tremolo to a signal according to an envelope.
///
/// This signal stops whenever the original signal does. If you instead want a
/// signal that stops when the envelope does, use [`StopTremolo`].
#[derive(Clone, Debug)]
pub struct Tremolo<S: Signal, E: Signal<Sample = Env>> {
    /// Inner data.
    inner: MutSgn<Volume<S>, E, Trem<S>>,
}

impl<S: Signal, E: Signal<Sample = Env>> Tremolo<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, env: E, vol: Vol) -> Self {
        Self {
            inner: MutSgn::new(Volume::new(sgn, vol), env, Trem::new()),
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

impl<S: Signal, E: Signal<Sample = Env>> Signal for Tremolo<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner.get()
    }

    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: Signal<Sample = Env>> Frequency for Tremolo<S, E> {
    fn freq(&self) -> Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: Signal<Sample = Env>> Base for Tremolo<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: Done, E: Signal<Sample = Env>> Done for Tremolo<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Stop, E: Signal<Sample = Env>> Stop for Tremolo<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: Panic, E: Signal<Sample = Env>> Panic for Tremolo<S, E> {
    fn panic(&mut self) {
        self.inner.panic();
    }
}

#[derive(Clone, Debug)]
pub struct StopTremolo<S: Signal, E: Stop<Sample = Env>> {
    /// Inner data.
    inner: ModSgn<Volume<S>, E, Trem<S>>,
}

impl<S: Signal, E: Stop<Sample = Env>> StopTremolo<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, env: E, vol: Vol) -> Self {
        Self {
            inner: ModSgn::new(Volume::new(sgn, vol), env, Trem::new()),
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

impl<S: Signal, E: Stop<Sample = Env>> Signal for StopTremolo<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner.get()
    }

    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: Stop<Sample = Env>> Frequency for StopTremolo<S, E> {
    fn freq(&self) -> Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: Stop<Sample = Env>> Base for StopTremolo<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: Signal, E: Stop<Sample = Env> + Done> Done for StopTremolo<S, E> {
    fn is_done(&self) -> bool {
        self.env().is_done()
    }
}

impl<S: Signal, E: Stop<Sample = Env>> Stop for StopTremolo<S, E> {
    fn stop(&mut self) {
        self.env_mut().stop();
    }
}

impl<S: Signal, E: Stop<Sample = Env> + Panic> Panic for StopTremolo<S, E> {
    fn panic(&mut self) {
        self.env_mut().panic();
    }
}

/// Gates a signal through an envelope.
///
/// Output will only come through when the envelope is above the threshold.
#[derive(Clone, Debug)]
pub struct Gate<S: Signal, E: Signal<Sample = Env>> {
    /// The gated signal.
    pub sgn: S,

    /// The envelope for the gating.
    pub env: E,

    /// The threshold for the gate.
    pub threshold: f64,
}

impl<S: Signal, E: Signal<Sample = Env>> Gate<S, E> {
    /// Initializes a new gate.
    pub fn new(sgn: S, env: E, threshold: f64) -> Self {
        Self {
            sgn,
            env,
            threshold,
        }
    }
}

impl<S: Signal, E: Signal<Sample = Env>> Signal for Gate<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        if self.env.get().0 >= self.threshold {
            self.sgn.get()
        } else {
            S::Sample::ZERO
        }
    }

    fn advance(&mut self) {
        self.sgn.advance();
        self.env.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.env.retrigger();
    }
}

// Todo: fade-in/fade-out
