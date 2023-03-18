use std::marker::PhantomData;

use crate::{prelude::*, signal::PointwiseMapSgn};

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
    pub const fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Gain measured in decibels.
    pub fn new_db(db: f64) -> Self {
        Self::new(10f64.powf(db / 20.0))
    }

    /// The gain in decibels.
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

impl<S: Signal> Default for Trem<S> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<S: Signal> Trem<S> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Signal> MapMut<Volume<S>, f64> for Trem<S> {
    fn modify(&mut self, sgn: &mut Volume<S>, gain: f64) {
        sgn.vol_mut().gain = gain;
    }
}

/// Applies tremolo to a signal according to an envelope.
pub type Tremolo<S, E> = Envelope<Volume<S>, E, Trem<S>>;

impl<S: Signal, E: Signal<Sample = Env>> Tremolo<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, env: E) -> Self {
        Self::new_generic(Volume::new(sgn, Vol::new(1.0)), env, Trem::new())
    }
}
