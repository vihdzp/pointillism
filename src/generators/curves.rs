//! Declares [`CurveEnv`], [`LoopCurveEnv`], and [`LoopGen`].
//!
//! These structures allow one to generate [`Signals`](Signal) from a curve,
//! meaning a struct implementing `Map<Input = f64, Output = f64>`.
//!
//! See also [`crate::curves`], where many basic curves are defined.

use std::marker::PhantomData;

use crate::prelude::*;

/// A map which converts an envelope into mono audio.
#[derive(Clone, Copy, Debug, Default)]
pub struct EnvToMono;

impl Map for EnvToMono {
    type Input = Env;
    type Output = Mono;

    fn eval(&self, x: Env) -> Mono {
        x.into()
    }
}

impl<S: Signal<Sample = Env>> MapSgn<S, EnvToMono> {
    /// Plays an envelope as a [`Mono`] audio file.
    ///
    /// For very low-frequency envelopes, this might lead to undesirable sounds.
    pub const fn env_gen(sgn: S) -> Self {
        Self::new(sgn, EnvToMono)
    }
}

/// Plays a curve at a specified speed, until it reaches the right endpoint.
///
/// See also [`LoopGen`].
#[derive(Clone, Copy, Debug)]
pub struct OneshotGen<S: Sample, C: Map<Input = f64, Output = f64>> {
    /// The curve being played.
    pub curve: C,

    /// The time for which the curve is played.
    pub time: Time,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to
    /// play.
    val: f64,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> OneshotGen<S, C> {
    /// Initializes a new [`CurveEnv`].
    pub const fn new(curve: C, time: Time) -> Self {
        Self {
            curve,
            time,
            val: 0.0,
            phantom: PhantomData,
        }
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along
    /// the curve we're currently reading.
    pub const fn val(&self) -> f64 {
        self.val
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Signal for OneshotGen<S, C> {
    type Sample = S;

    fn get(&self) -> S {
        S::from_val(self.curve.eval(self.val))
    }

    fn advance(&mut self) {
        self.val += 1.0 / self.time.frames();
        self.val = self.val.min(1.0);
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Base for OneshotGen<S, C> {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Done for OneshotGen<S, C> {
    fn is_done(&self) -> bool {
        self.val >= 1.0
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Stop for OneshotGen<S, C> {
    fn stop(&mut self) {
        self.val = 1.0;
    }
}

/// Loops a curve at a specified frequency.
///
/// This is an envelope, meaning it returns [`Env`] data. See [`LoopGen`] if
/// you want to generate mono audio instead.
///
/// See also [`CurveEnv`].
#[derive(Clone, Copy, Debug, Default)]
pub struct LoopGen<S: Sample, C: Map<Input = f64, Output = f64>> {
    /// The curve being played.
    pub curve: C,

    /// The frequency at which the curve is played.
    pub freq: Freq,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to
    /// play.
    val: f64,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> LoopGen<S, C> {
    /// Initializes a new [`LoopCurveEnv`].
    pub const fn new(curve: C, freq: Freq) -> Self {
        Self {
            curve,
            freq,
            val: 0.0,
            phantom: PhantomData,
        }
    }

    /// A reference to the curve being played.
    pub const fn curve(&self) -> &C {
        &self.curve
    }

    /// A mutable reference to the curve being played.
    pub fn curve_mut(&mut self) -> &mut C {
        &mut self.curve
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along
    /// the curve we're currently reading.
    pub fn val(&self) -> f64 {
        self.val
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Signal for LoopGen<S, C> {
    type Sample = S;

    fn get(&self) -> S {
        S::from_val(self.curve.eval(self.val))
    }

    fn advance(&mut self) {
        self.val += self.freq.hz() / crate::SAMPLE_RATE_F64;
        self.val %= 1.0;
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Frequency for LoopGen<S, C> {
    fn freq(&self) -> Freq {
        self.freq
    }

    fn freq_mut(&mut self) -> &mut Freq {
        &mut self.freq
    }
}

impl<S: Sample, C: Map<Input = f64, Output = f64>> Base for LoopGen<S, C> {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
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
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
    }
}
