use std::marker::PhantomData;

use crate::{
    sample::Env,
    signal::{Signal, StopSignal},
    MapMut, Time,
};

use super::{Envelope, Volume};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdsrPhase {
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

    /// Current phase of the envelope.
    phase: AdsrPhase,

    /// A value from `0.0` to `1.0` representing how far along the phase we are.
    val: f64,
}

impl Adsr {
    pub fn new(attack: Time, decay: Time, sustain: f64, release: Time) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            phase: AdsrPhase::Attack,
            val: 0.0,
        }
    }

    pub fn phase(&self) -> AdsrPhase {
        self.phase
    }
}

impl Signal for Adsr {
    type Sample = Env;

    fn get(&self) -> Env {
        Env(match self.phase() {
            AdsrPhase::Attack => self.val,
            AdsrPhase::Decay => 1.0 + (self.sustain - 1.0) * self.val,
            AdsrPhase::Sustain => self.sustain,
            AdsrPhase::Release => self.sustain * (1.0 - self.val),
            AdsrPhase::Done => 0.0,
        })
    }

    fn advance(&mut self) {
        match self.phase() {
            AdsrPhase::Attack => {
                self.val += 1.0 / self.attack.frames();

                if self.val > 1.0 {
                    self.phase = AdsrPhase::Decay;
                    self.val -= 1.0;
                }
            }

            AdsrPhase::Decay => {
                self.val += 1.0 / self.decay.frames();

                if self.val > 1.0 {
                    self.phase = AdsrPhase::Sustain;
                    self.val = 0.0;
                }
            }

            AdsrPhase::Release => {
                self.val += 1.0 / self.release.frames();

                if self.val > 1.0 {
                    self.phase = AdsrPhase::Done;
                }
            }

            AdsrPhase::Sustain | AdsrPhase::Done => {}
        }
    }

    fn retrigger(&mut self) {
        self.phase = AdsrPhase::Attack;
        self.val = 0.0;
    }
}

impl StopSignal for Adsr {
    fn stop(&mut self) {
        self.sustain = self.get().0;
        self.val = 0.0;
        self.phase = AdsrPhase::Release;
    }

    fn is_done(&self) -> bool {
        self.phase == AdsrPhase::Done
    }
}

/// The function that changes the volume of a [`Volume<S>`] envelope.
#[derive(Clone, Copy, Debug)]
pub struct VolFn<S: Signal> {
    /// Dummy value.
    phantom: PhantomData<Volume<S>>,
}

impl<S: Signal> Default for VolFn<S> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<S: Signal> VolFn<S> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Signal> MapMut<Volume<S>, f64> for VolFn<S> {
    fn modify(&self, sgn: &mut Volume<S>, gain: f64) {
        *sgn.gain_mut() = gain;
    }
}

/// A stoppable signal, hooked to an ADSR envelope.
#[derive(Clone, Debug)]
pub struct AdsrEnvelope<S: Signal> {
    /// The inner envelope.
    pub env: Envelope<Volume<S>, Adsr, VolFn<S>>,
}

impl<S: Signal> AdsrEnvelope<S> {
    /// Initializes a new ADSR envelope.
    pub fn new(sgn: S, env: Adsr) -> Self {
        Self {
            env: Envelope::new(Volume::new(sgn, 0.0), env, VolFn::new()),
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

impl<S: Signal> Signal for AdsrEnvelope<S> {
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

impl<S: Signal> StopSignal for AdsrEnvelope<S> {
    fn stop(&mut self) {
        self.adsr_mut().stop();
    }

    fn is_done(&self) -> bool {
        self.adsr().is_done()
    }
}
