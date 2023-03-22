//! Declares [`CurveEnv`], [`LoopCurveEnv`], and [`CurveGen`].
//!
//! These structures allow one to generate [`Signals`](Signal) from a curve,
//! meaning a struct implementing `Map<Input = f64, Output = f64>`.
//!
//! See also [`crate::curves`], where many basic curves are defined.

use crate::{
    freq::Freq,
    map::Map,
    sample::{Env, Mono, Sample},
    signal::{HasBase, HasFreq, MapSgn, Signal},
    time::Time,
};

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

/// Plays an envelope as a [`Mono`] audio file.
///
/// For very low-frequency envelopes, this might lead to undesirable sounds.
pub type EnvGen<S> = MapSgn<S, EnvToMono>;

impl<S: Signal<Sample = Env>> EnvGen<S> {
    /// Initializes a new [`EnvGen`].
    pub const fn new_env(sgn: S) -> Self {
        Self::new_generic(sgn, EnvToMono)
    }
}

/// Plays a curve at a specified speed, until it reaches the right endpoint.
///
/// See also [`LoopCurveEnv`].
#[derive(Clone, Copy, Debug)]
pub struct CurveEnv<C: Map<Input = f64, Output = f64>> {
    /// The curve being played.
    pub curve: C,

    /// The time for which the curve is played.
    pub time: Time,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to
    /// play.
    val: f64,
}

impl<C: Map<Input = f64, Output = f64>> CurveEnv<C> {
    /// Initializes a new [`CurveEnv`].
    pub const fn new(curve: C, time: Time) -> Self {
        Self {
            curve,
            time,
            val: 0.0,
        }
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along
    /// the curve we're currently reading.
    pub const fn val(&self) -> f64 {
        self.val
    }
}

impl<C: Map<Input = f64, Output = f64>> Signal for CurveEnv<C> {
    type Sample = Env;

    fn get(&self) -> Env {
        Env(self.curve.eval(self.val))
    }

    fn advance(&mut self) {
        self.val += 1.0 / self.time.frames();
        self.val = self.val.min(1.0);
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

impl<C: Map<Input = f64, Output = f64>> HasBase for CurveEnv<C> {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
    }
}

/// Loops a curve at a specified frequency.
///
/// This is an envelope, meaning it returns [`Env`] data. See [`CurveGen`] if
/// you want to generate mono audio instead.
///
/// See also [`CurveEnv`].
#[derive(Clone, Copy, Debug, Default)]
pub struct LoopCurveEnv<C: Map<Input = f64, Output = f64>> {
    /// The curve being played.
    pub curve: C,

    /// The frequency at which the curve is played.
    pub freq: Freq,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to
    /// play.
    val: f64,
}

impl<C: Map<Input = f64, Output = f64>> HasFreq for LoopCurveEnv<C> {
    fn freq(&self) -> Freq {
        self.freq
    }

    fn freq_mut(&mut self) -> &mut Freq {
        &mut self.freq
    }
}

impl<C: Map<Input = f64, Output = f64>> LoopCurveEnv<C> {
    /// Initializes a new [`LoopCurveEnv`].
    pub const fn new(curve: C, freq: Freq) -> Self {
        Self {
            curve,
            freq,
            val: 0.0,
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

impl<C: Map<Input = f64, Output = f64>> Signal for LoopCurveEnv<C> {
    type Sample = Env;

    fn get(&self) -> Env {
        Env(self.curve.eval(self.val))
    }

    fn advance(&mut self) {
        self.val += self.freq.hz() / crate::SAMPLE_RATE_F64;
        self.val %= 1.0;
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

impl<C: Map<Input = f64, Output = f64>> HasBase for LoopCurveEnv<C> {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
    }
}

/// Plays a curve as a [`Mono`] audio file.
///
/// For very low-frequency curves, this might lead to undesirable sounds.
pub type CurveGen<C> = crate::prelude::EnvGen<LoopCurveEnv<C>>;

impl<C: Map<Input = f64, Output = f64>> CurveGen<C> {
    /// Initializes a [`CurveGen`] from a [`LoopCurveEnv`].
    pub const fn new_sgn(curve_sgn: LoopCurveEnv<C>) -> Self {
        Self::new_env(curve_sgn)
    }

    /// Initializes a [`CurveGen`] from a given curve and a frequency.
    pub const fn new(curve: C, freq: Freq) -> Self {
        Self::new_sgn(LoopCurveEnv::new(curve, freq))
    }

    /// A reference to the curve being played.
    pub const fn curve(&self) -> &C {
        self.sgn().curve()
    }

    /// A mutable reference to the curve being played.
    pub fn curve_mut(&mut self) -> &mut C {
        self.sgn_mut().curve_mut()
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

impl<S: Sample> HasBase for NoiseGen<S> {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
    }
}
