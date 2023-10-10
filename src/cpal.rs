//! Integration with [`cpal`].
//!
//! You can use the methods in this file in order to play a song in real time.
//!
//! ## Example
//!
//! This example is adapted from the [`cpal`] docs.
//!
//! ```
//! # use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
//! # use pointillism::prelude::*;
//! # // This example won't work on GitHub actions!
//! # #[cfg(not(feature = "github-actions-hack"))] {
//! // Set up the host and device.
//! let host = cpal::default_host();
//! let device = host
//!     .default_output_device()
//!     .expect("no output device available");
//!
//! // Query for the max sample rate.
//! let mut supported_configs_range = device
//!     .supported_output_configs()
//!     .expect("error while querying configs");
//! let supported_config = supported_configs_range
//!     .next()
//!     .expect("no supported config?!")
//!     .with_max_sample_rate();
//! let sample_rate: SampleRate = supported_config.sample_rate().into();
//!
//! // Length of the sine wave.
//! let duration = std::time::Duration::from_secs(1);
//! // A reasonable buffer size.
//! let buffer_size = cpal::BufferSize::Fixed(1024);
//! // Note frequency.
//! let freq = Freq::from_hz(440.0, sample_rate);
//!
//! // Play a sine wave with a specified frequency.
//! let sgn = LoopGen::<Mono, _>::new(Sin, freq);
//!
//! // Creates the stream and plays it.
//! let stream = pointillism::cpal::build_output_stream_from_sgn(
//!     &device,
//!     Some(duration),
//!     sample_rate,
//!     buffer_size,
//!     |err| eprintln!("{err}"),
//!     sgn,
//! )
//! .expect("stream could not be created");
//! stream.play().expect("stream could not be played");
//!
//! // Make sure we don't exit before the file ends playing.
//! std::thread::sleep(duration);
//! # }
//! ```

use cpal::{traits::DeviceTrait, StreamConfig};

use crate::prelude::*;

/// A type alias for the return type of these functions.
pub type CpalResult = Result<<cpal::Device as DeviceTrait>::Stream, cpal::BuildStreamError>;

impl From<SampleRate> for cpal::SampleRate {
    fn from(value: SampleRate) -> Self {
        Self(value.0)
    }
}

impl From<cpal::SampleRate> for SampleRate {
    fn from(value: cpal::SampleRate) -> Self {
        Self(value.0)
    }
}

/// Builds a [`cpal`] output stream for a song.
///
/// For the meaning of the parameters and possible errors, see [`DeviceTrait::build_output_stream`].
///
/// ## Example
///
/// See the [module docs](self) for an example.
#[allow(clippy::missing_errors_doc)]
pub fn build_output_stream<
    A: smp::Audio,
    F: Send + 'static + FnMut(Time) -> A,
    E: FnMut(cpal::StreamError) + Send + 'static,
>(
    device: &cpal::Device,
    timeout: Option<std::time::Duration>,
    sample_rate: SampleRate,
    buffer_size: cpal::BufferSize,
    error_callback: E,
    mut song: F,
) -> CpalResult {
    let channels = A::size_u8();
    let mut time = Time::ZERO;

    device.build_output_stream(
        &StreamConfig {
            channels: u16::from(channels),
            sample_rate: sample_rate.into(),
            buffer_size,
        },
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut idx = 0;
            while idx < data.len() {
                let sample = song(time);
                for j in 0..A::SIZE {
                    // Truncation shouldn't happen in practice.
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        data[idx] = sample[j] as f32;
                    }
                    idx += 1;
                }
            }

            time.advance();
        },
        error_callback,
        timeout,
    )
}

/// Builds a [`cpal`] output stream for a signal.
///
/// For the meaning of the parameters and possible errors, see [`DeviceTrait::build_output_stream`].
///
/// ## Example
///
/// See the [module docs](self) for an example.
#[allow(clippy::missing_errors_doc)]
pub fn build_output_stream_from_sgn<
    S: SignalMut + Send + 'static,
    E: FnMut(cpal::StreamError) + Send + 'static,
>(
    device: &cpal::Device,
    timeout: Option<std::time::Duration>,
    sample_rate: SampleRate,
    buffer_size: cpal::BufferSize,
    error_callback: E,
    mut sgn: S,
) -> CpalResult
where
    S::Sample: smp::Audio,
{
    build_output_stream(
        device,
        timeout,
        sample_rate,
        buffer_size,
        error_callback,
        move |_| sgn.next(),
    )
}
