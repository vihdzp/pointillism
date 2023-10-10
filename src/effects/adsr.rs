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
/// You might instead want to use an [`ArEnvelope`].
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
    stage: Stage,

    /// The value from which the sustain starts.
    ///
    /// This can differ from the sustain value if the envelope is stopped before the `Sustain`
    /// phase.
    sustain_val: unt::Vol,

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
            stage: Stage::Attack,
            phase_time: unt::Time::ZERO,

            // Is properly initialized in `stop`.
            sustain_val: unt::Vol::ZERO,
        };

        // Skip any stages with zero time.
        adsr.set_stage();
        adsr
    }

    /// Current stage of the envelope.
    #[must_use]
    pub fn stage(&self) -> Stage {
        self.stage
    }

    /// Sets our stage to the correct one, based on the elapsed time.
    ///
    /// This should work even if some phases take zero time.
    fn set_stage(&mut self) {
        if self.stage == Stage::Attack && self.phase_time >= self.attack {
            self.stage = Stage::Decay;
            self.phase_time -= self.attack;
        }

        if self.stage == Stage::Decay && self.phase_time >= self.decay {
            self.stage = Stage::Sustain;
            self.phase_time -= self.decay;
        }

        if self.stage == Stage::Release && self.phase_time >= self.release {
            self.stage = Stage::Done;
        }
    }
}

impl Signal for Adsr {
    type Sample = smp::Env;

    fn get(&self) -> smp::Env {
        // Division by zero should not be possible, as phases with length zero are immediately
        // skipped.
        smp::Env(match self.stage() {
            Stage::Attack => self.phase_time / self.attack,
            Stage::Decay => 1.0 + (self.sustain.gain - 1.0) * (self.phase_time / self.decay),
            Stage::Sustain => self.sustain.gain,
            Stage::Release => self.sustain_val.gain * (1.0 - self.phase_time / self.release),
            Stage::Done => 0.0,
        })
    }
}

impl SignalMut for Adsr {
    fn advance(&mut self) {
        self.phase_time.advance();
        self.set_stage();
    }

    fn retrigger(&mut self) {
        self.stage = Stage::Attack;
        self.phase_time = unt::Time::ZERO;
    }
}

impl Done for Adsr {
    fn is_done(&self) -> bool {
        self.stage == Stage::Done
    }
}

impl Stop for Adsr {
    fn stop(&mut self) {
        self.sustain_val = unt::Vol::new(self.get().0);
        self.phase_time = unt::Time::ZERO;
        self.stage = Stage::Release;
    }
}

impl Panic for Adsr {
    fn panic(&mut self) {
        self.stage = Stage::Done;
    }
}

/// Hooks up a signal to an [`Adsr`] envelope.
///
/// Initialize with [`Self::new_adsr`].
pub type AdsrEnvelope<S> = StopTremolo<S, Adsr>;

impl<S: SignalMut> AdsrEnvelope<S> {
    /// Initializes an [`AdsrEnvelope`] with the given parameters.
    pub fn new_adsr(
        sgn: S,
        attack: unt::Time,
        decay: unt::Time,
        sustain: unt::Vol,
        release: unt::Time,
    ) -> Self {
        Self::new(sgn, Adsr::new(attack, decay, sustain, release))
    }
}
