#![doc = include_str!("../README.md")]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod curves;
pub mod effects;
pub mod generators;
pub mod map;
pub mod prelude;
pub mod sample;
pub mod signal;
pub mod units;

use prelude::*;

use hound::{Result, SampleFormat, WavSpec, WavWriter};

/// The specification for the output file.
const fn spec(channels: u8, sample_rate: SampleRate) -> WavSpec {
    WavSpec {
        channels: channels as u16,
        sample_rate: sample_rate.0,
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

/// Creates a song with a given duration, writing down each sample as it comes. The duration of the
/// file is exact to the sample.
///
/// The resulting WAV file will be mono or stereo, depending on whether the passed function returns
/// [`Mono`](crate::prelude::Mono) or [`Stereo`](crate::prelude::Stereo).
///
/// See the `examples` folder for example creations.
///
/// ## Errors
///
/// This should only return an error in case of an IO error.
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
pub fn create<P: AsRef<std::path::Path>, A: Audio, F: FnMut(Time) -> A>(
    filename: P,
    length: Time,
    sample_rate: SampleRate,
    mut song: F,
) -> Result<()> {
    let length = length.samples.int();

    // The size is either 1 or 2.
    #[allow(clippy::cast_possible_truncation)]
    let mut writer = WavWriter::create(filename, spec(A::SIZE as u8, sample_rate))?;

    let mut time = Time::ZERO;
    for _ in 0..length {
        song(time).write(&mut writer)?;
        time.advance();
    }

    writer.finalize()
}

/// A convenience function to [`create`] a song from a given signal.
///
/// The resulting WAV file will be mono or stereo, depending on whether the passed function returns
/// [`Mono`](crate::prelude::Mono) or [`Stereo`](crate::prelude::Stereo).
///
/// ## Errors
///
/// This should only return an error in case of an IO error.
pub fn create_from_sgn<P: AsRef<std::path::Path>, S: SignalMut>(
    filename: P,
    length: Time,
    sample_rate: SampleRate,
    mut sgn: S,
) -> Result<()>
where
    S::Sample: Audio,
{
    create(filename, length, sample_rate, |_| sgn.next())
}
