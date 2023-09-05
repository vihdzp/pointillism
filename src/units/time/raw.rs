//! Defines [`RawTime`] and its basic methods.

use crate::{units::freq::RawFreq, SAMPLE_RATE_F64};

use std::{
    fmt::{Formatter, Result as FmtResult},
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

/// An amount of time in **seconds**.
///
/// Most methods will require a [`Time`] instead, which is dependent on your sample rate. See
/// [`Time::from_raw`].
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct RawTime {
    /// Number of frames.
    pub seconds: f64,
}

impl RawTime {
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

    /// RawTime to frequency.
    #[must_use]
    pub fn freq(&self) -> RawFreq {
        RawFreq::new(1.0 / self.seconds())
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

impl From<std::time::Duration> for RawTime {
    fn from(value: std::time::Duration) -> Self {
        Self::new(value.as_secs_f64())
    }
}

impl From<RawTime> for Duration {
    fn from(value: RawTime) -> Self {
        Self::from_secs_f64(value.seconds())
    }
}

impl std::fmt::Debug for RawTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if super::HUMAN_DURATION && f.alternate() {
            write!(f, "{}", human_duration::human_duration(&(*self).into()))
        } else {
            f.debug_struct("RawTime")
                .field("seconds", &self.seconds)
                .finish()
        }
    }
}

impl std::fmt::Display for RawTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} s", self.seconds())
    }
}

impl Add for RawTime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.seconds + rhs.seconds)
    }
}

impl AddAssign for RawTime {
    fn add_assign(&mut self, rhs: Self) {
        self.seconds += rhs.seconds;
    }
}

impl Sub for RawTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.seconds - rhs.seconds)
    }
}

impl SubAssign for RawTime {
    fn sub_assign(&mut self, rhs: Self) {
        self.seconds -= rhs.seconds;
    }
}

impl Mul<RawTime> for f64 {
    type Output = RawTime;

    fn mul(self, rhs: RawTime) -> RawTime {
        RawTime::new(self * rhs.seconds)
    }
}

impl Mul<f64> for RawTime {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        rhs * self
    }
}

impl MulAssign<f64> for RawTime {
    fn mul_assign(&mut self, rhs: f64) {
        self.seconds *= rhs;
    }
}

impl Div<f64> for RawTime {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.seconds / rhs)
    }
}

impl DivAssign<f64> for RawTime {
    fn div_assign(&mut self, rhs: f64) {
        self.seconds /= rhs;
    }
}

impl Div<RawTime> for RawTime {
    type Output = f64;

    fn div(self, rhs: RawTime) -> f64 {
        self.seconds / rhs.seconds
    }
}

impl Rem for RawTime {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self::new(self.seconds % rhs.seconds)
    }
}

impl RemAssign for RawTime {
    fn rem_assign(&mut self, rhs: Self) {
        self.seconds %= rhs.seconds;
    }
}

impl std::iter::Sum for RawTime {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|x| x.seconds()).sum())
    }
}

/// A helper struct that can be used to do an event once after a certain time duration.
#[derive(Clone, Copy, Debug)]
pub struct RawTimer {
    /// Length of the timer.
    length: RawTime,

    /// Whether the timer is active.
    active: bool,
}

impl RawTimer {
    /// Initializes a new timer with the given length.
    #[must_use]
    pub const fn new(length: RawTime) -> Self {
        Self {
            length,
            active: true,
        }
    }

    /// Whether the timer should activate, given the current time elapsed. A timer can only activate
    /// once.
    pub fn tick(&mut self, time: RawTime) -> bool {
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
