#![cfg_attr(not(feature = "github-actions-hack"), doc = include_str!("../../README.md"))]
#![cfg_attr(
    feature = "github-actions-hack",
    doc = "If you're seeing this, you (accidentally or otherwise) enabled the \
    `github-actions-hack` feature. This is a workaround for GitHub actions not finding the \
    documentation file. Sorry!"
)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod buffers;
pub mod control;
pub mod curves;
pub mod effects;
pub mod generators;
pub mod map;
pub mod sample;
pub mod signal;
pub mod units;

#[cfg(feature = "cpal")]
pub mod cpal;
#[cfg(feature = "hound")]
pub use with_hound::*;

// Needed so that the docs render properly.
use crate::prelude::*;

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

/// Methods that require [`hound`].
#[cfg(feature = "hound")]
mod with_hound {
    use crate::prelude::*;

    /// The [specification](hound::WavSpec) for the output file.
    #[must_use]
    pub const fn spec(channels: u8, sample_rate: unt::SampleRate) -> hound::WavSpec {
        hound::WavSpec {
            channels: channels as u16,
            sample_rate: sample_rate.0,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        }
    }

    /// Creates a song with a given duration, writing down each sample as it comes. The duration of
    /// the file is exactly rounded down to the sample.
    ///
    /// The resulting WAV file will be mono or stereo, depending on whether the passed function
    /// returns [`smp::Mono`] or [`smp::Stereo`].
    ///
    /// See the `examples` folder for example creations.
    ///
    /// ## Errors
    ///
    /// This should only return an error in case of an IO error.
    ///
    /// ## Example
    ///
    /// We make the most basic song possible: a single sine wave.
    ///
    /// ```
    /// # use pointillism::prelude::*;
    /// // Project sample rate.
    /// const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;
    ///
    /// // File duration.
    /// let length = unt::Time::from_sec(1.0, SAMPLE_RATE);
    /// // Sine wave frequency.
    /// let freq = unt::Freq::from_hz(440.0, SAMPLE_RATE);
    ///
    /// // We create a mono signal that loops through a sine curve at the specified frequency.
    /// let mut sgn = gen::Loop::<smp::Mono, _>::new(crv::Sin, freq);
    ///
    /// // Export to file.
    /// pointillism::create_from_sgn("examples/sine.wav", length, SAMPLE_RATE, &mut sgn)  
    ///     .expect("IO error");
    /// ```
    pub fn create<P: AsRef<std::path::Path>, A: smp::Audio, F: FnMut(unt::Time) -> A>(
        filename: P,
        length: unt::Time,
        sample_rate: unt::SampleRate,
        mut song: F,
    ) -> hound::Result<()> {
        let length = length.samples.int();
        let mut writer = hound::WavWriter::create(filename, spec(A::size_u8(), sample_rate))?;

        let mut time = unt::Time::ZERO;
        for _ in 0..length {
            song(time).write(&mut writer)?;
            time.advance();
        }

        writer.finalize()
    }

    /// A convenience function to [`create`] a song from a given signal. The signal is not consumed.
    ///
    /// The resulting WAV file will be mono or stereo, depending on whether the passed function
    /// returns [`smp::Mono`] or [`smp::Stereo`].
    ///
    /// ## Errors
    ///
    /// This should only return an error in case of an IO error.
    ///
    /// ## Example
    ///
    /// For an example, see [`create`].
    pub fn create_from_sgn<P: AsRef<std::path::Path>, S: SignalMut>(
        filename: P,
        length: unt::Time,
        sample_rate: unt::SampleRate,
        sgn: &mut S,
    ) -> hound::Result<()>
    where
        S::Sample: smp::Audio,
    {
        create(filename, length, sample_rate, |_| sgn.next())
    }
}

/// Auxiliary module for importing traits.
pub mod traits {
    pub use crate::{
        buf::{ring::Ring, Buffer, BufferMut},
        map::{Map, Mut},
        sample::{Array, Sample, SampleBase},
        signal::*,
    };
}

/// The crate prelude.
pub mod prelude {
    #[cfg(feature = "hound")]
    pub use crate::buffers::wav::*;

    // Abbreviate module names.
    pub use crate::{
        buffers as buf, control as ctr, curves as crv, effects as eff, gen::poly as ply,
        generators as gen, map, sample as smp, traits as trt, units as unt,
    };
    // Import all traits.
    pub use crate::traits::*;
}
