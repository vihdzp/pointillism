//! Various simple effects one can use to modify signals.

pub mod adsr;
pub mod distortion;
pub mod pan;
pub mod sequence;
pub mod vol;

use crate::prelude::*;

/// Modifies a signal according to a given envelope.
#[derive(Clone, Debug)]
pub struct Envelope<S: Signal, E: Signal<Sample = Env>, F: MapMut<S, f64>> {
    /// The signal to modify.
    pub sgn: S,

    /// The envelope modifying the signal.
    pub env: E,

    /// The function to modify the signal.
    pub func: F,
}

impl<S: Signal, E: Signal<Sample = Env>, F: MapMut<S, f64>> Envelope<S, E, F> {
    /// Initializes a new [`Envelope`].
    pub fn new_generic(mut sgn: S, env: E, mut func: F) -> Self {
        func.modify(&mut sgn, env.get().0);
        Self { sgn, env, func }
    }
}

impl<S: Signal, E: Signal<Sample = Env>, F: MapMut<S, f64>> Signal for Envelope<S, E, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn.get()
    }

    fn advance(&mut self) {
        self.sgn.advance();
        self.func.modify(&mut self.sgn, self.env.next().0);
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.env.retrigger();
    }
}

impl<S: StopSignal, E: Signal<Sample = Env>, F: MapMut<S, f64>> StopSignal for Envelope<S, E, F> {
    fn stop(&mut self) {
        self.sgn.stop();
    }

    fn is_done(&self) -> bool {
        self.sgn.is_done()
    }
}

/// Gates a signal through an envelope. Output will only come through when the
/// envelope is above the threshold.
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
