//! Structures that generate signals, be they envelope or audio data.

pub mod curve;
pub mod poly;

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
pub fn to_pos(x: f64) -> f64 {
    (x + 1.0) / 2.0
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
pub fn to_sgn(x: f64) -> f64 {
    2.0 * x - 1.0
}
