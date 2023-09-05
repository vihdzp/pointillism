//! Implements the [`SampleRate`] type.

/// Represents the sample rate of an audio file, or of playback. Measured in **samples per second**.
///
/// Various common sample rates are defined as constants. We've set [`SampleRate::CD`] as the type
/// default, but we recognize that other standards exist, and have thus abstained from making many
/// helper methods using this assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SampleRate(pub u32);

impl SampleRate {
    /// Telephone quality.
    pub const TELEPHONE: Self = Self::new(8_000);
    /// Half the standard sample rate for CD audio.
    pub const HALF: Self = Self::new(Self::CD.0 / 2);
    /// The standard sample rate for CD audio.
    pub const CD: Self = Self::new(44_100);
    /// The standard sample rate for film.
    pub const FILM: Self = Self::new(48_000);

    /// Initializes a [`SampleRate`].
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

/// We use 44.1 kHz as the default sample rate.
impl Default for SampleRate {
    fn default() -> Self {
        Self::CD
    }
}

impl From<SampleRate> for f64 {
    fn from(value: SampleRate) -> Self {
        f64::from(value.0)
    }
}

impl std::ops::Mul<u32> for SampleRate {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Self(self.0 * rhs)
    }
}
