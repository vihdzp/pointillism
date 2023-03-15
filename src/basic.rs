//! Defines the basic types and traits for the crate.

use crate::SAMPLE_RATE;
use std::ops::*;

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
/// This is needed to get around orphan rules.
pub struct FnWrapper<F>(pub F);

/// An abstract trait for a structure representing a function `X → Y`.
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
pub trait MapMut<X, Y> {
    fn modify(&self, x: &mut X, y: Y);
}

impl<X, Y, F: Fn(&mut X, Y)> MapMut<X, Y> for FnWrapper<F> {
    fn modify(&self, x: &mut X, y: Y) {
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

/// Represents a frequency.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Freq {
    /// The frequency in hertz.
    pub hz: f64,
}

/// Converts a letter to a numeric note.
fn letter_to_note(letter: char) -> i16 {
    match letter {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => panic!("Invalid letter"),
    }
}

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

    /// Initializes a frequency in a given `edo` (equal division of the octave)
    /// a certain amount of `note`s above or below a `base` pitch (usually
    /// [`A4`](crate::A4)).
    pub fn new_edo(base: Freq, edo: u16, note: i16) -> Self {
        2f64.powf(note as f64 / edo as f64) * base
    }

    /// Initializes a frequency from a MIDI note.
    pub fn new_midi(note: i16) -> Self {
        Self::new_edo(crate::A4, 12, note - 69)
    }

    /// Initializes a pitch from its name (e.g. `"A4"` or `"G#5"`).
    pub fn new_name(name: &str) -> Self {
        let mut chars = name.chars();
        let mut note = letter_to_note(chars.next().unwrap());

        let idx = match chars.next().unwrap() {
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

        note += 12 * (name[idx..].parse::<i16>().unwrap() + 1);
        Self::new_midi(note)
    }
}

impl Mul<Freq> for f64 {
    type Output = Freq;

    fn mul(self, rhs: Freq) -> Freq {
        Freq::new(self * rhs.hz)
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
    pub fn new(seconds: f64) -> Self {
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

    /// Zero seconds.
    pub fn zero() -> Self {
        Self::default()
    }

    /// The time in seconds.
    pub fn seconds(&self) -> f64 {
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
