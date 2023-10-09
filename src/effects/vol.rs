//! Structures for changing the volume of an audio signal.

use std::marker::PhantomData;

use crate::prelude::*;

/// Represents the gain of some signal.
///
/// This also implements the [`Map`] trait, thus doubling as a function that multiplies the volume
/// of a signal.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vol {
    /// Gain factor.
    pub gain: f64,
}

impl Vol {
    /// Silence.
    pub const ZERO: Self = Self::new(0.0);
    /// Half amplitude.
    pub const HALF: Self = Self::new(0.5);
    /// Full volume.
    pub const FULL: Self = Self::new(1.0);
    /// Twice the amplitude.
    pub const TWICE: Self = Self::new(2.0);

    /// -3 dB.
    ///
    /// Roughly corresponds to a halving of power.
    pub const MDB3: Self = Self::new(0.707_945_784_384_137_9);
    /// -6 dB.
    ///
    /// Roughly corresponds to a halving of amplitude, voltage, or sound power level (SPL).
    pub const MDB6: Self = Self::new(0.501_187_233_627_272_2);
    /// -10 dB.
    ///
    /// What a human might percieve as "half as loud".
    pub const MDB10: Self = Self::new(0.316_227_766_016_837_94);

    /// +3 dB.
    ///
    /// Roughly corresponds to a doubling of power.
    pub const DB3: Self = Self::new(1.412_537_544_622_754_4);
    /// +6 dB.
    ///
    /// Roughly corresponds to a doubling of amplitude, voltage, or sound power level (SPL).
    pub const DB6: Self = Self::new(1.995_262_314_968_879_5);
    /// +10 dB.
    ///
    /// What a human might percieve as "twice as loud".
    pub const DB10: Self = Self::new(3.162_277_660_168_379_5);

    /// Initializes a new volume variable.
    #[must_use]
    pub const fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Gain measured in decibels.
    #[must_use]
    pub fn new_db(db: f64) -> Self {
        Self::new(10f64.powf(db / 20.0))
    }

    /// Linearly converts MIDI velocity into gain.
    ///
    /// This is not necessarily the best way to interpret MIDI velocity, but it is the simplest.
    #[cfg(feature = "midly")]
    #[must_use]
    pub fn new_vel(vel: midly::num::u7) -> Self {
        Self::new(f64::from(vel.as_int()) / 127.0)
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
        // We need to call it like this or `rust-analyzer` gets tripped up.
        Signal::get(&self.inner)
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

impl<S: Signal> MutEnv<Volume<S>> for Trem<S> {
    fn modify_env(&mut self, sgn: &mut Volume<S>, gain: Env) {
        sgn.vol_mut().gain = gain.0;
    }
}

/// Applies tremolo to a signal.
///
/// Note that "tremolo" here just means a **change** in volume controlled by an envelope. This is
/// more general than the usual meaning of tremolo, this being **oscillation** in volume. For
/// instance, [`AdsrEnvelope`] is a special case of [`Tremolo`] (technically [`StopTremolo`]).
///
/// This signal stops whenever the original signal does. If you instead want a signal that stops
/// when the envelope does, use [`StopTremolo`].
#[derive(Clone, Debug)]
pub struct Tremolo<S: Signal, E: Signal<Sample = Env>> {
    /// Inner data.
    inner: MutSgn<Volume<S>, E, Trem<S>>,
}

impl<S: Signal, E: Signal<Sample = Env>> Tremolo<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, env: E) -> Self {
        Self {
            // The volume is unimportant, as it immediately gets rewritten.
            inner: MutSgn::new(Volume::new(sgn, Vol::FULL), env, Trem::new()),
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
        // We need to call it like this or `rust-analyzer` gets tripped up.
        Signal::get(&self.inner)
    }
}

impl<S: SignalMut, E: SignalMut<Sample = Env>> SignalMut for Tremolo<S, E> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, E: SignalMut<Sample = Env>> Frequency for Tremolo<S, E> {
    fn freq(&self) -> Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, E: SignalMut<Sample = Env>> Base for Tremolo<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.inner.base_mut()
    }
}

impl<S: SignalMut + Done, E: SignalMut<Sample = Env>> Done for Tremolo<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Stop, E: SignalMut<Sample = Env>> Stop for Tremolo<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: Panic, E: SignalMut<Sample = Env>> Panic for Tremolo<S, E> {
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
pub struct StopTremolo<S: SignalMut, E: Stop<Sample = Env>> {
    /// Inner data.
    inner: ModSgn<Volume<S>, E, Trem<S>>,
}

/// Turns a [`Tremolo`] into a [`StopTremolo`]. This changes the functionality of the signal when
/// stopped, as described in the [`StopTremolo`] docs.
impl<S: SignalMut, E: Stop<Sample = Env>> From<Tremolo<S, E>> for StopTremolo<S, E> {
    fn from(value: Tremolo<S, E>) -> Self {
        Self {
            inner: value.inner.into(),
        }
    }
}

/// An envelope with attack and release.
///
/// Initialize with [`Self::new_ar`].
pub type ArEnvelope<S> = StopTremolo<S, OnceGen<Env, SawTri>>;

impl<S: SignalMut, E: Stop<Sample = Env>> StopTremolo<S, E> {
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

impl<S: SignalMut> ArEnvelope<S> {
    /// Initializes a new [`ArEnvelope`].
    pub fn new_ar(sgn: S, attack: Time, release: Time) -> Self {
        let time = attack + release;
        let shape = attack / time;
        Self::new(sgn, OnceGen::new(SawTri::new(Val::new(shape)), time))
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>> Signal for StopTremolo<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        // We need to call it like this or `rust-analyzer` gets tripped up.
        Signal::get(&self.inner)
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>> SignalMut for StopTremolo<S, E> {
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

impl<S: SignalMut, E: Stop<Sample = Env> + Done> Done for StopTremolo<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: SignalMut, E: Stop<Sample = Env>> Stop for StopTremolo<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: SignalMut, E: Stop<Sample = Env> + Panic> Panic for StopTremolo<S, E> {
    fn panic(&mut self) {
        self.inner.panic();
    }
}

/// Gates a signal through an envelope.
///
/// Output will only come through when the envelope is above the threshold.
#[derive(Clone, Debug)]
pub struct Gate<S: Signal, E: Signal<Sample = Env>> {
    /// The gated signal.
    sgn: S,

    /// The envelope for the gating.
    env: E,

    /// The threshold for the gate.
    threshold: f64,
}

impl<S: Signal, E: Signal<Sample = Env>> Gate<S, E> {
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

impl<S: Signal, E: Signal<Sample = Env>> Signal for Gate<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        if self.env.get().0 >= self.threshold {
            self.sgn.get()
        } else {
            S::Sample::ZERO
        }
    }
}

impl<S: SignalMut, E: SignalMut<Sample = Env>> SignalMut for Gate<S, E> {
    fn advance(&mut self) {
        self.sgn.advance();
        self.env.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.env.retrigger();
    }
}

impl<S: Frequency, E: SignalMut<Sample = Env>> Frequency for Gate<S, E> {
    fn freq(&self) -> Freq {
        self.sgn.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn.freq_mut()
    }
}

impl<S: Base, E: SignalMut<Sample = Env>> Base for Gate<S, E> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: Done, E: SignalMut<Sample = Env>> Done for Gate<S, E> {
    fn is_done(&self) -> bool {
        self.sgn().is_done()
    }
}

impl<S: Stop, E: SignalMut<Sample = Env>> Stop for Gate<S, E> {
    fn stop(&mut self) {
        self.sgn_mut().stop();
    }
}

impl<S: Panic, E: SignalMut<Sample = Env>> Panic for Gate<S, E> {
    fn panic(&mut self) {
        self.sgn_mut().panic();
    }
}

// Todo: fade-in/fade-out
