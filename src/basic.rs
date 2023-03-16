//! Defines the basic types and traits for the crate.

use crate::SAMPLE_RATE;
use std::{error::Error, fmt::Display, num::ParseIntError, ops::*};

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
pub fn to_pos(x: f64) -> f64 {
    (x + 1.0) / 2.0
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
pub fn to_sgn(x: f64) -> f64 {
    2.0 * x - 1.0
}

/// Clamps a signal between `-1.0` and `1.0`.
pub fn clip(x: f64) -> f64 {
    x.clamp(-1.0, 1.0)
}

/// A wrapper for a Rust function which converts it into a [`Map`] or
/// [`MapMut`].
///
/// It may be necessary to explicitly write down the types of the arguments to
/// the function.
#[derive(Clone, Copy, Debug)]
pub struct FnWrapper<F>(pub F);

/// An abstract trait for a structure representing a function `X → Y`.
///
/// Due to orphan rules, this trait can't be implemented for Rust functions. In
/// order to use it in this case, wrap your function in [`FnWrapper`].
pub trait Map<X, Y> {
    /// Evaluates the function.
    fn eval(&self, x: X) -> Y;
}

impl<X, Y, F: Fn(X) -> Y> Map<X, Y> for FnWrapper<F> {
    fn eval(&self, x: X) -> Y {
        self.0(x)
    }
}

/// An abstract trait for a structure representing a function taking `&mut X`
/// and `Y`.
///
/// Due to orphan rules, this trait can't be implemented for Rust functions. In
/// order to use it in this case, wrap your function in [`FnWrapper`].
pub trait MapMut<X, Y> {
    fn modify(&mut self, x: &mut X, y: Y);
}

impl<X, Y, F: FnMut(&mut X, Y)> MapMut<X, Y> for FnWrapper<F> {
    fn modify(&mut self, x: &mut X, y: Y) {
        self.0(x, y);
    }
}

/// Represents the gain of some signal.
#[derive(Clone, Copy, Debug)]
pub struct Vol {
    /// Gain factor.
    pub gain: f64,
}

impl Vol {
    /// Initializes a new volume variable.
    pub const fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Gain measured in decibels.
    pub fn new_db(db: f64) -> Self {
        Self::new(10f64.powf(db / 20.0))
    }

    /// The gain in decibels.
    pub fn db(&self) -> f64 {
        20.0 * self.gain.log10()
    }

    /// Silence.
    pub const fn zero() -> Self {
        Self::new(0.0)
    }
}

impl Default for Vol {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Map<f64, f64> for Vol {
    fn eval(&self, x: f64) -> f64 {
        x * self.gain
    }
}

/// Relative pitch corresponding to a note in a given EDO or
/// [equal division of the octave](https://en.wikipedia.org/wiki/Equal_temperament).
pub fn edo_note(edo: u16, note: f64) -> f64 {
    2f64.powf(note / edo as f64)
}

/// Relative pitch corresponding to a note in 12-EDO. See also [`edo_note`].
pub fn note(note: f64) -> f64 {
    edo_note(12, note)
}

/// Represents a frequency.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Freq {
    /// The frequency in hertz.
    pub hz: f64,
}

/// Converts a letter to a numeric note, from 0 to 11.
///
/// Returns `None` if anything other than a letter `A` - `G` is passed.
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

/// An error in [`Freq::new_name`].
#[derive(Clone, Debug)]
pub enum NameError {
    /// The string was too short.
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
    pub const fn new(hz: f64) -> Self {
        Self { hz }
    }

    /// The frequency in Hertz.
    pub const fn hz(&self) -> f64 {
        self.hz
    }

    /// The period, which equals the reciprocal of the frequency.
    pub fn period(&self) -> Time {
        Time::new(1.0 / self.hz())
    }

    /// Initializes a frequency in a given `edo` (equal division of the octave),
    /// a certain amount of `note`s above or below a `base` pitch (usually
    /// [`A4`](crate::A4)).
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::Freq;
    /// // C5 is 3 semitones above A4.
    /// let C5 = Freq::new_edo_note(pointillism::A4, 12, 3.0);
    /// ```
    pub fn new_edo_note(base: Freq, edo: u16, note: f64) -> Self {
        edo_note(edo, note) * base
    }

    /// Initializes a frequency in 12-EDO, a certain amount of `note`s above or
    /// below a `base` pitch (usually [`A4`](crate::A4)).
    /// 
    /// See also [`Freq::new_edo_note`].
    ///
    /// ## Example
    ///
    /// ```
    /// # use pointillism::Freq;
    /// // C5 is 3 semitones above A4.
    /// let C5 = Freq::new_note(pointillism::A4, 3.0);
    /// ```
    pub fn new_note(base: Freq, note: f64) -> Self {
        Self::new_edo_note(base, 12, note)
    }

    /// Initializes a frequency from a MIDI note.
    pub fn new_midi(a4: Freq, note: i16) -> Self {
        Self::new_edo_note(a4, 12, note as f64 - 69.0)
    }

    /// Initializes a pitch from its name (e.g. `"A4"` or `"G#5"`).
    pub fn new_name(a4: Freq, name: &str) -> Result<Self, NameError> {
        let mut chars = name.chars();

        if let (Some(letter), Some(next)) = (chars.next(), chars.next()) {
            if let Some(note) = letter_to_note(letter) {
                let mut note = note as i16;

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

/// Represents an amount of time.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Time {
    /// Number of seconds.
    pub seconds: f64,
}

impl Time {
    /// Initializes a time variable for the number of seconds.
    pub const fn new(seconds: f64) -> Self {
        Self { seconds }
    }

    /// Initializes a time variable for the number of frames.
    pub fn new_frames(frames: f64) -> Self {
        Self::new(frames / SAMPLE_RATE as f64)
    }

    /// The time for a single beat at a given BPM.
    pub fn new_beat(bpm: f64) -> Self {
        Self::new(60.0 / bpm)
    }

    /// Time to frequency.
    pub fn freq(&self) -> Freq {
        Freq::new(1.0 / self.seconds())
    }

    /// Zero seconds.
    pub const fn zero() -> Self {
        Self::new(0.0)
    }

    /// The time in seconds.
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The time in frames.
    pub fn frames(&self) -> f64 {
        self.seconds() * SAMPLE_RATE as f64
    }

    /// Advances the time by one frame.
    pub fn advance(&mut self) {
        self.seconds += 1.0 / SAMPLE_RATE as f64;
    }
}

impl From<Time> for Freq {
    fn from(value: Time) -> Self {
        value.freq()
    }
}

impl Mul<Freq> for f64 {
    type Output = Freq;

    fn mul(self, rhs: Freq) -> Freq {
        Freq::new(self * rhs.hz)
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.seconds + rhs.seconds)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.seconds += rhs.seconds;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.seconds - rhs.seconds)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.seconds -= rhs.seconds;
    }
}

impl Mul<Time> for f64 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Time {
        Time::new(self * rhs.seconds)
    }
}

impl Mul<f64> for Time {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Time {
    fn mul_assign(&mut self, rhs: f64) {
        self.seconds *= rhs;
    }
}

impl Div<f64> for Time {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.seconds / rhs)
    }
}

impl DivAssign<f64> for Time {
    fn div_assign(&mut self, rhs: f64) {
        self.seconds /= rhs;
    }
}

impl Rem for Time {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.seconds % rhs.seconds)
    }
}

impl RemAssign for Time {
    fn rem_assign(&mut self, rhs: Self) {
        self.seconds %= rhs.seconds;
    }
}
