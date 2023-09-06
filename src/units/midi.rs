//! Defines the type for a [`MidiNote`], and its basic methods.

use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    num::ParseIntError,
    str::FromStr,
};

/// A MIDI note. Note that `C4 = 60`, `A4 = 69`.
///
/// We use a 16-bit unsigned integer to store the MIDI note index. This is much larger than the MIDI
/// specification, which only uses values from 0-127. The main reason is so that methods that
/// convert [`RawFreq`](crate::prelude::RawFreq) into [`MidiNote`] and viceversa don't run out of
/// range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MidiNote {
    /// The MIDI note index.
    pub note: i16,
}

impl MidiNote {
    /// Initializes a new [`MidiNote`].
    #[must_use]
    pub const fn new(note: i16) -> Self {
        Self { note }
    }
}

/// We use `A4` as a default note.
impl Default for MidiNote {
    fn default() -> Self {
        Self::A4
    }
}

#[cfg(feature = "midly")]
impl From<midly::num::u7> for MidiNote {
    fn from(value: midly::num::u7) -> Self {
        Self::new(i16::from(value.as_int()))
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
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Short => write!(f, "the string was too short"),
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
                let mut note = i16::from(note);
                let index = match next {
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

                note += 12 * (name[index..].parse::<i16>()? + 1);
                Ok(MidiNote::new(note))
            } else {
                Err(NameError::Letter(letter))
            }
        } else {
            Err(NameError::Short)
        }
    }
}

impl Display for MidiNote {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // Truncation is impossible.
        #[allow(clippy::cast_possible_truncation)]
        let letter = note_to_letter(self.note.rem_euclid(12) as u8);
        let octave = isize::from(self.note / 12) - 1;
        write!(f, "{letter}{octave}")
    }
}
