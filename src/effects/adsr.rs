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
            Stage::Release => self.sustain * (1.0 - self.val),
            Stage::Done => 0.0,
        })
    }

    fn advance(&mut self) {
        // Note that `self.val` can end up infinite. This isn't a problem
        // though, as it will simply skip the ADSR stage in the next frame.
        match self.stage() {
            Stage::Attack => {
                self.val += 1.0 / self.attack.frames();

                if self.val > 1.0 {
                    self.stage = Stage::Decay;
                    self.val -= 1.0;
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

impl Stop for Adsr {
    fn stop(&mut self) {
        self.sustain = self.get().0;
        self.val = 0.0;
        self.stage = Stage::Release;
    }

    fn is_done(&self) -> bool {
        self.stage == Stage::Done
    }
}

/// The function that changes the volume of a [`Volume<S>`] envelope.
#[derive(Clone, Copy, Debug)]
pub struct VolFn<S: Signal> {
    /// Dummy value.
    phantom: std::marker::PhantomData<S>,
}

impl<S: Signal> VolFn<S> {
    /// Initializes a new [`VolFn`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: std::marker::PhantomData,
        }
    }
}

impl<S: Signal> Default for VolFn<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Signal> Mut<Volume<S>, f64> for VolFn<S> {
    fn modify(&mut self, sgn: &mut Volume<S>, gain: f64) {
        *sgn.vol_mut() = Vol::new(gain);
    }
}

/// A stoppable signal, hooked to an ADSR envelope.
#[derive(Clone, Debug)]
pub struct Envelope<S: Signal> {
    /// The inner envelope.
    pub env: MutSgn<Volume<S>, Adsr, VolFn<S>>,
}

impl<S: Signal> Envelope<S> {
    /// Initializes a new ADSR envelope.
    pub fn new(sgn: S, env: Adsr) -> Self {
        Self {
            env: MutSgn::new_generic(Volume::new(sgn, Vol::ZERO), env, VolFn::new()),
        }
    }

    /// Returns a reference to the signal modified by the ADSR envelope.
    pub fn sgn(&self) -> &S {
        &self.env.sgn.sgn
    }

    /// Returns a mutable reference to the signal modified by the ADSR envelope.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.env.sgn.sgn
    }

    /// Returns a reference to the ADSR signal.
    pub fn adsr(&self) -> &Adsr {
        &self.env.env
    }

    /// Returns a mutable reference to the ADSR signal.
    pub fn adsr_mut(&mut self) -> &mut Adsr {
        &mut self.env.env
    }
}

impl<S: Signal> Signal for Envelope<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.env.get()
    }

    fn advance(&mut self) {
        self.env.advance();
    }

    fn retrigger(&mut self) {
        self.env.retrigger();
    }
}

impl<S: Signal> Stop for Envelope<S> {
    fn stop(&mut self) {
        self.adsr_mut().stop();
    }

    fn is_done(&self) -> bool {
        self.adsr().is_done()
    }
}

impl<S: HasFreq> HasFreq for Envelope<S> {
    fn freq(&self) -> Freq {
        self.sgn().freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn_mut().freq_mut()
    }
}
