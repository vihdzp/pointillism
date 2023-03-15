mod basic;
pub mod effects;
pub mod generators;

pub use basic::*;

use hound::*;

/// The sample rate for the audio file, in samples per second.
pub const SAMPLE_RATE: u32 = 44100;

/// The number of channels in the output file. Only 1 and 2 are supported.
pub const CHANNELS: u8 = 2;

/// The specification for the output file.
pub const SPEC: WavSpec = WavSpec {
    channels: CHANNELS as u16,
    sample_rate: SAMPLE_RATE,
    bits_per_sample: 32,
    sample_format: SampleFormat::Float,
};

/// Pitch for the base note A4.
pub const A4: Freq = Freq::new(440.0);
