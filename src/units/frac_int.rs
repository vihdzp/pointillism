//! Implements a positive fractional number type.
//!
//! This is used for [`Time::samples`].

use std::ops::{Div, DivAssign, Mul, MulAssign};

/// The floating point 2<sup>16</sup> as an `f32`.
#[allow(clippy::cast_precision_loss)]
const POW_TWO_F32: f32 = (1u32 << 16) as f32;
/// The floating point 2<sup>16</sup> as an `f64`.
const POW_TWO_F64: f64 = (1u32 << 16) as f64;

/// A fractional number backed by a `u64`.
///
/// The number `FracInt(x)` represents x / 2<sup>16</sup>.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Rem,
    derive_more::RemAssign,
    derive_more::Sum,
)]
pub struct FracInt(u64);

impl FracInt {
    /// The number zero.
    pub const ZERO: Self = Self::new(0);
    /// The number one.
    pub const ONE: Self = Self::new(1);

    /// The maximum number that can be stored by this type.
    pub const MAX: Self = Self::new(u64::MAX);

    /// Initializes a [`FracInt`] from the integer and fractional parts.
    ///
    /// The number `int` must be less than 2<sup>48</sup>.
    #[must_use]
    pub const fn from_parts(int: u64, frac: u16) -> Self {
        Self((int << 16) + frac as u64)
    }

    /// Converts a number `x` less than 2<sup>48</sup> into a [`FracInt`].
    #[must_use]
    pub const fn new(x: u64) -> Self {
        Self::from_parts(x, 0)
    }

    /// The integer part of this number.
    #[must_use]
    pub const fn int(self) -> u64 {
        self.0 >> 16
    }

    /// The fractional part of this number, multiplied by 2<sup>16</sup>.
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub const fn frac_int(self) -> u16 {
        // Truncation is exactly what we want.
        self.0 as u16
    }

    /// Rounds an `f32` into a [`FracInt`].
    ///
    /// The value ought to be between 0 and 2<sup>48</sup>.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn from_f32(value: f32) -> Self {
        Self::from_parts(value as u64, (value.fract() * POW_TWO_F32).round() as u16)
    }

    /// Rounds an `f64` into a [`FracInt`].
    ///
    /// The value ought to be between 0 and 2<sup>48</sup>.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn from_f64(value: f64) -> Self {
        Self::from_parts(value as u64, (value.fract() * POW_TWO_F64).round() as u16)
    }

    /// Rounds this value as an `f32`.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn into_f32(self) -> f32 {
        self.0 as f32 / POW_TWO_F32
    }

    /// Rounds this value as an `f64`.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn into_f64(self) -> f64 {
        self.0 as f64 / POW_TWO_F64
    }

    /// The fractional part of this number.
    ///
    /// Since `f32` has more than the 16 needed mantissa digits, this conversion is exact.
    #[must_use]
    pub fn frac(self) -> f32 {
        f32::from(self.frac_int()) / POW_TWO_F32
    }
}

/// Implements [`From`] for integer types.
macro_rules! impl_from_int {
    ($($ty: ty),*) => {
        $(impl From<$ty> for FracInt {
            fn from(value: $ty) -> Self {
                Self::new(value as u64)
            }
        }
    )*};
}

impl_from_int!(u8, u16, u32);

impl std::fmt::Display for FracInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decimal = format!("{}", self.frac());
        write!(f, "{}{}", self.int(), &decimal[1..])
    }
}

/// Implements the basic [`Mul`] and [`Div`] traits for integer types.
macro_rules! impl_mul_div_uint {
    ($($ty: ty),*) => {$(
        impl Mul<$ty> for FracInt {
            type Output = Self;

            fn mul(self, rhs: $ty) -> Self {
                Self(self.0 * rhs as u64)
            }
        }

        impl Div<$ty> for FracInt {
            type Output = Self;

            fn div(self, rhs: $ty) -> Self {
                Self(self.0 / rhs as u64)
            }
        }
    )*};
}

impl_mul_div_uint!(u8, u16, u32, u64);

impl Mul<f64> for FracInt {
    type Output = Self;

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn mul(self, rhs: f64) -> Self {
        Self((self.0 as f64 * rhs) as u64)
    }
}

impl Div<f64> for FracInt {
    type Output = Self;

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn div(self, rhs: f64) -> Self {
        Self((self.0 as f64 / rhs) as u64)
    }
}

/// Implements the remaining [`Mul`] and [`Div`] traits.
macro_rules! impl_mul_div_other {
    ($($ty: ty),*) => {$(
        impl Mul<FracInt> for $ty {
            type Output = FracInt;

            fn mul(self, rhs: FracInt) -> FracInt {
                rhs * self
            }
        }

        impl MulAssign<$ty> for FracInt {
            fn mul_assign(&mut self, rhs: $ty) {
                *self = *self * rhs
            }
        }

        impl DivAssign<$ty> for FracInt {
            fn div_assign(&mut self, rhs: $ty) {
                *self = *self / rhs
            }
        }
    )*};
}

impl_mul_div_other!(u8, u16, u32, u64, f64);

impl Div for FracInt {
    type Output = f64;

    fn div(self, rhs: Self) -> f64 {
        self.into_f64() / rhs.into_f64()
    }
}

#[cfg(test)]
mod test {
    use super::FracInt;

    #[test]
    fn display() {
        assert_eq!(format!("{}", FracInt::new(0)), "0");
        assert_eq!(format!("{}", FracInt::new(1)), "1");
        assert_eq!(format!("{}", FracInt::from_f32(0.375)), "0.375");
    }
}
