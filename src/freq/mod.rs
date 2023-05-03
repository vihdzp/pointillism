//! Defines [`Freq`] and its basic methods.

pub mod boilerplate;

use crate::time::Time;

use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    num::{ParseIntError, TryFromIntError},
    ops::{Div, DivAssign, Mul, MulAssign},
    str::FromStr,
};

/// This magic number `69.0` corresponds to the MIDI index of A4.
pub const A4_MIDI: f64 = MidiNote::A4.note as f64;

/// Checked conversion from `f64` to `u8`.
///
/// ## Panics
///
/// This will panic if `x < 0.0` or `x >= 256.0`.
fn f64_to_u8(x: f64) -> u8 {
    if (0.0..256.0).contains(&x) {
        // We've done the check.
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        (x as u8)
    } else {
        panic!("value {x} not in range of u8")
    }
}

/// Checked conversion from `f64` to `u8`.
///
/// ## Panics
///
/// This will panic if `x < -128.0` or `x >= 128.0`.
fn f64_to_i8(x: f64) -> i8 {
    if (-128.0..128.0).contains(&x) {
        // We've done the check.
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        (x as i8)
    } else {
        panic!("value {x} not in range of i8")
    }
}

/// A MIDI note. Note that `C4 = 60`, `A4 = 69`.
///
/// We use a larger range than the MIDI specification, namely 0-255. This
/// shouldn't make much difference in practice, as note 135, at almost 20 kHz,
/// lies beyond the hearing range of most adults. More than this will exceed the
/// Nyquist rate at 44.1 kHz.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiNote {
    /// The MIDI note index.
    note: u8,
}

impl MidiNote {
    /// Initializes a new [`MidiNote`].
    #[must_use]
    pub const fn new(note: u8) -> Self {
        Self { note }
    }
}

/// We use `A4` as a default note.
impl Default for MidiNote {
    fn default() -> Self {
        Self::A4
    }
}

/// Converts a letter to a numeric note, from 0 to 11.
///
/// Returns `None` if anything other than a letter `A` - `G` is passed.
#[must_use]
pub const fn letter_to_note(letter: char) -> Option<u8> {
    match letter {
        'C' => Some(0),
        'D' => Some(2),
        'E' => Some(4),
        'F' => Some(5),
        'G' => Some(7),
        'A' => Some(9),
        'B' => Some(11),
        _ => None,
    }
}

/// Converts a numeric note to a letter, from 0 to 11.
///
/// For consistency, we use sharps and no flats in the letter names.
///
/// ## Panics
///
/// Panics if anything other than a number from 0 to 11 is passed.
#[must_use]
pub const fn note_to_letter(note: u8) -> &'static str {
    match note {
        0 => "C",
        1 => "C#",
        2 => "D",
        3 => "D#",
        4 => "E",
        5 => "F",
        6 => "F#",
        7 => "G",
        8 => "G#",
        9 => "A",
        10 => "A#",
        11 => "B",
        _ => panic!("invalid note"),
    }
}

/// An error in [`MidiNote::from_str`].
#[derive(Clone, Debug)]
pub enum NameError {
    /// The string is not at least two characters long.
    Short,

    /// The note is outside of the bounds for a [`MidiNote`].
    OutOfBounds,

    /// An invalid letter name for a note was read.
    ///
    /// Note that this is case-sensitive.
    Letter(char),

    /// The integer after the letter name could not be parsed.
    Parse(ParseIntError),
}

impl From<ParseIntError> for NameError {
    fn from(value: ParseIntError) -> Self {
        NameError::Parse(value)
    }
}

impl From<TryFromIntError> for NameError {
    fn from(_: TryFromIntError) -> Self {
        NameError::OutOfBounds
    }
}

impl Display for NameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Short => write!(f, "the string was too short"),
            Self::OutOfBounds => write!(f, "note is out of bounds (C-1 to D#20)"),
            Self::Letter(c) => write!(f, "letter {c} is invalid"),
            Self::Parse(err) => write!(f, "integer parsing error: {err}"),
        }
    }
}

impl std::error::Error for NameError {}

impl FromStr for MidiNote {
    type Err = NameError;

    fn from_str(name: &str) -> Result<Self, NameError> {
        let mut chars = name.chars();

        if let (Some(letter), Some(next)) = (chars.next(), chars.next()) {
            if let Some(note) = letter_to_note(letter) {
                // We do auxiliary calculations with higher range.
                let mut note = i16::from(note);

                let idx = match next {
                    '#' => {
                        note += 1;
                        2
                    }
                    'b' => {
                        note -= 1;
                        2
                    }
                    _ => 1,
                };

                note += 12 * (name[idx..].parse::<i16>()? + 1);
                Ok(MidiNote::new(note.try_into()?))
            } else {
                Err(NameError::Letter(letter))
            }
        } else {
            Err(NameError::Short)
        }
    }
}

impl Display for MidiNote {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let octave = isize::from(self.note / 12) - 1;
        write!(f, "{}{}", note_to_letter(self.note % 12), octave)
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

impl Display for Freq {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} Hz", self.hz())
    }
}

/// Relative pitch corresponding to a note in a given EDO or
/// [equal division of the octave](https://en.wikipedia.org/wiki/Equal_temperament).
#[must_use]
pub fn edo_note(edo: u16, note: f64) -> f64 {
    2f64.powf(note / f64::from(edo))
}

/// Relative pitch corresponding to a note in 12-EDO. See also [`edo_note`].
#[must_use]
pub fn note(note: f64) -> f64 {
    edo_note(12, note)
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

    /// Initializes a frequency in a given `edo` (equal division of the octave),
    /// a certain amount of `notes` above or below a `base` pitch (usually
    /// [`A4`](Freq::A4)).
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
        edo_note(edo, note) * base
    }

    /// Initializes a frequency in 12-EDO, a certain amount of `notes` above or
    /// below a `base` pitch (usually [`A4`](Freq::A4)).
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
    /// // The nearest note to `C5` is indeed `C5`.
    /// assert_eq!(Freq::C5.round_midi_with(Freq::A4), MidiNote::C5);
    /// ```
    #[must_use]
    pub fn round_midi_with(self, a4: Freq) -> MidiNote {
        MidiNote::new(f64_to_u8(self.round_midi_aux(a4).round()))
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
    /// // The nearest note to `C5` is indeed `C5`.
    /// assert_eq!(Freq::C5.round_midi(), MidiNote::C5);
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
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`MidiNote`].
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// let (note, semitones) = Freq::C5.midi_semitones_with(Freq::A4);
    ///
    /// // The nearest note to `C5` is indeed `C5`.
    /// assert_eq!(note, MidiNote::C5);
    /// assert!(semitones.abs() < 1e-7);
    /// ```
    #[must_use]
    pub fn midi_semitones_with(self, a4: Freq) -> (MidiNote, f64) {
        let note = self.round_midi_aux(a4);
        let round = note.round();
        (MidiNote::new(f64_to_u8(round)), note - round)
    }

    /// Rounds this frequency to the nearest MIDI note, and how many semitones away from this note
    /// it is.
    ///   
    /// See [`Self::midi_semitones_with`] in order to specify the `A4` tuning.
    ///
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`MidiNote`].
    ///
    /// ## Example
    /// ```
    /// # use pointillism::prelude::*;
    /// let (note, semitones) = Freq::C5.midi_semitones();
    ///
    /// // The nearest note to `C5` is indeed `C5`.
    /// assert_eq!(note, MidiNote::C5);
    /// assert!(semitones.abs() < 1e-7);
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

/// By default, we format a note as `"{hz} Hz"`. The alternate formatting mode results in `"{note}
/// {cents}c"`.
impl Debug for Freq {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            let (note, semitones) = self.midi_semitones();
            let cents = f64_to_i8(semitones * 100.0);
            write!(f, "{note} {cents:+}c")
        } else {
            write!(f, "{} Hz", self.hz())
        }
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

impl Div<Freq> for Freq {
    type Output = f64;

    fn div(self, rhs: Freq) -> f64 {
        self.hz / rhs.hz
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
