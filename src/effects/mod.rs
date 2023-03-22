//! Various simple effects one can use to modify signals.

pub mod adsr;
pub mod distortion;
pub mod freq;
pub mod mix;
pub mod pan;
pub mod sequence;
pub mod vol;

use crate::prelude::*;

/// Modifies a signal according to a given envelope.
#[derive(Clone, Debug)]
pub struct MutSgn<S: Signal, E: Signal<Sample = Env>, F: Mut<S, f64>> {
    /// The signal to modify.
    pub sgn: S,

    /// The envelope modifying the signal.
    pub env: E,

    /// The function to modify the signal.
    pub func: F,
}

impl<S: Signal, E: Signal<Sample = Env>, F: Mut<S, f64>> MutSgn<S, E, F> {
    /// Initializes a new [`MutSgn`].
    pub fn new_generic(mut sgn: S, env: E, mut func: F) -> Self {
        func.modify(&mut sgn, env.get().0);
        Self { sgn, env, func }
    }
}

impl<S: Signal, E: Signal<Sample = Env>, F: Mut<S, f64>> Signal for MutSgn<S, E, F> {
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

impl<S: Stop, E: Signal<Sample = Env>, F: Mut<S, f64>> Stop for MutSgn<S, E, F> {
    fn stop(&mut self) {
        self.sgn.stop();
    }

    fn is_done(&self) -> bool {
        self.sgn.is_done()
    }
}
