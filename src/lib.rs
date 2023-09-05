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

/// Creates a song with a given duration, writing down each sample as it comes. The duration of the
/// file is exact to the frame.
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
    mut song: F,
) -> Result<()> {
    // Precision loss should never occur in practical circumstances.
    let length = length.frames() as u64;

    // The size is either 1 or 2.
    #[allow(clippy::cast_possible_truncation)]
    let mut writer = WavWriter::create(filename, spec(A::SIZE as u8))?;

    for frames in 0..length {
        song(Time::new_frames(frames as f64)).write(&mut writer)?;
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
    mut sgn: S,
) -> Result<()>
where
    S::Sample: Audio,
{
    create(filename, length, |_| sgn.next())
}
