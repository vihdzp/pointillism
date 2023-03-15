//! # Pointillism
//!
//! A library for generating and manipulating audio.

mod basic;
pub mod effects;
pub mod generators;

pub use basic::*;

/// The sample rate for the audio file, in samples per second.
pub const SAMPLE_RATE: u32 = 44100;

/// The number of channels in the output file.
pub const CHANNELS: u8 = 2;

/// The specification for the output file.
pub const SPEC: hound::WavSpec = hound::WavSpec {
    channels: CHANNELS as u16,
    sample_rate: SAMPLE_RATE,
    bits_per_sample: 32,
    sample_format: hound::SampleFormat::Float,
};

/// Pitch for the base note A4.
pub const A4: Freq = Freq::new(440.0);
