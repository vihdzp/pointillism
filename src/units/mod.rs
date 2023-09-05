//! Implements different units for [time], [frequency](freq), and [notes](note).

pub mod boilerplate;
pub mod frac_int;
mod freq;
pub mod midi;
mod sample_rate;
mod time;

pub use freq::{Freq, Interval, RawFreq};
pub use sample_rate::SampleRate;
pub use time::{Time, Timer};
