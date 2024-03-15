//! Defines the type [`RawFreq`].
//!
//! This type measures frequency in the natural unit of
//! [hertz](https://en.wikipedia.org/wiki/Hertz). However, in order to use actually use it for most
//! things, you'll need to convert it into [`Freq`], which is measured in inverse samples.
//!
//! ## Constants
//!
//! We initialize constants for [`RawFreq`] and [`MidiNote`](unt::MidiNote). For instance,
//! [`RawFreq::A4`] = 440 Hz.
//!
//! Each name is made out of a pitch letter, followed by an optional `S` or `B` for sharps and
//! flats, followed by the octave number. We use `N` for a negative sign.
//!
//! Enharmonic notes are given their individual constant names, for good measure.
//!
//! We only implement the notes from octaves -1 to 10, as anything lower is unsupported as a
//! (standard) MIDI note, and anything higher is too high-pitched to be practical. This range
//! well-covers the human hearing range.

use crate::{prelude::*, units::A4_MIDI};

use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

/// Represents a frequency in **hertz**. Must be positive.
///
/// Most methods will require a [`unt::Freq`] instead, which is dependent on your sample rate. See
/// [`unt::Freq::from_raw`].
///
/// Not to be confused with [`Frequency`](crate::signal::Frequency).
///
/// ## Type invariant checking
///
/// It's impractical to constantly check that frequencies are positive, and there's not really any
/// simple ways to mess this up, so we don't check the invariant. That's not to say things won't go
/// wrong if the invariant is broken!
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RawFreq {
    /// The frequency in hertz.
    pub hz: f64,
}

/// We use `A4` as a default frequency. This means that, for instance,
///
/// ```
/// # use pointillism::prelude::*;
/// let osc = gen::Loop::<smp::Mono, crv::Sin>::default();
/// ```
///
/// will result in a 440 Hz sine wave when sampled at 44.1 kHz.
impl Default for RawFreq {
    fn default() -> Self {
        RawFreq::A4
    }
}

/// The alternate formatting mode results in `"{note} {cents}c"`.
impl Display for RawFreq {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            let (note, semitones) = self.midi_semitones();
            let cents = semitones * 100.0;

            // Precision defaults to 0.
            write!(f, "{note} {cents:+.*}c", f.precision().unwrap_or_default())
        } else {
            write!(f, "{} Hz", self.hz)
        }
    }
}

impl RawFreq {
    /// Initializes a given frequency.
    ///
    /// Note that the frequency will generally be assumed to be positive.
    #[must_use]
    pub const fn new(hz: f64) -> Self {
        Self { hz }
    }

    pointillism_macros::freq!();

    /// The period, which equals the reciprocal of the frequency.
    #[must_use]
    pub fn period(&self) -> unt::RawTime {
        unt::RawTime::new(1.0 / self.hz)
    }

    /// Bends a note by a number of notes in a given `edo`.
    ///
    /// You can use this to generate an scale in some EDO, based on some note.
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// # use assert_approx_eq::assert_approx_eq;
    /// // C5 is 3 semitones above A4.
    /// let c5 = unt::RawFreq::A4.bend_edo(12, 3.0);
    /// assert_approx_eq!(c5.hz, unt::RawFreq::C5.hz);
    /// ```
    #[must_use]
    pub fn bend_edo(self, edo: u16, bend: f64) -> Self {
        unt::Interval::edo_note(edo, bend) * self
    }

    /// Bends a note by a number of notes in 12-EDO.
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// # use assert_approx_eq::assert_approx_eq;
    /// // C5 is 3 semitones above A4.
    /// let c5 = unt::RawFreq::A4.bend(3.0);
    /// assert_approx_eq!(c5.hz, unt::RawFreq::C5.hz);
    /// ```
    #[must_use]
    pub fn bend(self, bend: f64) -> Self {
        self.bend_edo(12, bend)
    }

    /// Initializes a frequency from a MIDI note.
    ///
    /// This allows the user to specify the `A4` tuning. Use [`Self::new_midi`] for the default of
    /// 440 Hz.
    #[must_use]
    pub fn new_midi_with(a4: Self, note: unt::MidiNote) -> Self {
        a4.bend(f64::from(note.note) - A4_MIDI)
    }

    /// Initializes a frequency from a MIDI note.
    ///
    /// This assumes A4 = 440 Hz. See [`Self::new_midi_with`] in order to specify the `A4` tuning.
    #[must_use]
    pub fn new_midi(note: unt::MidiNote) -> Self {
        Self::new_midi_with(RawFreq::A4, note)
    }

    /// Rounds this frequency to the nearest (fractional) MIDI note.
    #[must_use]
    fn round_midi_aux(self, a4: Self) -> f64 {
        (self.hz / a4.hz).log2() * 12.0 + A4_MIDI
    }

    /// Rounds this frequency to the nearest MIDI note.
    ///
    /// This allows the user to specify the `A4` tuning. Use [`Self::round_midi`] for the default of
    /// 440 Hz.
    ///
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`unt::MidiNote`].
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = unt::RawFreq::A4.bend(0.6);
    ///
    /// // The nearest note is `A#4`.
    /// assert_eq!(freq.round_midi_with(unt::RawFreq::A4), unt::MidiNote::AS4);
    /// ```
    #[must_use]
    pub fn round_midi_with(self, a4: Self) -> unt::MidiNote {
        // Truncation should not occur in practice.
        #[allow(clippy::cast_possible_truncation)]
        unt::MidiNote::new((self.round_midi_aux(a4).round()) as i16)
    }

    /// Rounds this frequency to the nearest MIDI note.
    ///
    /// See [`Self::new_midi_with`] in order to specify the `A4` tuning.
    ///
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`unt::MidiNote`].
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = unt::RawFreq::A4.bend(0.6);
    ///
    /// // The nearest note is `A#4`.
    /// assert_eq!(freq.round_midi(), unt::MidiNote::AS4);
    /// ```
    #[must_use]
    pub fn round_midi(self) -> unt::MidiNote {
        self.round_midi_with(RawFreq::A4)
    }

    /// Rounds this frequency to the nearest MIDI note, and how many semitones away from this note
    /// it is.
    ///
    /// This allows the user to specify the `A4` tuning. Use [`Self::midi_semitones`] for the
    /// default of 440 Hz.
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = unt::RawFreq::A4.bend(0.6);
    /// let (note, semitones) = freq.midi_semitones_with(unt::RawFreq::A4);
    ///
    /// // The nearest note is `A#4`, and it's -40 cents from it.
    /// assert_eq!(note, unt::MidiNote::AS4);
    /// assert!((semitones + 0.4).abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn midi_semitones_with(self, a4: Self) -> (unt::MidiNote, f64) {
        let note = self.round_midi_aux(a4);
        let round = note.round();

        // Truncation should not occur in practice.
        #[allow(clippy::cast_possible_truncation)]
        (unt::MidiNote::new(round as i16), note - round)
    }

    /// Rounds this frequency to the nearest MIDI note, and how many semitones away from this note
    /// it is.
    ///   
    /// See [`Self::midi_semitones_with`] in order to specify the `A4` tuning.
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// # use assert_approx_eq::assert_approx_eq;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = unt::RawFreq::A4.bend(0.6);
    /// let (note, semitones) = freq.midi_semitones();
    ///
    /// // The nearest note is `A#4`, and it's -40 cents from it.
    /// assert_eq!(note, unt::MidiNote::AS4);
    /// assert_approx_eq!(semitones, -0.4);
    /// ```
    #[must_use]
    pub fn midi_semitones(self) -> (unt::MidiNote, f64) {
        self.midi_semitones_with(RawFreq::A4)
    }
}

/// We use A4 = 440 Hz.
impl From<unt::MidiNote> for RawFreq {
    fn from(note: unt::MidiNote) -> Self {
        Self::new_midi(note)
    }
}

/// Initializes a [`RawFreq`] from a note name.
impl FromStr for RawFreq {
    type Err = crate::units::midi::NameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        unt::MidiNote::from_str(name).map(RawFreq::from)
    }
}

impl Mul<f64> for RawFreq {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.hz * rhs)
    }
}

impl Mul<RawFreq> for f64 {
    type Output = RawFreq;

    fn mul(self, rhs: RawFreq) -> RawFreq {
        rhs * self
    }
}

impl MulAssign<f64> for RawFreq {
    fn mul_assign(&mut self, rhs: f64) {
        self.hz *= rhs;
    }
}

impl Div<f64> for RawFreq {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.hz / rhs)
    }
}

impl DivAssign<f64> for RawFreq {
    fn div_assign(&mut self, rhs: f64) {
        self.hz /= rhs;
    }
}

impl Mul<unt::Interval> for RawFreq {
    type Output = Self;

    fn mul(self, rhs: unt::Interval) -> Self {
        rhs.ratio * self
    }
}

impl Mul<RawFreq> for unt::Interval {
    type Output = RawFreq;

    fn mul(self, rhs: RawFreq) -> RawFreq {
        self.ratio * rhs
    }
}

impl MulAssign<unt::Interval> for RawFreq {
    fn mul_assign(&mut self, rhs: unt::Interval) {
        self.hz *= rhs.ratio;
    }
}

impl Div<unt::Interval> for RawFreq {
    type Output = Self;

    fn div(self, rhs: unt::Interval) -> Self {
        Self::new(self.hz / rhs.ratio)
    }
}

impl DivAssign<unt::Interval> for RawFreq {
    fn div_assign(&mut self, rhs: unt::Interval) {
        self.hz /= rhs.ratio;
    }
}

impl Div for RawFreq {
    type Output = unt::Interval;

    fn div(self, rhs: Self) -> unt::Interval {
        unt::Interval::new(self.hz / rhs.hz)
    }
}
