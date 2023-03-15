pub mod adsr;
pub mod events;
mod pan;

pub use pan::*;

use crate::{sample::Env, signal::Signal, MapMut};

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
    /// Initializes a new envelope.
    pub fn new(mut sgn: S, env: E, func: F) -> Self {
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
