//! Effects pertaining to frequency, such as [`Vibrato`].

use std::marker::PhantomData;

use crate::prelude::*;

/// The function that applies vibrato to a signal.
pub struct Vib<S: HasFreq> {
    /// Base frequency.
    pub base: Freq,

    /// Dummy variable.
    phantom: PhantomData<S>,
}

impl<S: HasFreq> Vib<S> {
    /// Initializes a new [`Vib`].
    #[must_use]
    pub const fn new(base: Freq) -> Self {
        Self {
            base,
            phantom: PhantomData,
        }
    }
}

impl<S: HasFreq> Default for Vib<S> {
    fn default() -> Self {
        Self::new(Freq::default())
    }
}

impl<S: HasFreq> Mut<S, f64> for Vib<S> {
    fn modify(&mut self, sgn: &mut S, bend: f64) {
        *sgn.freq_mut() = self.base * bend;
    }
}

/// Applies tremolo to a signal according to an envelope.
pub type Vibrato<S, E> = MutSgn<S, E, Vib<S>>;

impl<S: HasFreq, E: Signal<Sample = Env>> Vibrato<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, base: Freq, env: E) -> Self {
        Self::new_generic(sgn, env, Vib::new(base))
    }
}
