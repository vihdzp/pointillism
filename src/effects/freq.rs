//! Effects pertaining to frequency, such as [`Vibrato`].

use std::marker::PhantomData;

use crate::prelude::*;

/// The function that applies vibrato to a signal.
pub struct Vib<S: Frequency> {
    /// Base frequency.
    pub base: Freq,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Frequency> Vib<S> {
    /// Initializes a new [`Vib`].
    #[must_use]
    pub const fn new(base: Freq) -> Self {
        Self {
            base,
            phantom: PhantomData,
        }
    }
}

impl<S: Frequency> Default for Vib<S> {
    fn default() -> Self {
        Self::new(Freq::default())
    }
}

impl<S: Frequency> MutEnv<S> for Vib<S> {
    fn modify_env(&mut self, sgn: &mut S, bend: Env) {
        *sgn.freq_mut() = self.base * bend.0;
    }
}

/// Applies vibrato (change in pitch) to a signal according to an envelope.
pub struct Vibrato<S: Frequency, E: SignalMut<Sample = Env>> {
    /// Inner data.
    inner: MutSgn<S, E, Vib<S>>,
}

impl<S: Frequency, E: SignalMut<Sample = Env>> Vibrato<S, E> {
    /// Initializes a new [`Tremolo`].
    pub fn new(sgn: S, base: Freq, env: E) -> Self {
        Self {
            inner: MutSgn::new(sgn, env, Vib::new(base)),
        }
    }

    /// Returns a reference to the signal whose volume is modulated.
    pub const fn sgn(&self) -> &S {
        self.inner.sgn()
    }

    /// Returns a mutable reference to the signal whose volume is modulated.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut()
    }

    /// Returns a reference to the envelope controlling the volume.
    pub const fn env(&self) -> &E {
        self.inner.env()
    }

    /// Returns a mutable reference to the envelope controlling the volume.
    pub fn env_mut(&mut self) -> &mut E {
        self.inner.env_mut()
    }
}

impl<S: Frequency, E: SignalMut<Sample = Env>> Signal for Vibrato<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner.get()
    }
}

impl<S: Frequency, E: SignalMut<Sample = Env>> SignalMut for Vibrato<S, E> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency + Base, E: SignalMut<Sample = Env>> Base for Vibrato<S, E> {
    type Base = S::Base;

    fn base(&self) -> &Self::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self.inner.base_mut()
    }
}

impl<S: Frequency, E: SignalMut<Sample = Env>> Frequency for Vibrato<S, E> {
    fn freq(&self) -> Freq {
        self.inner.func().base
    }

    fn freq_mut(&mut self) -> &mut Freq {
        &mut self.inner.func_mut().base
    }
}

impl<S: Frequency + Done, E: SignalMut<Sample = Env>> Done for Vibrato<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Frequency + Stop, E: SignalMut<Sample = Env>> Stop for Vibrato<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}
