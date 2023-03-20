use std::marker::PhantomData;

use crate::prelude::*;

/// A trait for a signal with a "main" frequency.
///
/// This is implemented both for signals that have a frequency parameter such as
/// [`LoopCurveEnv`], as well as straightforward wrappers for these signals.
pub trait HasFreq: Signal {
    /// The "main" frequency of the signal.
    fn freq(&self) -> Freq;

    /// A mutable reference to the "main" frequency of the signal.
    fn freq_mut(&mut self) -> &mut Freq;
}

/// The function that applies vibrato to a signal.
pub struct Vib<S: HasFreq> {
    /// Base frequency.
    pub base: Freq,

    /// Dummy variable.
    phantom: PhantomData<S>,
}

impl<S: HasFreq> Vib<S> {
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

impl<S: HasFreq> MapMut<S, f64> for Vib<S> {
    fn modify(&mut self, sgn: &mut S, bend: f64) {
        *sgn.freq_mut() = self.base * bend;
    }
}

/// Applies tremolo to a signal according to an envelope.
pub type Vibrato<S, E> = Envelope<S, E, Vib<S>>;

impl<S: HasFreq, E: Signal<Sample = Env>> Vibrato<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, base: Freq, env: E) -> Self {
        Self::new_generic(sgn, env, Vib::new(base))
    }
}
