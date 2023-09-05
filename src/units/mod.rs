//! Implements different units for [time] and [frequency](freq), among others.
//!
//! ## Conventions
//!
//! There are many conventions made in music about units, some more justified than others. Since our
//! goal is to provide both generality and convenience, we take the following stances:
//!
//! | Natural units | We use [seconds](https://en.wikipedia.org/wiki/Second) for time, and [hertz](https://en.wikipedia.org/wiki/Hertz) for frequency. |
//! |-|-|
//! | Tuning | We recognize the 12-note equal temperament with A4 = 440 Hz as near-universal for Western music. As such, many helper methods and constants will make this assumption. However, we provide more general methods for creating notes with arbitrary frequencies. | 
//! | Sample rate | We recognize the 44.1 kHz sample rate as being the most common for audio, and have thus set it as the type default. However, we recognize both that other standards (notably 48 kHz) exist, and that there's utility in audio with lower sample rates. Thus, we've abstained from making many helper methods and constants with this assumption. |
//!
//! TODO: CONTINUE WRITING

pub mod boilerplate;
pub mod frac_int;
mod freq;
pub mod midi;
mod sample_rate;
mod time;

pub use freq::{Freq, Interval, RawFreq};
pub use sample_rate::SampleRate;
pub use time::{Time, Timer};

/// This magic number `69.0` corresponds to the MIDI index of A4.
const A4_MIDI: f64 = midi::Note::A4.note as f64;
