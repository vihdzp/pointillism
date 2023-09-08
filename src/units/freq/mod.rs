//! Implements the types for frequency: [`RawFreq`], [`Freq`], [`Interval`].
//!
//! ## Equal division of the octave
//!
//! Most music is written using 12 notes. These form an [equal division of the
//! octave](https://en.wikipedia.org/wiki/Equal_temperament). However, much interesting music can be
//! made that uses other more complicated scales. For this reason, although we provide convenience
//! methods for 12-EDO, we also provide many other methods that allow you to specify your own equal
//! division of the octave. If you want something even more general, you can always input the raw
//! numbers yourself.

mod interval;
mod raw;

pub use {interval::Interval, raw::RawFreq};

use crate::{
    prelude::MidiNote,
    units::{SampleRate, A4_MIDI},
};

use std::ops::{Div, DivAssign, Mul, MulAssign};

/// A frequency, measured in **inverse samples**.
///
/// Note that in order to convert between a [`RawFreq`] in hertz and this type, you must know the
/// [`SampleRate`]. See [`Self::from_raw`].
///
/// ## Type invariant checking
///
/// It's impractical to constantly check that frequencies are positive, and there's not really any
/// simple ways to mess this up, so we don't check the invariant. That's not to say things won't go
/// wrong if the invariant is broken!
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Freq {
    /// The frequency in inverse samples.
    samples: f64,
}

impl Freq {
    /// Zero frequency.
    ///
    /// You can use this as a dummy value, or as the frequency of a static wave (why though?).
    pub const ZERO: Self = Self::new(0.0);

    /// Initializes a frequency in **inverse samples**.
    ///
    /// If you want to use the more natural unit of hertz, see [`Self::from_raw`].
    #[must_use]
    pub const fn new(samples: f64) -> Self {
        Self { samples }
    }

    /// The frequency in inverse samples.
    #[must_use]
    pub const fn samples(self) -> f64 {
        self.samples
    }

    /// Converts [`RawFreq`] into [`Freq`], using the specified sample rate.
    #[must_use]
    pub fn from_raw(raw: RawFreq, sample_rate: SampleRate) -> Self {
        raw / sample_rate
    }

    /// Converts [`RawFreq`] into [`Freq`], using the default sample rate.
    #[must_use]
    pub fn from_raw_default(raw: RawFreq) -> Self {
        Self::from_raw(raw, SampleRate::default())
    }

    /// Initializes a [`Freq`] from the value in hertz, and a sample rate.
    #[must_use]
    pub fn from_hz(hz: f64, sample_rate: SampleRate) -> Self {
        Self::from_raw(RawFreq::new(hz), sample_rate)
    }

    /// Initializes a [`Freq`] from the value in hertz, using the default sample rate.
    #[must_use]
    pub fn from_hz_default(hz: f64) -> Self {
        Self::from_hz(hz, SampleRate::default())
    }

    /// Converts [`Freq`] into [`RawFreq`], using the specified sample rate.
    #[must_use]
    pub fn into_raw(self, sample_rate: SampleRate) -> RawFreq {
        RawFreq::new(self.samples() * f64::from(sample_rate))
    }

    /// Converts [`Freq`] into [`RawFreq`], using the default sample rate.
    #[must_use]
    pub fn into_raw_default(self) -> RawFreq {
        self.into_raw(SampleRate::default())
    }

    /// Bends a note by a number of notes in a given `edo`.
    ///
    /// You can use this to generate an scale in some EDO, based on some note.
    ///
    /// ## Example
    ///
    /// For an example, see the functionally identical [`RawFreq::bend_edo`].
    #[must_use]
    pub fn bend_edo(self, edo: u16, bend: f64) -> Self {
        Interval::edo_note(edo, bend) * self
    }

    /// Bends a note by a number of notes in 12-EDO.
    ///
    /// ## Example
    ///
    /// For an example, see the functionally identical [`RawFreq::bend`].
    #[must_use]
    pub fn bend(self, bend: f64) -> Self {
        self.bend_edo(12, bend)
    }

    /// Initializes a frequency from a MIDI note.
    ///
    /// The note for `A4`, which depends on both the tuning and the sample rate, must be specified.
    #[must_use]
    pub fn new_midi(a4: Self, note: MidiNote) -> Self {
        a4.bend(f64::from(note.note) - A4_MIDI)
    }

    /// Rounds this frequency to the nearest (fractional) MIDI note.
    #[must_use]
    fn round_midi_aux(self, a4: Self) -> f64 {
        (self.samples() / a4.samples()).log2() * 12.0 + A4_MIDI
    }

    /// Rounds this frequency to the nearest MIDI note.
    ///
    /// The note for `A4`, which depends on both the tuning and the sample rate, must be specified.
    ///
    /// ## Panics
    ///
    /// Panics if this frequency is outside of the range for a [`MidiNote`].
    ///
    /// ## Example
    ///
    /// For an example, see the functionally identical [`RawFreq::round_midi_with`].
    #[must_use]
    pub fn round_midi(self, a4: Self) -> MidiNote {
        // Truncation should not occur in practice.
        #[allow(clippy::cast_possible_truncation)]
        MidiNote::new((self.round_midi_aux(a4).round()) as i16)
    }

    /// Rounds this frequency to the nearest MIDI note, and how many semitones away from this note
    /// it is.
    ///
    /// The note for `A4`, which depends on both the tuning and the sample rate, must be specified.
    ///
    /// ## Example
    ///
    /// For an example, see the functionally identical [`RawFreq::midi_semitones_with`].
    #[must_use]
    pub fn midi_semitones(self, a4: Self) -> (MidiNote, f64) {
        let note = self.round_midi_aux(a4);
        let round = note.round();

        // Truncation should not occur in practice.
        #[allow(clippy::cast_possible_truncation)]
        (MidiNote::new(round as i16), note - round)
    }
}

impl RawFreq {
    /// Converts [`Freq`] into [`RawFreq`], using the specified sample rate.
    ///
    /// To use the default 44.1 kHz sample rate, use [`Self::from_freq`].
    #[must_use]
    pub fn from_freq_with(freq: Freq, sample_rate: SampleRate) -> Self {
        Self::new(freq.samples() * f64::from(sample_rate))
    }

    /// Converts [`Freq`] into [`RawFreq`], using the specified sample rate.
    ///
    /// To specify the sample rate, use [`Self::from_freq`].
    #[must_use]
    pub fn from_freq(freq: Freq) -> Self {
        Self::from_freq_with(freq, SampleRate::default())
    }
}

/// We use `A4` as a default frequency, and 44.1 kHz as a default sample rate. This means that, for
/// instance,
///
/// ```
/// # use pointillism::prelude::*;
/// let osc = LoopGen::<Mono, Sin>::default();
/// ```
///
/// will result in a 440 Hz sine wave when sampled at 44.1 kHz.
impl Default for Freq {
    fn default() -> Self {
        Freq::from_raw(RawFreq::default(), SampleRate::default())
    }
}

impl std::fmt::Display for Freq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:.*} samples^-1",
            f.precision().unwrap_or_default(),
            self.samples,
        )
    }
}

impl Mul<f64> for Freq {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.samples * rhs)
    }
}

impl Mul<Freq> for f64 {
    type Output = Freq;

    fn mul(self, rhs: Freq) -> Freq {
        rhs * self
    }
}

impl MulAssign<f64> for Freq {
    fn mul_assign(&mut self, rhs: f64) {
        self.samples *= rhs;
    }
}

impl Div<f64> for Freq {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.samples / rhs)
    }
}

impl DivAssign<f64> for Freq {
    fn div_assign(&mut self, rhs: f64) {
        self.samples /= rhs;
    }
}

impl Mul<Interval> for Freq {
    type Output = Self;

    fn mul(self, rhs: Interval) -> Self {
        rhs.ratio * self
    }
}

impl Mul<Freq> for Interval {
    type Output = Freq;

    fn mul(self, rhs: Freq) -> Freq {
        self.ratio * rhs
    }
}

impl MulAssign<Interval> for Freq {
    fn mul_assign(&mut self, rhs: Interval) {
        self.samples *= rhs.ratio;
    }
}

impl Div<Interval> for Freq {
    type Output = Self;

    fn div(self, rhs: Interval) -> Self {
        Self::new(self.samples / rhs.ratio)
    }
}

impl DivAssign<Interval> for Freq {
    fn div_assign(&mut self, rhs: Interval) {
        self.samples /= rhs.ratio;
    }
}

impl Div for Freq {
    type Output = Interval;

    fn div(self, rhs: Freq) -> Interval {
        Interval::new(self.samples / rhs.samples)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test that pretty-printing a note works.
    #[test]
    fn print_a4() {
        assert_eq!(format!("{:#}", RawFreq::A4.bend(0.4)), "A4 +40c");
        assert_eq!(format!("{:#.2}", RawFreq::A4.bend(0.4)), "A4 +40.00c");
    }

    /// Test parsing a note.
    #[test]
    fn parse_a4() {
        let a4: RawFreq = "A4".parse().unwrap();
        assert_approx_eq::assert_approx_eq!(a4.hz, RawFreq::A4.hz);
    }

    /// Tests [`Freq`] printing.
    #[test]
    fn print_freq() {
        assert_eq!(format!("{:.6}", Freq::default()), "0.009977 samples^-1")
    }
}
