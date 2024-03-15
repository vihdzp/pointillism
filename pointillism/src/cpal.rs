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
//! let sample_rate: unt::SampleRate = supported_config.sample_rate().into();
//!
//! // Length of the sine wave.
//! let raw_length = unt::RawTime::SEC;
//! let length = unt::Time::from_raw(raw_length, sample_rate);
//! // A reasonable buffer size.
//! let buffer_size = cpal::BufferSize::Fixed(1024);
//! // Note frequency.
//! let freq = unt::Freq::from_hz(440.0, sample_rate);
//!
//! // Play a sine wave with a specified frequency.
//! let sgn = gen::Loop::<smp::Mono, _>::new(crv::Sin, freq);
//!
//! // Creates the stream and plays it.
//! let stream = Song::new_sgn_owned(length, sample_rate, sgn).build_output_stream(
//!     &device,
//!     buffer_size,
//!     |err| eprintln!("{err}"),
//! )
//! .expect("stream could not be created");
//! stream.play().expect("stream could not be played");
//!
//! // Make sure we don't exit before the file ends playing.
//! std::thread::sleep(raw_length.into());
//! # }
//! ```

use crate::prelude::*;
use cpal::{traits::DeviceTrait, StreamConfig};

/// A type alias for the return type of these functions.
pub type CpalResult = Result<<cpal::Device as DeviceTrait>::Stream, cpal::BuildStreamError>;

impl From<unt::SampleRate> for cpal::SampleRate {
    fn from(value: unt::SampleRate) -> Self {
        Self(value.0)
    }
}

impl From<cpal::SampleRate> for unt::SampleRate {
    fn from(value: cpal::SampleRate) -> Self {
        Self(value.0)
    }
}

impl<F: Send + 'static + SongFunc> Song<F> {
    /// Builds a [`cpal`] output stream for a song. The sample rate and length are queried directly
    /// from the [`Song`]. Use a length of [`unt::Time::MAX`] for an infinite stream.
    ///
    /// Note that you'll probably need to use [`Self::new_sgn_owned`] to define the song and use
    /// this.  
    ///
    /// For the meaning of the parameters and possible errors, see [`cpal::BuildStreamError`].
    ///
    /// ## Example
    ///
    /// See the [module docs](self) for an example.
    #[allow(clippy::missing_errors_doc)]
    pub fn build_output_stream<E: FnMut(cpal::StreamError) + Send + 'static>(
        mut self,
        device: &cpal::Device,
        buffer_size: cpal::BufferSize,
        error_callback: E,
    ) -> CpalResult {
        let channels = F::Sample::size_u8();
        let mut time = unt::Time::ZERO;
        let timeout = if self.length == unt::Time::MAX {
            None
        } else {
            Some(self.length.into_raw(self.sample_rate).into())
        };

        device.build_output_stream(
            &StreamConfig {
                channels: u16::from(channels),
                sample_rate: self.sample_rate.into(),
                buffer_size,
            },
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut idx = 0;
                while idx < data.len() {
                    let sample = self.song.eval(time);
                    for j in 0..(channels as usize) {
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
}
