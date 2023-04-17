//! Defines [`Time`] and its basic methods.

use crate::{freq::Freq, SAMPLE_RATE_F64};

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Represents an amount of time.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Time {
    /// Number of seconds.
    pub seconds: f64,
}

impl Time {
    /// Zero time.
    pub const ZERO: Self = Self::new(0.0);

    /// A second.
    pub const SEC: Self = Self::new(1.0);

    /// A minute.
    pub const MIN: Self = Self::new(60.0);

    /// An hour.
    pub const HR: Self = Self::new(3600.0);

    /// A day.
    pub const DAY: Self = Self::new(86400.0);

    /// Initializes a time variable for the number of seconds.
    #[must_use]
    pub const fn new(seconds: f64) -> Self {
        Self { seconds }
    }

    /// Initializes a time variable for the number of frames.
    #[must_use]
    pub fn new_frames(frames: f64) -> Self {
        Self::new(frames / SAMPLE_RATE_F64)
    }

    /// The time for a single beat at a given BPM.
    #[must_use]
    pub fn new_beat(bpm: f64) -> Self {
        Self::new(60.0 / bpm)
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
