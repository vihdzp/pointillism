//! Integration with [`cpal`].

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
#[allow(clippy::missing_errors_doc)]
pub fn build_output_stream<
    A: Audio,
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
    S::Sample: Audio,
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
