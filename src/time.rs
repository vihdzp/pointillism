//! Defines [`Time`] and its basic methods.

use crate::{freq::Freq, SAMPLE_RATE_F64};

use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    time::Duration,
};

/// Number of seconds in a minute.
const MIN_SECS: f32 = 60.0;

/// Number of seconds in an hour.
const HR_SECS: f32 = 60.0 * MIN_SECS;

/// Number of seconds in a day.
const DAY_SECS: f32 = 24.0 * HR_SECS;

/// Number of seconds in a year (365 days).
const YR_SECS: f32 = 365.0 * DAY_SECS;

/// Represents an amount of time.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Time {
    /// Number of seconds.
    pub seconds: f32,
}

impl Time {
    /// Zero time.
    pub const ZERO: Self = Self::new(0.0);

    /// A second.
    pub const SEC: Self = Self::new(1.0);

    /// A minute.
    pub const MIN: Self = Self::new(MIN_SECS);

    /// An hour.
    pub const HR: Self = Self::new(HR_SECS);

    /// A day.
    pub const DAY: Self = Self::new(DAY_SECS);

    /// A year.
    pub const YR: Self = Self::new(YR_SECS);

    /// Initializes a time variable for the number of seconds.
    #[must_use]
    pub const fn new(seconds: f32) -> Self {
        Self { seconds }
    }

    /// Initializes a time variable for the number of frames.
    #[must_use]
    pub fn new_frames(frames: f32) -> Self {
        Self::new(frames / SAMPLE_RATE_F64)
    }

    /// The time for a single beat at a given BPM.
    #[must_use]
    pub fn new_beat(bpm: f32) -> Self {
        Self::new(MIN_SECS / bpm)
    }

    /// Time to frequency.
    #[must_use]
    pub fn freq(&self) -> Freq {
        Freq::new(1.0 / self.seconds())
    }

    /// The time in seconds.
    #[must_use]
    pub const fn seconds(&self) -> f32 {
        self.seconds
    }

    /// The time in milliseconds.
    #[must_use]
    pub fn milliseconds(&self) -> f32 {
        1e3 * self.seconds()
    }

    /// The time in frames.
    #[must_use]
    pub fn frames(&self) -> f32 {
        self.seconds() * SAMPLE_RATE_F64
    }

    /// Advances the time by one frame.
    pub fn advance(&mut self) {
        self.seconds += 1.0 / SAMPLE_RATE_F64;
    }
}

impl From<Duration> for Time {
    fn from(value: Duration) -> Self {
        Self::new(value.as_secs_f32())
    }
}

impl From<Time> for Duration {
    fn from(value: Time) -> Self {
        Self::from_secs_f32(value.seconds())
    }
}

/// We use the [`human_duration`] crate for pretty-printing.
impl Debug for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f.alternate() {
            write!(f, "{}", human_duration::human_duration(&(*self).into()))
        } else {
            f.debug_struct("Time")
                .field("seconds", &self.seconds)
                .finish()
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} s", self.seconds())
    }
}

impl Mul<Freq> for f32 {
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

impl Mul<Time> for f32 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Time {
        Time::new(self * rhs.seconds)
    }
}

impl Mul<f32> for Time {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Time {
    fn mul_assign(&mut self, rhs: f32) {
        self.seconds *= rhs;
    }
}

impl Div<f32> for Time {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.seconds / rhs)
    }
}

impl DivAssign<f32> for Time {
    fn div_assign(&mut self, rhs: f32) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn yr_secs() {
        assert_eq!(format!("{:#?}", Time::YR), "1y 0mon 0d 0h 0m 0ms");
    }
}
