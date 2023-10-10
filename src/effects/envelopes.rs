//! Signal envelopes, notably including the [`AdsrEnv`].

use crate::prelude::*;

/// Any of the stages in an AR envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArStage {
    /// Start to peak.
    Attack,
    /// Peak to done.
    Release,
    /// Done.
    Done,
}

/// An AR envelope. Outputs values between `0.0` and `1.0`.
///
/// ```txt
///        ⟋\
///      ⟋   \
///  A ⟋      \ R
///  ⟋         \
/// •―――――――――――•  [DC = 0]
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Ar {
    /// The time from the signal start to its peak.
    pub attack: unt::Time,
    /// The time from the signal stop to it being done.
    pub release: unt::Time,

    /// Current stage of the envelope.
    stage: ArStage,

    /// The volume from which the release starts.
    ///
    /// This can differ from full volume if the envelope is stopped before the `Release` phase.
    release_vol: unt::Vol,

    /// How many frames have we spent in this phase?
    phase_time: unt::Time,
}

impl Ar {
    /// Initializes a new [`Ar`] envelope.
    #[must_use]
    pub fn new(attack: unt::Time, release: unt::Time) -> Self {
        let mut ar = Self {
            attack,
            release,
            stage: ArStage::Attack,
            phase_time: unt::Time::ZERO,

            // Is properly initialized in `stop`, or when the release phase starts.
            release_vol: unt::Vol::ZERO,
        };

        // Skip any stages with zero time.
        ar.set_stage();
        ar
    }

    /// Current stage of the envelope.
    #[must_use]
    pub fn stage(&self) -> ArStage {
        self.stage
    }

    /// Sets our stage to the correct one, based on the elapsed time.
    ///
    /// This should work even if some phases take zero time.
    fn set_stage(&mut self) {
        if self.stage == ArStage::Attack && self.phase_time >= self.attack {
            self.stage = ArStage::Release;
            self.release_vol = unt::Vol::FULL;
            self.phase_time -= self.attack;
        }

        if self.stage == ArStage::Release && self.phase_time >= self.release {
            self.stage = ArStage::Done;
        }
    }
}

impl Signal for Ar {
    type Sample = smp::Env;

    fn get(&self) -> smp::Env {
        // Division by zero should not be possible, as phases with length zero are immediately
        // skipped.
        smp::Env(match self.stage() {
            ArStage::Attack => self.phase_time / self.attack,
            ArStage::Release => self.release_vol.gain * (1.0 - self.phase_time / self.release),
            ArStage::Done => 0.0,
        })
    }
}

impl SignalMut for Ar {
    fn advance(&mut self) {
        self.phase_time.advance();
        self.set_stage();
    }

    fn retrigger(&mut self) {
        self.stage = ArStage::Attack;
        self.phase_time = unt::Time::ZERO;
    }
}

impl Done for Ar {
    fn is_done(&self) -> bool {
        self.stage == ArStage::Done
    }
}

impl Stop for Ar {
    fn stop(&mut self) {
        self.release_vol = unt::Vol::new(self.get().0);
        self.phase_time = unt::Time::ZERO;
        self.stage = ArStage::Release;
    }
}

impl Panic for Ar {
    fn panic(&mut self) {
        self.stage = ArStage::Done;
    }
}

/// An envelope with attack and release.
///
/// Initialize with [`Self::new_ar`].
pub type ArEnv<S> = eff::StopTremolo<S, Ar>;

impl<S: SignalMut> ArEnv<S> {
    /// Initializes an [`ArEnv`] with the given parameters.
    pub fn new_ar(sgn: S, ar: Ar) -> Self {
        Self::new(sgn, ar)
    }
}

/// Any of the stages in an ADSR envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdsrStage {
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

impl From<ArStage> for AdsrStage {
    fn from(value: ArStage) -> Self {
        match value {
            ArStage::Attack => Self::Attack,
            ArStage::Release => Self::Release,
            ArStage::Done => Self::Done,
        }
    }
}

/// An ADSR envelope. Outputs values between `0.0` and `1.0`.
///
/// The following diagram shows the four phases of an ADSR. Note that the release phase is only
/// activated once the envelope is [stopped](Stop).
///
/// ```txt
///        ⟋⟍ D
///      ⟋    ⟍   S
///  A ⟋        •――――•
///  ⟋                \ R
/// •――――――――――――――――――•  [DC = 0]
/// ```
///
/// If you don't care for the sustain, you might instead want to use an [`ArEnvelope`].
#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    /// The time from the signal start to its peak.
    pub attack: unt::Time,
    /// The time from the signal peak to its sustain point.
    pub decay: unt::Time,
    /// The sustain value for the signal.
    pub sustain: unt::Vol,
    /// The time from the signal stop to it being done.
    pub release: unt::Time,

    /// Current stage of the envelope.
    stage: AdsrStage,

    /// The volume from which the release starts.
    ///
    /// This can differ from the sustain value if the envelope is stopped before the `Sustain`
    /// phase.
    release_vol: unt::Vol,

    /// How many frames have we spent in this phase?
    phase_time: unt::Time,
}

impl Adsr {
    /// Initializes a new [`Adsr`] envelope.
    #[must_use]
    pub fn new(attack: unt::Time, decay: unt::Time, sustain: unt::Vol, release: unt::Time) -> Self {
        let mut adsr = Self {
            attack,
            decay,
            sustain,
            release,
            stage: AdsrStage::Attack,
            phase_time: unt::Time::ZERO,

            // Is properly initialized in `stop`.
            release_vol: unt::Vol::ZERO,
        };

        // Skip any stages with zero time.
        adsr.set_stage();
        adsr
    }

    /// Current stage of the envelope.
    #[must_use]
    pub fn stage(&self) -> AdsrStage {
        self.stage
    }

    /// Sets our stage to the correct one, based on the elapsed time.
    ///
    /// This should work even if some phases take zero time.
    fn set_stage(&mut self) {
        if self.stage == AdsrStage::Attack && self.phase_time >= self.attack {
            self.stage = AdsrStage::Decay;
            self.phase_time -= self.attack;
        }

        if self.stage == AdsrStage::Decay && self.phase_time >= self.decay {
            self.stage = AdsrStage::Sustain;
            self.phase_time -= self.decay;
        }

        if self.stage == AdsrStage::Release && self.phase_time >= self.release {
            self.stage = AdsrStage::Done;
        }
    }
}

impl Signal for Adsr {
    type Sample = smp::Env;

    fn get(&self) -> smp::Env {
        // Division by zero should not be possible, as phases with length zero are immediately
        // skipped.
        smp::Env(match self.stage() {
            AdsrStage::Attack => self.phase_time / self.attack,
            AdsrStage::Decay => 1.0 + (self.sustain.gain - 1.0) * (self.phase_time / self.decay),
            AdsrStage::Sustain => self.sustain.gain,
            AdsrStage::Release => self.release_vol.gain * (1.0 - self.phase_time / self.release),
            AdsrStage::Done => 0.0,
        })
    }
}

impl SignalMut for Adsr {
    fn advance(&mut self) {
        self.phase_time.advance();
        self.set_stage();
    }

    fn retrigger(&mut self) {
        self.stage = AdsrStage::Attack;
        self.phase_time = unt::Time::ZERO;
    }
}

impl Done for Adsr {
    fn is_done(&self) -> bool {
        self.stage == AdsrStage::Done
    }
}

impl Stop for Adsr {
    fn stop(&mut self) {
        self.release_vol = unt::Vol::new(self.get().0);
        self.phase_time = unt::Time::ZERO;
        self.stage = AdsrStage::Release;
    }
}

impl Panic for Adsr {
    fn panic(&mut self) {
        self.stage = AdsrStage::Done;
    }
}

/// Hooks up a signal to an [`Adsr`] envelope.
///
/// Initialize with [`Self::new_adsr`].
pub type AdsrEnv<S> = eff::StopTremolo<S, Adsr>;

impl<S: SignalMut> AdsrEnv<S> {
    /// Initializes an [`AdsrEnv`] with the given parameters.
    pub fn new_adsr(sgn: S, adsr: Adsr) -> Self {
        Self::new(sgn, adsr)
    }
}
