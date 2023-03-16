//! Structures that generate signals, be they envelope or audio data.

use crate::{sample::*, signal::Signal, Freq, Map, Time};

pub mod curves;
pub mod mix;
pub mod noise;
pub mod poly;

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

    /// A reference to the curve being played.
    pub const fn curve(&self) -> &C {
        &self.curve
    }

    /// A mutable reference to the curve being played.
    pub fn curve_mut(&mut self) -> &mut C {
        &mut self.curve
    }

    /// Returns the frequency of the curve.
    pub fn freq(&self) -> Freq {
        self.freq
    }

    /// Returns a mutable refrence to the frequency of the curve.
    pub fn freq_mut(&mut self) -> &mut Freq {
        &mut self.freq
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

    /// A reference to the curve being played.
    pub fn curve(&self) -> &C {
        self.sgn().curve()
    }

    /// A mutable reference to the curve being played.
    pub fn curve_mut(&mut self) -> &mut C {
        self.sgn_mut().curve_mut()
    }

    /// Returns the frequency of the curve.
    pub fn freq(&self) -> Freq {
        self.sgn().freq()
    }

    /// Returns a mutable refrence to the frequency of the curve.
    pub fn freq_mut(&mut self) -> &mut Freq {
        self.sgn_mut().freq_mut()
    }
}
