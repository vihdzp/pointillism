//! A compositional library for musical composition.
//!
//! # Examples
//!
//! If you want to see pointillism in action and what it's capable of, run the examples in the
//! `examples` folder. There's also many simple examples scattered throughout the source code,
//! showing off different features.
//!
//! For a starting example, see the [`Song`] docs.
//!
//! **Note:** Some examples may be loud, dissonant, and/or jarring. Hearing discretion is advised.
//!
//! # Design
//!
//! The default way in which pointillism outputs audio is by writing sample by sample into a 32-bit
//! floating point `.wav` file. Internal calculations use 64-bit floating points.
//!
//! For convenience, the [`Signal`] trait is provided. Types implementing this trait generate sample
//! data frame by frame. If the type also implements [`SignalMut`], it can be advanced or
//! retriggered.
//!
//! Signals may be composed to create more complex signals, using for instance the [`eff::MapSgn`]
//! and [`eff::MutSgn`] structs. Moreover, you can implement the [`Signal`] and [`SignalMut`] traits
//! for your own structs, giving you vast control over the samples you're producing.
//!
//! ## Naming scheme
//!
//! The `pointillism` code has a lot of moving parts, and a bunch of similarly named types. Because
//! of this, we rely on the `prelude` to categorize things neatly.
//!
//! Every type has a three-letter namespace which helps categorizes it. The main namespaces are as
//! follows:
//!
//! | Namespace | Full Name | Contents |
//! |-|-|-|
//! | [`buf`] | `buffers` | Audio buffers and associated traits.
//! | [`crv`] | `curves` | Basic oscillator shapes, and builder methods for more complex ones (in the future).
//! | [`ctr`] | `control` | Control structures, which allow for events to happen at specified time intervals.
//! | [`eff`] | `effects` | For effects, meaning types that alter other signals.
//! | [`map`] | `map` | Basic maps and associated traits.
//! | [`gen`] | `generators` | Types that generate a signal "on their own". This includes the basic oscillators like [`gen::Loop`] and [`gen::Once`].
//! | [`sgn`] | `signal` | Traits on signals, including the basic [`Signal`] and [`SignalMut`].
//! | [`smp`] | `smp` | Basic traits and types for sample types, including [`smp::Mono`] and [`smp::Stereo`].
//! | [`rtn`] | `routing` | Structures for mixing or combining different signals together.
//! | [`unt`] | `units` | Different units for musical measurement, and associated arithmetical boilerplate.
//!
//! Note that traits are always imported when the prelude is imported. This simplifies some complex
//! `impl` declarations, and also makes the trait methods available whenever.
//!
//! Some of these namespaces also contain further nested namespaces, almost always three letters.
//! See the documentation for the full breakdown.
//!
//! ## Compile-time
//!
//! You can think of pointillism as a compile-time modular synthesizer, where every new struct is
//! its own module.
//!
//! Advantages of this design are extensibility and generality. It's relatively easy to create a
//! highly customizable and complex signal with many layers by composing some functions together.
//!
//! The downside is that these synths end up having unwieldy type signatures. Moreso, it's really
//! hard to build synths in real time.
//!
//! ## Features
//!
//! The project uses the following features:
//!
//! | Feature | Enables |
//! |-|-|
//! | [`hound`](https://docs.rs/hound/latest/hound)* | Saving songs as WAV files. |
//! | [`cpal`](https://docs.rs/cpal/latest/cpal) | Playing songs in a dedicated thread. |
//! | [`midly`](https://docs.rs/midly/latest/midly) | Reading and playing back MIDI files. |
//! | [`human-duration`](https://docs.rs/human-duration/latest/human_duration)* | Pretty-printing for the [`unt::RawTime`] type. |
//!
//! \* Features marked with an asterisk are enabled by default.
//!
//! # Goals
//!
//! Future goals of pointillism are:
//!
//! - (Better) algorithmic reverbs
//! - Limiters, compressors, sidechaining
//! - [Me](https://viiii.bandcamp.com) making a whole album with it :D
//!
//! # Disclaimer
//!
//! This is a passion project made by one college student learning about DSP. I make no guarantees
//! on it being well-designed, well-maintained, or usable for your own goals.
//!
//! If you just want to make music with code, and especially if you enjoy live feedback,
//! [SuperCollider](https://supercollider.github.io) and [Pure Data](https://puredata.info) will
//! most likely be better alternatives for you.
//!
//! That said, if you happen to stumble across this and make something cool, please let me know!

#![warn(clippy::cargo)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod buffers;
pub mod control;
pub mod curves;
pub mod effects;
pub mod generators;
pub mod map;
pub mod routing;
pub mod sample;
pub mod signal;
pub mod units;

#[cfg(feature = "cpal")]
pub mod cpal;
#[cfg(feature = "hound")]
pub use with_hound::*;

// Needed so that the docs render properly.
use crate::prelude::*;

/// A generic "out of bounds" error message.
pub const OOB: &str = "index out of bounds";

/// Increments a value in `0..len` by one, and wraps it around.
///
/// This should be marginally more efficient than `value = (value + 1) % len`, as it avoids the more
/// costly modulo operation.
pub(crate) fn mod_inc(len: usize, value: &mut usize) {
    *value += 1;

    if *value == len {
        *value = 0;
    }
}

/// A trait for some function that returns samples frame by frame.
pub trait SongFunc {
    /// The type of samples returned.
    type Sample: Audio;

    /// Gets the next sample.
    fn eval(&mut self, time: unt::Time) -> Self::Sample;
}

/// Wraps a `FnMut(unt::Time) -> A` so that it implements the [`SongFunc`] trait.
pub struct Func<T>(T);

/// Wraps a [`SignalMut`] so that it implements the [`SongFunc`] trait.
pub struct Sgn<S>(S);

/// Wraps a [`&mut SignalMut`](SignalMut) so that it implements the [`SongFunc`] trait.
pub struct SgnMut<'a, S>(&'a mut S);

impl<A: Audio, T: FnMut(unt::Time) -> A> SongFunc for Func<T> {
    type Sample = A;
    fn eval(&mut self, time: units::Time) -> Self::Sample {
        (self.0)(time)
    }
}

impl<S: SignalMut> SongFunc for Sgn<S>
where
    S::Sample: Audio,
{
    type Sample = S::Sample;
    fn eval(&mut self, _: units::Time) -> Self::Sample {
        self.0.next()
    }
}

impl<'a, S: SignalMut> SongFunc for SgnMut<'a, S>
where
    S::Sample: Audio,
{
    type Sample = S::Sample;
    fn eval(&mut self, _: units::Time) -> Self::Sample {
        self.0.next()
    }
}

/// Represents a song, a piece of code that can be evaluated frame by frame to generate succesive
/// samples. The duration of the file is exactly rounded down to the sample. The song will be mono
/// or stereo, depending on whether the passed function returns [`smp::Mono`] or [`smp::Stereo`].
///
/// See the `examples` folder for example creations.
///
/// ## Example
///
/// We make the most basic song possible: a single sine wave.
///
/// ```
/// # use pointillism::prelude::*;
/// // Project sample rate.
/// const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;
///
/// // File duration.
/// let length = unt::Time::from_sec(1.0, SAMPLE_RATE);
/// // Sine wave frequency.
/// let freq = unt::Freq::from_hz(440.0, SAMPLE_RATE);
///
/// // We create a mono signal that loops through a sine curve at the specified frequency.
/// let mut sgn = gen::Loop::<smp::Mono, _>::new(crv::Sin, freq);
///
/// // Export to file.
/// Song::new_sgn(length, SAMPLE_RATE, &mut sgn).export("examples/sine.wav");
/// ```
pub struct Song<F: SongFunc> {
    /// The length of the song in samples.
    length: unt::Time,
    /// The sample rate of the song.
    sample_rate: unt::SampleRate,
    /// The [`SongFunc`] that generates the song.
    song: F,
}

impl<F: SongFunc> Song<F> {
    pub const fn new_raw(length: unt::Time, sample_rate: unt::SampleRate, song: F) -> Self {
        Self {
            length,
            sample_rate,
            song,
        }
    }
}

impl<A: Audio, F: FnMut(unt::Time) -> A> Song<Func<F>> {
    /// Creates a new [`Song`] from a function, taking the elapsed time as an argument. To instead
    /// take in a signal, see [`Self::new_sgn`].
    ///
    /// The resulting WAV file will be mono or stereo, depending on whether the passed function
    /// returns [`smp::Mono`] or [`smp::Stereo`].
    ///
    /// ## Example
    ///
    /// For an example, see the [`Song`] docs.
    pub const fn new(length: unt::Time, sample_rate: unt::SampleRate, song: F) -> Self {
        Self::new_raw(length, sample_rate, Func(song))
    }
}

impl<'a, S: SignalMut> Song<SgnMut<'a, S>>
where
    S::Sample: Audio,
{
    /// A convenience function to create a [`new`](Self::new) song from a given signal. The signal
    /// is not consumed. To instead take in a function, see [`Self::new`]. If you have to own the
    /// song, use [`Self::new_sgn_owned`].
    ///
    /// The resulting WAV file will be mono or stereo, depending on whether the passed function
    /// returns [`smp::Mono`] or [`smp::Stereo`].
    ///
    /// ## Example
    ///
    /// For an example, see the [`Song`] docs.
    pub fn new_sgn(length: unt::Time, sample_rate: unt::SampleRate, sgn: &'a mut S) -> Self
    where
        S::Sample: Audio,
    {
        Self::new_raw(length, sample_rate, SgnMut(sgn))
    }
}

impl<S: SignalMut> Song<Sgn<S>>
where
    S::Sample: Audio,
{
    /// A convenience function to create a [`new`](Self::new) song from a given signal. The signal
    /// is consumed. To instead take in a function, see [`Self::new`]. If you don't have to own the
    /// song, use [`Self::new_sgn`].
    ///
    /// The resulting WAV file will be mono or stereo, depending on whether the passed function
    /// returns [`smp::Mono`] or [`smp::Stereo`].
    ///
    /// ## Example
    ///
    /// For an example, see the [`Song`] docs.
    pub fn new_sgn_owned(length: unt::Time, sample_rate: unt::SampleRate, sgn: S) -> Self
    where
        S::Sample: Audio,
    {
        Self::new_raw(length, sample_rate, Sgn(sgn))
    }
}

/// Methods that require [`hound`].
#[cfg(feature = "hound")]
mod with_hound {
    use crate::prelude::*;

    /// The [specification](hound::WavSpec) for the output file.
    #[must_use]
    pub const fn spec(channels: u8, sample_rate: unt::SampleRate) -> hound::WavSpec {
        hound::WavSpec {
            channels: channels as u16,
            sample_rate: sample_rate.0,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        }
    }

    impl<F: SongFunc> Song<F> {
        /// Exports a song as a WAV file. Requires the [`hound`] feature.
        ///
        /// ## Errors
        ///
        /// This should only return an error in the case of an IO error.
        pub fn export_res<P: AsRef<std::path::Path>>(&mut self, filename: P) -> hound::Result<()> {
            let length = self.length.samples.int();
            let mut writer =
                hound::WavWriter::create(filename, spec(F::Sample::size_u8(), self.sample_rate))?;

            let mut time = unt::Time::ZERO;
            for _ in 0..length {
                self.song.eval(time).write(&mut writer)?;
                time.advance();
            }

            writer.finalize()
        }

        /// A convenience function for calling [`Self::export_res`], panicking in case of an IO
        /// error.
        ///
        /// ## Panics
        ///
        /// Panics in case of an IO error.
        pub fn export<P: AsRef<std::path::Path>>(&mut self, filename: P) {
            self.export_res(filename).expect("IO error");
        }
    }
}

/// The crate prelude.
///
/// See the readme for a full list of abbreviations.
pub mod prelude {
    // Abbreviate module names.
    pub use crate::{
        buffers as buf, control as ctr, curves as crv, effects as eff, generators as gen, map,
        routing as rtn, sample as smp, signal as sgn, units as unt,
    };

    // Import traits.
    pub use crate::{
        buf::{Buffer, BufferMut, Ring},
        eff::flt::FilterMap,
        map::{Env, Map, Mut},
        sgn::{Base, Done, Frequency, Panic, Signal, SignalMut, Stop},
        smp::{Array, Audio, Sample, SampleBase},
        Song, SongFunc,
    };
    pub(crate) use sgn::impl_base;
}
