//! Defines [`RawTime`] and its basic methods.

use std::{
    fmt::{Formatter, Result as FmtResult},
    ops::{Div, DivAssign, Mul, MulAssign},
    time::Duration,
};

/// Number of milliseconds in a second.
const SECS_MSEC: f64 = 1000.0;
/// Number of seconds in a minute.
const MIN_SECS: f64 = 60.0;
/// Number of seconds in an hour.
const HR_SECS: f64 = 60.0 * MIN_SECS;
/// Number of seconds in a day.
const DAY_SECS: f64 = 24.0 * HR_SECS;
/// Number of seconds in a year (365 days).
const YR_SECS: f64 = 365.0 * DAY_SECS;

/// An amount of time in **seconds**. Must be positive.
///
/// Most methods will require a [`Time`](super::Time) instead, which is dependent on your sample
/// rate. See [`Time::from_raw`](super::Time::from_raw).
///
/// ## Type invariant checking
///
/// It's impractical to constantly check that times are positive, and there's not really any simple
/// ways to mess this up, so we don't check the invariant. That's not to say things won't go wrong
/// if the invariant is broken!
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Rem,
    derive_more::RemAssign,
    derive_more::Sum,
)]
pub struct RawTime {
    /// Number of seconds.
    pub seconds: f64,
}

impl RawTime {
    /// No time.
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

    /// Initializes a time variable, in **seconds**.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub const fn new(seconds: f64) -> Self {
        Self { seconds }
    }

    /// Initializes a time variable for the number of hours.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub fn new_hr(hours: f64) -> Self {
        Self::new(hours * HR_SECS)
    }

    /// Initializes a time variable for the number of minutes.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub fn new_min(minutes: f64) -> Self {
        Self::new(minutes * MIN_SECS)
    }

    /// Initializes a time variable for the number of seconds.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub const fn new_sec(seconds: f64) -> Self {
        Self::new(seconds)
    }

    /// Initializes a time variable for the number of milliseconds.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub fn new_msec(millis: f64) -> Self {
        Self::new(millis / SECS_MSEC)
    }

    /// The time for a single beat at a given BPM.
    ///
    /// This number must be nonnegative, although this isn't checked.
    #[must_use]
    pub fn new_beat(bpm: f64) -> Self {
        Self::new(MIN_SECS / bpm)
    }

    /// The time in hours.
    #[must_use]
    pub fn hours(&self) -> f64 {
        self.seconds / HR_SECS
    }

    /// The time in minutes.
    #[must_use]
    pub fn minutes(&self) -> f64 {
        self.seconds / MIN_SECS
    }

    /// The time in seconds.
    #[must_use]
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The time in milliseconds.
    #[must_use]
    pub fn milliseconds(&self) -> f64 {
        SECS_MSEC * self.seconds
    }

    /// Converts a [`Duration`] into [`RawTime`].
    #[must_use]
    pub fn from_duration(duration: Duration) -> Self {
        Self::new(duration.as_secs_f64())
    }

    /// Converts a [`RawTime`] into a [`Duration`].
    #[must_use]
    pub fn into_duration(self) -> Duration {
        Duration::from_secs_f64(self.seconds)
    }
}

impl From<RawTime> for std::time::Duration {
    fn from(value: RawTime) -> Self {
        Self::from_secs_f64(value.seconds)
    }
}

impl std::fmt::Display for RawTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        #[cfg(feature = "human-duration")]
        if f.alternate() {
            return write!(
                f,
                "{}",
                human_duration::human_duration(&(*self).into_duration())
            );
        }

        write!(f, "{}s", self.seconds)
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
