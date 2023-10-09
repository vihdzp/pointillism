//! Implements different units for time, frequency, MIDI notes, among others.
//!
//! ## Conventions
//!
//! There are many conventions made in music about units, some more justified than others. Since our
//! goal is to provide both generality and convenience, we take the following stances:
//!
//! | Concern | Handling |
//! |-|-|
//! | Natural units | We use [seconds](https://en.wikipedia.org/wiki/Second) for time, and [hertz](https://en.wikipedia.org/wiki/Hertz) for frequency. |
//! | Note names | We use [scientific pitch notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation), defaulting to names with sharps in display. We also support the [MIDI tuning standard](https://en.wikipedia.org/wiki/MIDI_tuning_standard), though with a wider range. |
//! | Tuning | We recognize the [12-note equal temperament](https://en.wikipedia.org/wiki/12_equal_temperament) with [A4 = 440 Hz](https://en.wikipedia.org/wiki/A440_(pitch_standard)) as near-universal for Western music. As such, many helper methods and constants will make this assumption. However, we provide more general methods for creating notes with arbitrary frequencies. |
//! | Sample rate | We recognize the [44.1 kHz](https://en.wikipedia.org/wiki/44,100_Hz) sample rate as being the most common for audio, and have thus set it as the type default. However, we recognize both that other standards (notably 48 kHz) exist, and that there's utility in audio with lower or higher sample rates. Thus, we've abstained from making many helper methods and constants with this assumption. |

pub mod boilerplate;
mod freq;
mod midi;
mod q_factor;
mod sample_rate;
mod time;
mod vol;

use std::ops::{Div, Mul};

pub use freq::{Freq, Interval, RawFreq};
pub use midi::MidiNote;
pub use q_factor::QFactor;
pub use sample_rate::SampleRate;
pub use time::{FracInt, RawTime, Time, Timer};
pub use vol::Vol;

/// This magic number `69.0` corresponds to the MIDI index of A4.
const A4_MIDI: f64 = midi::MidiNote::A4.note as f64;

// Boilerplate arithmetic implementations:

impl Mul<RawFreq> for RawTime {
    type Output = f64;

    fn mul(self, rhs: RawFreq) -> f64 {
        self.seconds * rhs.hz
    }
}

impl Mul<RawTime> for RawFreq {
    type Output = f64;

    fn mul(self, rhs: RawTime) -> f64 {
        rhs * self
    }
}

impl Mul<Freq> for Time {
    type Output = f64;

    fn mul(self, rhs: Freq) -> f64 {
        self.samples.into_f64() * rhs.samples
    }
}

impl Mul<Time> for Freq {
    type Output = f64;

    fn mul(self, rhs: Time) -> f64 {
        rhs * self
    }
}

impl RawTime {
    /// Converts time into frequency.
    #[must_use]
    pub fn raw_freq(self) -> RawFreq {
        RawFreq::new(1.0 / self.seconds)
    }
}

impl RawFreq {
    /// Converts frequency into time.
    #[must_use]
    pub fn raw_time(self) -> RawTime {
        RawTime::new(1.0 / self.hz)
    }
}

impl Time {
    /// Converts time into frequency.
    #[must_use]
    pub fn freq(self) -> Freq {
        Freq::new(1.0 / self.samples.into_f64())
    }
}

impl Freq {
    /// Converts frequency into time.
    #[must_use]
    pub fn time(self) -> Time {
        Time::new(FracInt::from_f64(1.0 / self.samples))
    }
}

impl Mul<RawTime> for SampleRate {
    type Output = Time;

    fn mul(self, rhs: RawTime) -> Time {
        Time::new(FracInt::from_f64(f64::from(self.0) * rhs.seconds))
    }
}

impl Mul<SampleRate> for RawTime {
    type Output = Time;

    fn mul(self, rhs: SampleRate) -> Time {
        rhs * self
    }
}

impl Div<SampleRate> for Time {
    type Output = RawTime;

    fn div(self, rhs: SampleRate) -> RawTime {
        RawTime::new((self.samples / rhs.0).into_f64())
    }
}

impl Mul<Freq> for SampleRate {
    type Output = RawFreq;

    fn mul(self, rhs: Freq) -> RawFreq {
        RawFreq::new(f64::from(self.0) * rhs.samples)
    }
}

impl Mul<SampleRate> for Freq {
    type Output = RawFreq;

    fn mul(self, rhs: SampleRate) -> RawFreq {
        rhs * self
    }
}

impl Div<SampleRate> for RawFreq {
    type Output = Freq;

    fn div(self, rhs: SampleRate) -> Freq {
        Freq::new(self.hz / f64::from(rhs))
    }
}
