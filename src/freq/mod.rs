//! Defines [`Freq`] and its basic methods.
//!
//! ## Equal division of the octave
//!
//! TODO: write something here
//! [equal division of the octave](https://en.wikipedia.org/wiki/Equal_temperament)

pub mod boilerplate;
pub mod midi;

use crate::time::Time;

use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

use self::midi::{MidiNote, NameError, A4_MIDI};

/// Represents an interval, or a ratio between notes.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Interval {
    /// The ratio in question.
    pub ratio: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self::UNISON
    }
}

impl Interval {
    /// Initializes a new ratio.
    #[must_use]
    pub const fn new(ratio: f64) -> Self {
        Self { ratio }
    }

    /// Relative pitch corresponding to a note in a given EDO.
    #[must_use]
    pub fn edo_note(edo: u16, note: f64) -> Self {
        Self::new(2f64.powf(note / f64::from(edo)))
    }

    /// Relative pitch corresponding to a note in 12-EDO.
    ///
    /// See also [`edo_note`].
    #[must_use]
    pub fn note(note: f64) -> Self {
        Self::edo_note(12, note)
    }

    /// Returns the inverse ratio.
    #[must_use]
    pub fn inv(self) -> Self {
        Self::new(1.0 / self.ratio)
    }

    /// Takes the square root of an interval.
    ///
    /// For instance, a 12-EDO tritone is exactly the square root of an octave.
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self::new(self.ratio.sqrt())
    }

    /// Raises an interval to an integer power.
    #[must_use]
    pub fn powi(self, n: i32) -> Self {
        Self::new(self.ratio.powi(n))
    }

    /// Raises an interval to a floating point power.
    #[must_use]
    pub fn powf(self, n: f64) -> Self {
        Self::new(self.ratio.powf(n))
    }
}

/// Represents a frequency.
///
/// Not to be confused with [`Frequency`](crate::signal::Frequency).
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Freq {
    /// The frequency in hertz.
    pub hz: f64,
}

/// We use `A4` as a default frequency. This means that, for instance,
///
/// ```
/// # use pointillism::prelude::*;
/// let osc = LoopGen::<Mono, Sin>::default();
/// ```
///
/// will result in a 440 Hz sine wave.
impl Default for Freq {
    fn default() -> Self {
        Freq::A4
    }
}

/// The alternate formatting mode results in `"{note} {cents}c"`.
impl Debug for Freq {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if f.alternate() {
            let (note, semitones) = self.midi_semitones();

            // Truncation should be impossible.
            #[allow(clippy::cast_possible_truncation)]
            let cents = (semitones * 100.0) as i16;
            write!(f, "{note} {cents:+}c")
        } else {
            f.debug_struct("Freq").field("hz", &self.hz).finish()
        }
    }
}

impl Display for Freq {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} Hz", self.hz())
    }
}

impl Freq {
    /// Initializes a given frequency.
    ///
    /// Note that the frequency will generally be assumed to be positive.
    #[must_use]
    pub const fn new(hz: f64) -> Self {
        Self { hz }
    }

    /// The frequency in Hertz.
    #[must_use]
    pub const fn hz(&self) -> f64 {
        self.hz
    }

    /// The period, which equals the reciprocal of the frequency.
    #[must_use]
    pub fn period(&self) -> Time {
        Time::new(1.0 / self.hz())
    }

    /// Initializes a frequency a certain amount of `notes` in a given `edo` above or below a `base`
    /// pitch (usually [`A4`](Freq::A4)).
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// // C5 is 3 semitones above A4.
    /// let C5 = Freq::new_edo_note(Freq::A4, 12, 3.0);
    /// assert!((C5.hz - Freq::C5.hz).abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn new_edo_note(base: Freq, edo: u16, note: f64) -> Self {
        Interval::edo_note(edo, note) * base
    }

    /// Initializes a frequency a certain amount of `notes` in 12-EDO above or below a `base` pitch
    /// (usually [`A4`](Freq::A4)).
    ///
    /// See also [`Freq::new_edo_note`].
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// // C5 is 3 semitones above A4.
    /// let C5 = Freq::new_note(Freq::A4, 3.0);
    /// assert!((C5.hz - Freq::C5.hz).abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn new_note(base: Freq, note: f64) -> Self {
        Self::new_edo_note(base, 12, note)
    }

    /// Bends a note by a number of notes in a given `edo`.
    #[must_use]
    pub fn bend_edo(self, edo: u16, bend: f64) -> Self {
        Interval::edo_note(edo, bend) * self
    }

    /// Bends a note by a number of notes in 12-EDO.
    #[must_use]
    pub fn bend(self, bend: f64) -> Self {
        self.bend_edo(12, bend)
    }

    /// Initializes a frequency from a MIDI note.
    ///
    /// This allows the user to specify the `A4` tuning. Use [`Self::new_midi`] for the default of
    /// 440 Hz.
    #[must_use]
    pub fn new_midi_with(a4: Freq, note: MidiNote) -> Self {
        Self::new_edo_note(a4, 12, f64::from(note.note) - A4_MIDI)
    }

    /// Initializes a frequency from a MIDI note.
    ///
    /// See [`Self::new_midi_with`] in order to specify the `A4` tuning.
    #[must_use]
    pub fn new_midi(note: MidiNote) -> Self {
        Self::new_midi_with(Freq::A4, note)
    }

    /// Rounds this frequency to the nearest (fractional) MIDI note.
    #[must_use]
    fn round_midi_aux(self, a4: Freq) -> f64 {
        (self.hz() / a4.hz()).log2() * 12.0 + A4_MIDI
    }

    /// Rounds this frequency to the nearest MIDI note.
    ///
    /// This allows the user to specify the `A4` tuning. Use [`Self::round_midi`] for the default of
    /// 440 Hz.
    ///
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`MidiNote`].
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = Freq::A4.bend(0.6);
    ///
    /// // The nearest note is `A#4`.
    /// assert_eq!(freq.round_midi_with(Freq::A4), MidiNote::AS4);
    /// ```
    #[must_use]
    pub fn round_midi_with(self, a4: Freq) -> MidiNote {
        // Truncation should not occur in practice.
        #[allow(clippy::cast_possible_truncation)]
        MidiNote::new((self.round_midi_aux(a4).round()) as i16)
    }

    /// Rounds this frequency to the nearest MIDI note.
    ///
    /// See [`Self::new_midi_with`] in order to specify the `A4` tuning.
    ///
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`MidiNote`].
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = Freq::A4.bend(0.6);
    ///
    /// // The nearest note is `A#4`.
    /// assert_eq!(freq.round_midi(), MidiNote::AS4);
    /// ```
    #[must_use]
    pub fn round_midi(self) -> MidiNote {
        self.round_midi_with(Freq::A4)
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
    /// let freq = Freq::A4.bend(0.6);
    /// let (note, semitones) = freq.midi_semitones_with(Freq::A4);
    ///
    /// // The nearest note is `A#4`, and it's -40 cents from it.
    /// assert_eq!(note, MidiNote::AS4);
    /// assert!((semitones + 0.4).abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn midi_semitones_with(self, a4: Freq) -> (MidiNote, f64) {
        let note = self.round_midi_aux(a4);
        let round = note.round();

        // Truncation should not occur in practice.
        #[allow(clippy::cast_possible_truncation)]
        (MidiNote::new(round as i16), note - round)
    }

    /// Rounds this frequency to the nearest MIDI note, and how many semitones away from this note
    /// it is.
    ///   
    /// See [`Self::midi_semitones_with`] in order to specify the `A4` tuning.
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// // Pitch-bend A4 by 60 cents.
    /// let freq = Freq::A4.bend(0.6);
    /// let (note, semitones) = freq.midi_semitones();
    ///
    /// // The nearest note is `A#4`, and it's -40 cents from it.
    /// assert_eq!(note, MidiNote::AS4);
    /// assert!((semitones + 0.4).abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn midi_semitones(self) -> (MidiNote, f64) {
        self.midi_semitones_with(Freq::A4)
    }
}

/// We use A4 = 440 Hz.
impl From<MidiNote> for Freq {
    fn from(note: MidiNote) -> Self {
        Self::new_midi(note)
    }
}

impl FromStr for Freq {
    type Err = NameError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        MidiNote::from_str(name).map(Freq::from)
    }
}

impl Mul<f64> for Freq {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Freq {
    fn mul_assign(&mut self, rhs: f64) {
        self.hz *= rhs;
    }
}

impl Div<f64> for Freq {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.hz / rhs)
    }
}

impl DivAssign<f64> for Freq {
    fn div_assign(&mut self, rhs: f64) {
        self.hz /= rhs;
    }
}

impl Mul<Interval> for Freq {
    type Output = Self;

    fn mul(self, rhs: Interval) -> Self::Output {
        rhs.ratio * self
    }
}

impl Mul<Freq> for Interval {
    type Output = Freq;

    fn mul(self, rhs: Freq) -> Self::Output {
        self.ratio * rhs
    }
}

impl MulAssign<Interval> for Freq {
    fn mul_assign(&mut self, rhs: Interval) {
        self.hz *= rhs.ratio;
    }
}

impl Div<Interval> for Freq {
    type Output = Self;

    fn div(self, rhs: Interval) -> Self {
        Self::new(self.hz / rhs.ratio)
    }
}

impl DivAssign<Interval> for Freq {
    fn div_assign(&mut self, rhs: Interval) {
        self.hz /= rhs.ratio;
    }
}

impl Div for Freq {
    type Output = Interval;

    fn div(self, rhs: Freq) -> Interval {
        Interval::new(self.hz / rhs.hz)
    }
}

impl Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Interval) -> Self {
        Self::new(self.ratio * rhs.ratio)
    }
}

impl MulAssign for Interval {
    fn mul_assign(&mut self, rhs: Self) {
        self.ratio *= rhs.ratio;
    }
}

impl Div for Interval {
    type Output = Self;

    fn div(self, rhs: Interval) -> Self {
        Self::new(self.ratio / rhs.ratio)
    }
}

impl DivAssign for Interval {
    fn div_assign(&mut self, rhs: Self) {
        self.ratio /= rhs.ratio;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_a4() {
        assert_eq!(format!("{:#?}", Freq::A4), "A4 +0c");
    }

    #[test]
    fn parse_a4() {
        let a4: Freq = "A4".parse().unwrap();
        assert!((a4.hz - Freq::A4.hz).abs() < 1e-7);
    }
}
