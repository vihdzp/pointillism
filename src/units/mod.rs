//! Implements different units for time and frequency, among others.
//!
//! ## Conventions
//!
//! There are many conventions made in music about units, some more justified than others. Since our
//! goal is to provide both generality and convenience, we take the following stances:
//!
//! | Concern | Handling |
//! |-|-|
//! | Natural units | We use [seconds](https://en.wikipedia.org/wiki/Second) for time, and [hertz](https://en.wikipedia.org/wiki/Hertz) for frequency. |
//! | Note names | We use [scientific pitch notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation). We also support the [MIDI tuning standard](https://en.wikipedia.org/wiki/MIDI_tuning_standard). |
//! | Tuning | We recognize the [12-note equal temperament](https://en.wikipedia.org/wiki/12_equal_temperament) with [A4 = 440 Hz](https://en.wikipedia.org/wiki/A440_(pitch_standard)) as near-universal for Western music. As such, many helper methods and constants will make this assumption. However, we provide more general methods for creating notes with arbitrary frequencies. |
//! | Sample rate | We recognize the [44.1 kHz](https://en.wikipedia.org/wiki/44,100_Hz) sample rate as being the most common for audio, and have thus set it as the type default. However, we recognize both that other standards (notably 48 kHz) exist, and that there's utility in audio with lower sample rates. Thus, we've abstained from making many helper methods and constants with this assumption. |

pub mod boilerplate;
mod frac_int;
mod freq;
pub mod midi;
mod sample_rate;
mod time;

// We define these in different files for simplicity, but they're all ultimately units.
pub use frac_int::FracInt;
pub use freq::{Freq, Interval, RawFreq};
pub use sample_rate::SampleRate;
pub use time::{RawTime, RawTimer, Time};

/// This magic number `69.0` corresponds to the MIDI index of A4.
const A4_MIDI: f64 = midi::Note::A4.note as f64;

impl std::ops::Mul<RawFreq> for RawTime {
    type Output = f64;

    fn mul(self, rhs: RawFreq) -> f64 {
        self.seconds() * rhs.hz()
    }
}

impl std::ops::Mul<RawTime> for RawFreq {
    type Output = f64;

    fn mul(self, rhs: RawTime) -> f64 {
        rhs * self
    }
}

impl std::ops::Mul<Freq> for Time {
    type Output = f64;

    fn mul(self, rhs: Freq) -> f64 {
        f64::from(self.samples()) * rhs.samples()
    }
}
