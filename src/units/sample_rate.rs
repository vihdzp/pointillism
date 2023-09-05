/// Represents the sample rate of an audio file, or of playback.
///
/// Many common sample rates are defined as constants. That said, we recognize the widespread
/// prevalence of 44.1 kHz as a sample rate. As such, we\ve set it as the type default, and we've
/// created many convenience methods that assume this value.
pub struct SampleRate(pub u32);

impl SampleRate {
    /// Telephone quality.
    pub const TELEPHONE: Self = Self::new(8_000);
    /// Half the standard sample rate.
    pub const HALF: Self = Self::new(Self::DEFAULT.0 / 2);
    /// The standard sample rate.
    pub const DEFAULT: Self = Self::new(44_100);
    /// Studio quality.
    pub const STUDIO: Self = Self::new(48_000);
    /// Twice studio quality.
    pub const DOUBLE: Self = Self::new(Self::STUDIO.0 * 2);
    /// Four times studio quality.
    pub const QUAD: Self = Self::new(Self::STUDIO.0 * 4);

    /// Initializes a [`SampleRate`].
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

/// We use 44.1 kHz as the default sample rate.
impl Default for SampleRate {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<SampleRate> for f32 {
    fn from(value: SampleRate) -> Self {
        value.0 as f32
    }
}

impl From<SampleRate> for f64 {
    fn from(value: SampleRate) -> Self {
        value.0 as f64
    }
}

impl std::ops::Mul<u32> for SampleRate {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Self(self.0 * rhs)
    }
}
