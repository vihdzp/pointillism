//! Defines [`Freq`] and its basic methods.

use crate::time::Time;

use std::{
    error::Error,
    fmt::Display,
    num::ParseIntError,
    ops::{Div, DivAssign, Mul, MulAssign},
};

/// Represents a frequency.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Freq {
    /// The frequency in hertz.
    pub hz: f64,
}

/// We use `A4` as a default frequency. This means that, for instance,
///
/// ```
/// # use pointillism::prelude::*;
/// let osc = CurveGen::<Sin>::default();
/// ```
///
/// will result in a 440 Hz sine wave.
impl Default for Freq {
    fn default() -> Self {
        crate::A4
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

/// An error in [`Freq::new_name`].
#[derive(Clone, Debug)]
pub enum NameError {
    /// The string is not at least two characters long.
    Short,

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

impl Display for NameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "the string was too short"),
            Self::Letter(c) => write!(f, "letter {c} is invalid"),
            Self::Parse(err) => write!(f, "integer parsing error: {err}"),
        }
    }
}

impl Error for NameError {}

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
    /// [`A4`](crate::A4)).
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// // C5 is 3 semitones above A4.
    /// let C5 = Freq::new_edo_note(A4, 12, 3.0);
    /// ```
    #[must_use]
    pub fn new_edo_note(base: Freq, edo: u16, note: f64) -> Self {
        edo_note(edo, note) * base
    }

    /// Initializes a frequency in 12-EDO, a certain amount of `notes` above or
    /// below a `base` pitch (usually [`A4`](crate::A4)).
    ///
    /// See also [`Freq::new_edo_note`].
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// // C5 is 3 semitones above A4.
    /// let C5 = Freq::new_note(A4, 3.0);
    /// ```
    #[must_use]
    pub fn new_note(base: Freq, note: f64) -> Self {
        Self::new_edo_note(base, 12, note)
    }

    /// Initializes a frequency from a MIDI note.
    #[must_use]
    pub fn new_midi(a4: Freq, note: i16) -> Self {
        Self::new_edo_note(a4, 12, f64::from(note) - 69.0)
    }

    /// Initializes a pitch from its name (e.g. `"A4"` or `"G#5"`).
    ///
    /// ## Errors
    ///
    /// This function can return an error in the following circumstances:
    ///
    /// - The string is not at least two characters long.
    /// - The string doesn't start with a letter `A` - `G`.
    /// - An integer (`i16`) could not be parsed after the letter.
    pub fn new_name(a4: Freq, name: &str) -> Result<Self, NameError> {
        let mut chars = name.chars();

        if let (Some(letter), Some(next)) = (chars.next(), chars.next()) {
            if let Some(note) = letter_to_note(letter) {
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
                Ok(Self::new_midi(a4, note))
            } else {
                Err(NameError::Letter(letter))
            }
        } else {
            Err(NameError::Short)
        }
    }
}

impl From<Freq> for Time {
    fn from(value: Freq) -> Self {
        value.period()
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
