//! Defines [`Time`] and its basic methods.

use crate::{units::freq::Freq, SAMPLE_RATE_F64};

use std::{
    fmt::{Debug, Display, Formatter, Result},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
    time::Duration,
};

/// Number of seconds in a minute.
const MIN_SECS: f64 = 60.0;

/// Number of seconds in an hour.
const HR_SECS: f64 = 60.0 * MIN_SECS;

/// Number of seconds in a day.
const DAY_SECS: f64 = 24.0 * HR_SECS;

/// Number of seconds in a year (365 days).
const YR_SECS: f64 = 365.0 * DAY_SECS;

/// Accurately represents an amount of time.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Time {
    /// Number of frames.
    pub seconds: f64,
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
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub const fn new(seconds: f64) -> Self {
        Self { seconds }
    }

    /// Initializes a time variable for the number of frames.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub fn new_frames(frames: f64) -> Self {
        Self::new(frames / SAMPLE_RATE_F64)
    }

    /// The time for a single beat at a given BPM.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub fn new_beat(bpm: f64) -> Self {
        Self::new(MIN_SECS / bpm)
    }

    /// Time to frequency.
    #[must_use]
    pub fn freq(&self) -> Freq {
        Freq::new(1.0 / self.seconds())
    }

    /// The time in seconds.
    #[must_use]
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The time in milliseconds.
    #[must_use]
    pub fn milliseconds(&self) -> f64 {
        1e3 * self.seconds()
    }

    /// The time in frames.
    #[must_use]
    pub fn frames(&self) -> f64 {
        self.seconds() * SAMPLE_RATE_F64
    }

    /// Advances the time by one frame.
    pub fn advance(&mut self) {
        self.seconds += 1.0 / SAMPLE_RATE_F64;
    }
}

impl From<Duration> for Time {
    fn from(value: Duration) -> Self {
        Self::new(value.as_secs_f64())
    }
}

impl From<Time> for Duration {
    fn from(value: Time) -> Self {
        Self::from_secs_f64(value.seconds())
    }
}

/// If the `human_duration` feature is enabled, we use the [`human_duration`] crate for
/// pretty-printing.
const HUMAN_DURATION: bool = cfg!(feature = "human-duration");

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if HUMAN_DURATION && f.alternate() {
            write!(f, "{}", human_duration::human_duration(&(*self).into()))
        } else {
            f.debug_struct("Time")
                .field("seconds", &self.seconds)
                .finish()
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} s", self.seconds())
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

    fn mul(self, rhs: f64) -> Self {
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

impl Div<Time> for Time {
    type Output = f64;

    fn div(self, rhs: Time) -> f64 {
        self.seconds / rhs.seconds
    }
}

impl Rem for Time {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self::new(self.seconds % rhs.seconds)
    }
}

impl RemAssign for Time {
    fn rem_assign(&mut self, rhs: Self) {
        self.seconds %= rhs.seconds;
    }
}

impl Sum for Time {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|x| x.seconds()).sum())
    }
}

/// A helper struct that can be used to do an event once after a certain time duration.
#[derive(Clone, Copy, Debug)]
pub struct Timer {
    /// Length of the timer.
    length: Time,

    /// Whether the timer is active.
    active: bool,
}

impl Timer {
    /// Initializes a new timer with the given length.
    #[must_use]
    pub const fn new(length: Time) -> Self {
        Self {
            length,
            active: true,
        }
    }

    /// Whether the timer should activate, given the current time elapsed. A timer can only activate
    /// once.
    pub fn tick(&mut self, time: Time) -> bool {
        if !self.active {
            return false;
        }

        let done = time >= self.length;
        if done {
            self.active = false;
        }
        done
    }

    /// Resets the timer, allowing it to activate again.
    pub fn reset(&mut self) {
        self.active = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn yr_secs() {
        assert_eq!(format!("{:#?}", Time::YR), "1y 0mon 0d 0h 0m 0s 0ms");
    }
}
