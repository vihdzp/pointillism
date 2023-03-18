//! Defines [`Time`] and its basic methods.

use crate::prelude::*;

use std::ops::*;

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
