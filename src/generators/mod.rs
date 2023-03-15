//! Structures that generate signals, be they envelope or audio data.

use crate::{sample::*, signal::Signal, Freq, Map, Time};

pub mod poly;

/// A sine curve.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sin {
    /// Phase of the sine curve, from `0.0` to `1.0`.
    pub phase: f64,
}

impl Sin {
    /// A sine wave with a given phase.
    pub const fn new(phase: f64) -> Self {
        Self { phase }
    }

    /// A sine wave.
    #[allow(clippy::self_named_constructors)]
    pub const fn sin() -> Self {
        Self::new(0.0)
    }

    /// A cosine wave.
    pub const fn cos() -> Self {
        Self::new(0.25)
    }
}

impl Map<f64, f64> for Sin {
    fn eval(&self, x: f64) -> f64 {
        ((x + self.phase) * std::f64::consts::TAU).sin()
    }
}

/// A pulse wave.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug)]
pub struct Pulse {
    /// Shape of the pulse curve, from `0.0` to `1.0`.
    pub shape: f64,
}

impl Pulse {
    /// A pulse wave.
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }

    /// A square wave.
    pub const fn sq() -> Self {
        Self::new(0.5)
    }
}

impl Default for Pulse {
    fn default() -> Self {
        Self::sq()
    }
}

impl Map<f64, f64> for Pulse {
    fn eval(&self, x: f64) -> f64 {
        if x < self.shape {
            1.0
        } else {
            -1.0
        }
    }
}

/// A curve interpolating between a saw and a triangle.
///
/// Takes on values between `-1.0` and `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct SawTri {
    /// Position of the peak of the wave.
    pub shape: f64,
}

impl SawTri {
    /// A saw-triangle curve.
    pub const fn new(shape: f64) -> Self {
        Self { shape }
    }

    /// A (right to left) saw wave.
    pub const fn inv_saw() -> Self {
        Self::new(0.0)
    }

    /// A triangle wave.
    pub const fn tri() -> Self {
        Self::new(0.5)
    }

    /// A (left to right) saw wave.
    pub const fn saw() -> Self {
        Self::new(1.0)
    }
}

impl Map<f64, f64> for SawTri {
    fn eval(&self, x: f64) -> f64 {
        if x < self.shape {
            2.0 * x / self.shape - 1.0
        } else {
            2.0 * (1.0 - x) / (1.0 - self.shape) - 1.0
        }
    }
}

/// Plays a curve at a specified speed, until it reaches the right endpoint.
#[derive(Clone, Copy, Debug)]
pub struct CurveEnv<C: Map<f64, f64>> {
    /// The curve being played.
    pub curve: C,

    /// The time for which the curve is played.
    pub time: Time,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to
    /// play.
    val: f64,
}

impl<C: Map<f64, f64>> CurveEnv<C> {
    /// Initializes a new [`CurveEnv`].
    pub fn new(curve: C, time: Time) -> Self {
        Self {
            curve,
            time,
            val: 0.0,
        }
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along
    /// the curve we're currently reading.
    pub fn val(&self) -> f64 {
        self.val
    }
}

impl<C: Map<f64, f64>> Signal for CurveEnv<C> {
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

/// Loops a curve at a specified frequency.
#[derive(Clone, Copy, Debug)]
pub struct LoopCurveEnv<C: Map<f64, f64>> {
    /// The curve being played.
    pub curve: C,

    /// The frequency at which the curve is played.
    pub freq: Freq,

    /// A value between `0.0` and `1.0` indicating what sample of the curve to
    /// play.
    val: f64,
}

impl<C: Map<f64, f64>> LoopCurveEnv<C> {
    /// Initializes a new [`LoopCurveEnv`].
    pub fn new(curve: C, freq: Freq) -> Self {
        Self {
            curve,
            freq,
            val: 0.0,
        }
    }

    /// Returns the value between `0.0` and `1.0` which represents how far along
    /// the curve we're currently reading.
    pub fn val(&self) -> f64 {
        self.val
    }
}

impl<C: Map<f64, f64>> Signal for LoopCurveEnv<C> {
    type Sample = Env;

    fn get(&self) -> Env {
        Env(self.curve.eval(self.val))
    }

    fn advance(&mut self) {
        self.val += self.freq.hz() / crate::SAMPLE_RATE as f64;
        self.val %= 1.0;
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
    }
}

/// Plays a curve as a [`Mono`] audio file.
///
/// For very low-frequency curves, this might lead to undesirable sounds.
pub type CurveGen<C> = crate::signal::EnvGen<LoopCurveEnv<C>>;

impl<C: Map<f64, f64>> CurveGen<C> {
    /// Initializes a [`CurveGen`] from a [`LoopCurveEnv`].
    pub fn new_sgn(curve_sgn: LoopCurveEnv<C>) -> Self {
        Self::new_env(curve_sgn)
    }

    /// Initializes a [`CurveGen`] from a given curve and a frequency.
    pub fn new(curve: C, freq: Freq) -> Self {
        Self::new_sgn(LoopCurveEnv::new(curve, freq))
    }

    /// Returns the frequency of the curve.
    pub fn freq(&self) -> Freq {
        self.sgn.freq
    }

    /// Returns a mutable refrence to the frequency of the curve.
    pub fn freq_mut(&mut self) -> &mut Freq {
        &mut self.sgn.freq
    }
}

/// Generates random data.
#[derive(Clone, Copy, Debug)]
pub struct NoiseGen<S: Sample> {
    /// The current random value.
    val: S,
}

impl<S: Sample> Default for NoiseGen<S> {
    fn default() -> Self {
        Self { val: S::rand() }
    }
}

impl<S: Sample> NoiseGen<S> {
    /// Initializes a new [`NoiseGen`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Sample> Signal for NoiseGen<S> {
    type Sample = S;

    fn get(&self) -> Self::Sample {
        self.val
    }

    fn advance(&mut self) {
        self.val = S::rand();
    }

    fn retrigger(&mut self) {
        self.advance();
    }
}
