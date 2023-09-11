//! Implements the [`Interval`] type.

/// Represents an interval, or a ratio between notes.
///
/// This is functionally identical to a single `f64`, but it implements some helper methods and
/// constants tailored for musical intervals.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Interval {
    /// The ratio in question.
    ///
    /// This should be positive!
    pub ratio: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self::UNISON
    }
}

impl Interval {
    /// Unison interval `1/1`.
    pub const UNISON: Self = Self::new(1.0);
    /// Minor third `6/5`.
    pub const MIN3: Self = Self::new(6.0 / 5.0);
    /// Major third `5/4`.
    pub const MAJ3: Self = Self::new(5.0 / 4.0);
    /// Perfect fourth `4/3`.
    pub const P4: Self = Self::new(4.0 / 3.0);
    /// Perfect fifth `3/2`.
    pub const P5: Self = Self::new(3.0 / 2.0);
    /// Minor sixth `5/3`.
    pub const MIN6: Self = Self::new(8.0 / 5.0);
    /// Major sixth `5/3`.
    pub const MAJ6: Self = Self::new(5.0 / 3.0);
    /// Harmonic seventh `7/4`.
    pub const H7: Self = Self::new(7.0 / 4.0);
    /// Octave interval `2/1`.
    pub const OCTAVE: Self = Self::new(2.0);
    /// Tritave interval `3/1`.
    pub const TRITAVE: Self = Self::new(3.0);

    /// Initializes a new ratio.
    #[must_use]
    pub const fn new(ratio: f64) -> Self {
        Self { ratio }
    }

    /// Relative pitch corresponding to a note in a given EDO.
    #[must_use]
    pub fn edo_note(edo: u16, note: f64) -> Self {
        Self::new(2f64.powf(note / f64::from(edo)))
    }

    /// Relative pitch corresponding to a note in 12-EDO.
    ///
    /// See also [`Self::edo_note`].
    #[must_use]
    pub fn note(note: f64) -> Self {
        Self::edo_note(12, note)
    }

    /// Returns the inverse ratio.
    #[must_use]
    pub fn inv(self) -> Self {
        Self::new(1.0 / self.ratio)
    }

    /// Takes the square root of an interval.
    ///
    /// For instance, a 12-EDO tritone is exactly the square root of an octave.
    #[must_use]
    pub fn sqrt(self) -> Self {
        Self::new(self.ratio.sqrt())
    }

    /// Raises an interval to an integer power.
    #[must_use]
    pub fn powi(self, n: i32) -> Self {
        Self::new(self.ratio.powi(n))
    }

    /// Raises an interval to a floating point power.
    #[must_use]
    pub fn powf(self, n: f64) -> Self {
        Self::new(self.ratio.powf(n))
    }

    /// An interval in octaves.
    #[must_use]
    pub fn octaves(oct: f64) -> Self {
        Self::OCTAVE.powf(oct)
    }
}

impl std::ops::Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Interval) -> Self {
        Self::new(self.ratio * rhs.ratio)
    }
}

impl std::ops::MulAssign for Interval {
    fn mul_assign(&mut self, rhs: Self) {
        self.ratio *= rhs.ratio;
    }
}

impl std::ops::Div for Interval {
    type Output = Self;

    fn div(self, rhs: Interval) -> Self {
        Self::new(self.ratio / rhs.ratio)
    }
}

impl std::ops::DivAssign for Interval {
    fn div_assign(&mut self, rhs: Self) {
        self.ratio /= rhs.ratio;
    }
}
