//! Effects pertaining to frequency, such as [`Vibrato`].

use std::marker::PhantomData;

use crate::prelude::*;

/// The function that applies vibrato to a signal.
pub struct Vib<S: Frequency> {
    /// Base frequency.
    pub base: unt::Freq,
    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Frequency> Vib<S> {
    /// Initializes a new [`Vib`].
    #[must_use]
    pub const fn new(base: unt::Freq) -> Self {
        Self {
            base,
            phantom: PhantomData,
        }
    }
}

impl<S: Frequency> Default for Vib<S> {
    fn default() -> Self {
        Self::new(unt::Freq::default())
    }
}

impl<S: Frequency> Env<S> for Vib<S> {
    fn modify_env(&mut self, sgn: &mut S, bend: smp::Env) {
        *sgn.freq_mut() = self.base * bend.0;
    }
}

/// Applies vibrato (change in pitch) to a signal according to an envelope.
///
/// The envelope serves as a multiplier for a base frequency.
pub struct Vibrato<S: Frequency, E: SignalMut<Sample = smp::Env>> {
    /// Inner data.
    inner: eff::MutSgn<S, E, Vib<S>>,
}

impl<S: Frequency, E: SignalMut<Sample = smp::Env>> Vibrato<S, E> {
    /// Initializes a new [`Vibrato`].
    pub fn new(sgn: S, base: unt::Freq, env: E) -> Self {
        Self {
            inner: eff::MutSgn::new(sgn, env, Vib::new(base)),
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

impl<S: Frequency, E: SignalMut<Sample = smp::Env>> Signal for Vibrato<S, E> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner._get()
    }
}

impl<S: Frequency, E: SignalMut<Sample = smp::Env>> SignalMut for Vibrato<S, E> {
    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency + Base, E: SignalMut<Sample = smp::Env>> Base for Vibrato<S, E> {
    type Base = S::Base;

    fn base(&self) -> &Self::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self.inner.base_mut()
    }
}

impl<S: Frequency, E: SignalMut<Sample = smp::Env>> Frequency for Vibrato<S, E> {
    fn freq(&self) -> unt::Freq {
        self.inner.func().base
    }

    fn freq_mut(&mut self) -> &mut unt::Freq {
        &mut self.inner.func_mut().base
    }
}

impl<S: Frequency + Done, E: SignalMut<Sample = smp::Env>> Done for Vibrato<S, E> {
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Frequency + Stop, E: SignalMut<Sample = smp::Env>> Stop for Vibrato<S, E> {
    fn stop(&mut self) {
        self.inner.stop();
    }
}
