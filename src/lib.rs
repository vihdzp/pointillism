//! # Pointillism
//!
//! A compositional library for musical composition.
//!
//! ## Examples
//!
//! If you want to see `pointillism` in action and what it's capable of, run the
//! examples in the `examples` folder.
//!
//! **Note:** Some examples may be loud, dissonant, and/or jarring. Hearing
//! discretion is advised.
//!
//! ## Design
//!
//! The way in which `pointillism` outputs audio is by writing sample by sample
//! into a 32-bit floating point `.wav` file. Internal calculations use 64-bit
//! floating points.
//!
//! For convenience, the [`Signal`] trait is provided.
//! Structs implementing this trait generate sample data frame by frame, which
//! can be advanced or retriggered.
//!
//! Signals may be composed to create more complex signals, using for instance
//! the [`MapSgn`](crate::prelude::MutSgn) and
//! [`MutSgn`](crate::prelude::MutSgn) structs. Moreover, you can implement the
//! [`Signal`] trait for your own structs, giving you vast control over the
//! samples you're producing.
//!
//! Signals that generate audio on their own are called *generators*. Their
//! names are suffixed by `Gen`. Signals that modify the output from another
//! signal are called *effects*.
//!
//! You can think of pointillism as a compile-time modular synthesizer, where
//! every new struct is its own module.
//!
//! ## Versions
//!
//! The following versions of `pointillism` exist:
//!
//! - 0.1.0 - 0.1.7: very early versions, have been yanked from `crates`.
//! - 0.2.0 - 0.2.6: more stable versions, but still subject to drastic change.
//!
//! Once the basic structure of `pointillism` stabilizes, the version will
//! advance to 0.3.0, and a changelog will be made.
//!
//! * [`Signal`]: crate::prelude::Signal

#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]

pub mod curves;
pub mod effects;
pub mod freq;
pub mod generators;
pub mod map;
pub mod prelude;
pub mod sample;
pub mod signal;
pub mod time;

use freq::Freq;
use hound::{Result, SampleFormat, WavSpec, WavWriter};
use sample::Audio;
use time::Time;

/// The sample rate for the audio file, in samples per second.
pub const SAMPLE_RATE: u16 = 44100;

/// The sample rate for the audio file, in samples per second.
pub const SAMPLE_RATE_F64: f64 = SAMPLE_RATE as f64;

/// The specification for the output file.
const fn spec(channels: u8) -> WavSpec {
    WavSpec {
        channels: channels as u16,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    }
}

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
#[must_use]
pub fn pos(x: f64) -> f64 {
    (x + 1.0) / 2.0
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[must_use]
pub fn sgn(x: f64) -> f64 {
    2.0 * x - 1.0
}

/// Creates a song with a given duration, writing down each sample as it comes.
///
/// The resulting WAV file will be mono or stereo, depending on whether the
/// passed function returns [`Mono`](crate::prelude::Mono) or
/// [`Stereo`](crate::prelude::Stereo).
///
/// See the `examples` folder for example creations.
///
/// ## Errors
///
/// This should only return an error in case of an IO error.
pub fn create<P: AsRef<std::path::Path>, A: Audio, F: FnMut(Time) -> A>(
    filename: P,
    length: Time,
    mut song: F,
) -> Result<()> {
    let mut timer = Time::ZERO;
    let mut writer = WavWriter::create(filename, spec(A::CHANNELS))?;

    while timer < length {
        song(timer).write(&mut writer)?;
        timer.advance();
    }

    writer.finalize()
}
