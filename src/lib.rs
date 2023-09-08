#![cfg_attr(not(feature = "github-actions-hack"), doc = include_str!("../README.md"))]
#![cfg_attr(
    feature = "github-actions-hack",
    doc = "This is a workaround for GitHub actions not finding the documentation file."
)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
// `pointillism` is really meant to be used through its prelude.
#![allow(clippy::module_name_repetitions)]

pub mod curves;
pub mod effects;
pub mod generators;
pub mod map;
pub mod prelude;
pub mod sample;
pub mod signal;
pub mod units;

#[cfg(feature = "cpal")]
pub mod cpal;

#[cfg(feature = "hound")]
use prelude::{Audio, SampleRate, SignalMut, Time};

/// Increments a value in `0..len` by one, and wraps it around.
///
/// This should be marginally more efficient than `value = (value + 1) % len`, as it avoids the more
/// costly modulo operation.
pub(crate) fn mod_inc(len: usize, value: &mut usize) {
    *value += 1;

    if *value == len {
        *value = 0;
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

/// The specification for the output file.
#[cfg(feature = "hound")]
#[must_use]
pub const fn spec(channels: u8, sample_rate: SampleRate) -> hound::WavSpec {
    hound::WavSpec {
        channels: channels as u16,
        sample_rate: sample_rate.0,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    }
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
#[cfg(feature = "hound")]
pub fn create<P: AsRef<std::path::Path>, A: Audio, F: FnMut(Time) -> A>(
    filename: P,
    length: Time,
    sample_rate: SampleRate,
    mut song: F,
) -> hound::Result<()> {
    let length = length.samples.int();
    let mut writer = hound::WavWriter::create(filename, spec(A::size_u8(), sample_rate))?;

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
#[cfg(feature = "hound")]
pub fn create_from_sgn<P: AsRef<std::path::Path>, S: SignalMut>(
    filename: P,
    length: Time,
    sample_rate: SampleRate,
    mut sgn: S,
) -> hound::Result<()>
where
    S::Sample: Audio,
{
    create(filename, length, sample_rate, |_| sgn.next())
}
