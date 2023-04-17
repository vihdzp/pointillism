//! Implements an ADSR envelope for signals.

use crate::prelude::*;

/// Any of the stages in an ADSR envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    /// Start to peak.
    Attack,

    /// Peak to sustain.
    Decay,

    /// Sustain.
    Sustain,

    /// Stop to done.
    Release,

    /// Done.
    Done,
}

/// An ADSR envelope. Outputs values between `0.0` and `1.0`.
#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    /// The time from the signal start to its peak.
    pub attack: Time,

    /// The time from the signal peak to its sustain point.
    pub decay: Time,

    /// The sustain value for the signal.
    pub sustain: f64,

    /// The time from the signal stop to it being done.
    pub release: Time,

    /// Current stage of the envelope.
    stage: Stage,

    /// The value from which the sustain starts.
    ///
    /// This can differ from the sustain value if the envelope is stopped before
    /// the `Sustain` phase.
    sustain_val: f64,

    /// A value from `0.0` to `1.0` representing how far along the phase we are.
    val: f64,
}

impl Adsr {
    /// Initializes a new [`Adsr`] envelope.
    #[must_use]
    pub fn new(attack: Time, decay: Time, sustain: f64, release: Time) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            stage: Stage::Attack,
            val: 0.0,

            // Is properly initialized in `stop`.
            sustain_val: 0.0,
        }
    }

    /// Current stage of the envelope.
    #[must_use]
    pub fn stage(&self) -> Stage {
        self.stage
    }
}

impl Signal for Adsr {
    type Sample = Env;

    fn get(&self) -> Env {
        Env(match self.stage() {
            Stage::Attack => self.val,
            Stage::Decay => 1.0 + (self.sustain - 1.0) * self.val,
            Stage::Sustain => self.sustain,
            Stage::Release => self.sustain_val * (1.0 - self.val),
            Stage::Done => 0.0,
        })
    }

    fn advance(&mut self) {
        match self.stage() {
            Stage::Attack => {
                self.val += 1.0 / self.attack.frames();

                if self.val > 1.0 {
                    self.stage = Stage::Decay;
                    self.val = 0.0;
                }
            }

            Stage::Decay => {
                self.val += 1.0 / self.decay.frames();

                if self.val > 1.0 {
                    self.stage = Stage::Sustain;
                    self.val = 0.0;
                }
            }

            Stage::Release => {
                self.val += 1.0 / self.release.frames();

                if self.val > 1.0 {
                    self.stage = Stage::Done;
                }
            }

            Stage::Sustain | Stage::Done => {}
        }
    }

    fn retrigger(&mut self) {
        self.stage = Stage::Attack;
        self.val = 0.0;
    }
}

impl Done for Adsr {
    fn is_done(&self) -> bool {
        self.stage == Stage::Done
    }
}

impl Stop for Adsr {
    fn stop(&mut self) {
        self.sustain_val = self.get().0;
        self.val = 0.0;
        self.stage = Stage::Release;
    }
}

impl Panic for Adsr {
    fn panic(&mut self) {
        self.stage = Stage::Done;
    }
}

/// A stoppable signal, hooked to an ADSR envelope.
#[derive(Clone, Debug)]
pub struct Envelope<S: Signal> {
    /// The inner envelope.
    inner: Tremolo<S, Adsr>,
}

impl<S: Signal> Envelope<S> {
    /// Initializes a new ADSR envelope.
    pub fn new(sgn: S, env: Adsr) -> Self {
        Self {
            inner: Tremolo::new(sgn, env, Vol::ZERO),
        }
    }

    /// Returns a reference to the signal modified by the ADSR envelope.
    pub fn sgn(&self) -> &S {
        self.inner.sgn()
    }

    /// Returns a mutable reference to the signal modified by the ADSR envelope.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut()
    }

    /// Returns a reference to the ADSR signal.
    pub fn adsr(&self) -> &Adsr {
        self.inner.env()
    }

    /// Returns a mutable reference to the ADSR signal.
    pub fn adsr_mut(&mut self) -> &mut Adsr {
        self.inner.env_mut()
    }
}

impl<S: Signal> Signal for Envelope<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inner.get()
    }

    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency> Frequency for Envelope<S> {
    fn freq(&self) -> Freq {
        self.sgn().freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn_mut().freq_mut()
    }
}

impl<S: Base> Base for Envelope<S> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn_mut().base_mut()
    }
}

impl<S: Signal> Done for Envelope<S> {
    fn is_done(&self) -> bool {
        self.adsr().is_done()
    }
}

impl<S: Signal> Stop for Envelope<S> {
    fn stop(&mut self) {
        self.adsr_mut().stop();
    }
}

impl<S: Signal> Panic for Envelope<S> {
    fn panic(&mut self) {
        self.adsr_mut().panic();
    }
}
