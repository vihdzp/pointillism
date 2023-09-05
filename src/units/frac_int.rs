//! Implements a positive fractional number type.
//!
//! For our purposes, this is faster, more convenient, and more precise than floating point.

/// A fractional number backed by a `u64`.
///
/// The number `FracInt(x)` represents x / 2<sup>16</sup>.
#[derive(Clone, Copy, Debug)]
pub struct FracInt(u64);

impl FracInt {
    /// The number zero.
    pub const ZERO: Self = Self::new(0);
    /// The number one.
    pub const ONE: Self = Self::new(1);

    /// Initializes a [`FracInt`] from the integer and fractional parts.
    ///
    /// The number `int` must be less than 2<sup>48</sup>.
    pub const fn from_parts(int: u64, frac: u16) -> Self {
        Self((int << 16) + frac as u64)
    }

    /// Converts a number `x` less than 2<sup>48</sup> into a [`FracInt`].
    pub const fn new(x: u64) -> Self {
        Self::from_parts(x, 0)
    }

    /// The integer part of this number.
    pub const fn int(self) -> u64 {
        self.0 >> 16
    }

    /// The fractional part of this number, multiplied by 2<sup>16</sup>.
    #[allow(clippy::cast_possible_truncation)]
    pub const fn frac_int(self) -> u16 {
        // Truncation is exactly what we want.
        self.0 as u16
    }

    /// Rounds an `f32` into a [`FracInt`].
    pub fn from_f32(x: f32) -> Self {
        Self::from_parts(x as u64, (x.fract() * ((1 << 16) as f32)).round() as u16)
    }

    /// Rounds an `f64` into a [`FracInt`].
    pub fn from_f64(x: f64) -> Self {
        Self::from_parts(x as u64, (x.fract() * ((1 << 16) as f64)).round() as u16)
    }

    /// The fractional part of this number.
    ///
    /// Since `f32` has more than the 16 needed mantissa digits, this conversion is exact.
    pub fn frac(self) -> f32 {
        self.frac_int() as f32 / ((1 << 16) as f32)
    }
}

impl std::ops::Add for FracInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for FracInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// Implements [`From`].
macro_rules! impl_from {
    ($($ty: ty),*) => {
        $(impl From<$ty> for FracInt{
            fn from(value: $ty) -> Self {
                Self::new(value as u64)
            }
        }
    )*};
}

impl_from!(u8, u16, u32, u64, u128);

impl std::fmt::Display for FracInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decimal = format!("{}", self.frac());
        write!(f, "{}{}", self.int(), &decimal[1..])
    }
}

mod test {
    use super::FracInt;

    #[test]
    fn display() {
        assert_eq!(format!("{}", FracInt::new(0)), "0");
        assert_eq!(format!("{}", FracInt::new(1)), "1");
        assert_eq!(format!("{}", FracInt::from_f32(0.375)), "0.375")
    }
}
