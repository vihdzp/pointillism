#![cfg_attr(not(feature = "github-actions-hack"), doc = include_str!("../README.md"))]
#![cfg_attr(
    feature = "github-actions-hack",
    doc = "If you're seeing this, you (accidentally or otherwise) enabled the \
    `github-actions-hack` feature. This is a workaround for GitHub actions not finding the \
    documentation file. Sorry!"
)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]

pub mod buffer;
pub mod control;
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
pub use with_hound::*;

// Needed so that the docs render properly.
#[allow(unused_imports)]
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
    /// returns [`Mono`](crate::prelude::Mono) or [`Stereo`](crate::prelude::Stereo).
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
    /// const SAMPLE_RATE: SampleRate = SampleRate::CD;
    ///
    /// // File duration.
    /// let length = Time::from_sec(1.0, SAMPLE_RATE);
    /// // Sine wave frequency.
    /// let freq = Freq::from_hz(440.0, SAMPLE_RATE);
    ///
    /// // We create a mono signal that loops through a sine curve at the specified frequency.
    /// let mut sgn = gen::Loop::<Mono, _>::new(Sin, freq);
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
    /// returns [`Mono`](crate::prelude::Mono) or [`Stereo`](crate::prelude::Stereo).
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
