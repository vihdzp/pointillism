//! Structures for changing the volume of an audio signal.

use std::marker::PhantomData;

use crate::prelude::*;

/// Represents the gain of some signal.
#[derive(Clone, Copy, Debug)]
pub struct Vol {
    /// Gain factor.
    pub gain: f64,
}

impl Vol {
    /// Silence.
    pub const ZERO: Self = Self::new(0.0);

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
pub type Volume<S> = PointwiseMapSgn<S, Vol>;

impl<S: Signal> Volume<S> {
    /// Initializes a new signal with a given [`Vol`].
    pub const fn new(sgn: S, vol: Vol) -> Self {
        Self::new_pointwise(sgn, vol)
    }

    /// Volume of the signal.
    pub const fn vol(&self) -> Vol {
        *self.func()
    }

    /// Returns a mutable reference to the volume of the signal.
    pub fn vol_mut(&mut self) -> &mut Vol {
        self.func_mut()
    }
}

/// The function that applies tremolo to a volume signal.
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
pub type Tremolo<S, E> = MutSgn<Volume<S>, E, Trem<S>>;

impl<S: Signal, E: Signal<Sample = Env>> Tremolo<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, env: E) -> Self {
        Self::new_generic(Volume::new(sgn, Vol::new(1.0)), env, Trem::new())
    }

    pub fn sgn(&self) -> &S {
        self.sgn.sgn()
    }

    pub fn sgn_mut(&mut self) -> &mut S {
        self.sgn.sgn_mut()
    }
}

/// Applies tremolo to a signal according to a curve envelope.
///
/// In contrast to [`Tremolo`], this signal stops when the curve does.
pub struct StopTremolo<S: Signal, C: Map<Input = f64, Output = f64>> {
    pub sgn: Tremolo<S, CurveEnv<C>>,
}

impl<S: Signal, C: Map<Input = f64, Output = f64>> StopTremolo<S, C> {
    pub fn new(sgn: S, curve_env: CurveEnv<C>) -> Self {
        Self {
            sgn: Tremolo::new(sgn, curve_env),
        }
    }
}

impl<S: Signal, C: Map<Input = f64, Output = f64>> Signal for StopTremolo<S, C> {
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.sgn.get()
    }

    fn advance(&mut self) {
        self.sgn.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
    }
}

impl<S: Signal, C: Map<Input = f64, Output = f64>> Stop for StopTremolo<S, C> {
    fn stop(&mut self) {}

    fn is_done(&self) -> bool {
        self.sgn.env.val() >= 1.0
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
