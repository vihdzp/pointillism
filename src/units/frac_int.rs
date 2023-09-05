//! Implements a positive fractional number type.
//!
//! This is used for [`Time::samples`].

use std::ops::{Div, DivAssign, Mul, MulAssign};

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

    /// The fractional part of this number.
    ///
    /// Since `f32` has more than the 16 needed mantissa digits, this conversion is exact.
    pub fn frac(self) -> f32 {
        self.frac_int() as f32 / ((1 << 16) as f32)
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

impl_from_int!(u8, u16, u32, u64, u128);

/// Implements [`From`] for floating point types.
macro_rules! impl_from_float {
    ($($ty: ty),*) => {
        $(impl From<$ty> for FracInt {
            fn from(value: $ty) -> Self {
                Self::from_parts(value as u64, (value.fract() * ((1 << 16) as $ty)).round() as u16)
            }
        }
    )*};
}

impl_from_float!(f32, f64);

/// Implements [`Into`] for floating point types.
macro_rules! impl_into_float {
    ($($ty: ty),*) => {
        $(impl From<FracInt> for $ty {
            fn from(value: FracInt) -> Self {
                value.0 as $ty / ((1 << 16) as $ty)
            }
        }
    )*};
}

impl_into_float!(f32, f64);

impl std::fmt::Display for FracInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decimal = format!("{}", self.frac());
        write!(f, "{}{}", self.int(), &decimal[1..])
    }
}

impl Mul<u64> for FracInt {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Self::new(self.0 * rhs)
    }
}

impl Div<u64> for FracInt {
    type Output = Self;

    fn div(self, rhs: u64) -> Self {
        Self::new(self.0 / rhs)
    }
}

impl Mul<f64> for FracInt {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::from(self.0 as f64 * rhs)
    }
}

impl Div<f64> for FracInt {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::from(self.0 as f64 / rhs)
    }
}

/// Implements the remaining [`Mul`] and [`Div`] traits.
macro_rules! impl_mul_div {
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

impl_mul_div!(u64, f64);

impl Div for FracInt {
    type Output = f64;

    fn div(self, rhs: Self) -> f64 {
        f64::from(self) / f64::from(rhs)
    }
}

#[cfg(test)]
mod test {
    use super::FracInt;

    #[test]
    fn display() {
        assert_eq!(format!("{}", FracInt::new(0)), "0");
        assert_eq!(format!("{}", FracInt::new(1)), "1");
        assert_eq!(format!("{}", FracInt::from(0.375f32)), "0.375")
    }
}
