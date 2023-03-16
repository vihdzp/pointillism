//! # Pointillism
//!
//! A compositional library for musical composition.
//!
//! ## Examples
//!
//! If you want to see `pointillism` in action and what it's capable of, run the
//! examples in the `examples` folder.
//!
//! **Note:** Some examples may be loud, dissonant, and/or jarring. Hearing
//! discretion is advised.
//!
//! ## Design
//!
//! The way in which `pointillism` outputs audio is by writing sample by sample
//! into a `.wav` file. The output is hardcoded to use 32-bit floating point,
//! although calculations are internally done using 64-bit, for extra precision.
//!
//! For convenience, the [`Signal`](crate::signal::Signal) trait is provided.
//! Structs implementing this trait generate sample data frame by frame, which
//! can be advanced, or retriggered.
//!
//! Signals may be composed to create more complex signals, using for instance
//! an [`Envelope`](crate::effects::Envelope). Moreover, you can implement the
//! trait for your own structs, giving you vast control over the samples you're
//! producing.

mod basic;
pub mod effects;
pub mod generators;
pub mod prelude;
pub mod sample;
pub mod signal;

pub use basic::*;
use hound::*;
use sample::AudioSample;

/// The sample rate for the audio file, in samples per second.
pub const SAMPLE_RATE: u32 = 44100;

/// The specification for the output file.
const fn spec(channels: u8) -> WavSpec {
    WavSpec {
        channels: channels as u16,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    }
}

/// Pitch for the base note A4.
pub const A4: Freq = Freq::new(440.0);

/// Creates a song with a given duration, writing down each sample as it comes.
pub fn create<P: AsRef<std::path::Path>, A: AudioSample, F: FnMut(Time) -> A>(
    filename: P,
    length: Time,
    mut song: F,
) {
    let mut timer = Time::zero();
    let mut writer = WavWriter::create(filename, spec(A::CHANNELS)).unwrap();

    while timer < length {
        song(timer).write(&mut writer).unwrap();
        timer.advance();
    }

    writer.finalize().unwrap();
}
