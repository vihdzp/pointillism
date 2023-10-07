//! Imports the most commmon traits and structures.

#[cfg(feature = "hound")]
pub use crate::curves::buffer::wav::*;

pub use crate::{
    curves::{
        buffer::*,
        interpolate::{
            CubicStretch, DropStretch, HermiteStretch, Interpolate, LinearStretch, Stretch,
        },
        *,
    },
    effects::{
        adsr::{Adsr, AdsrEnvelope},
        delay::*,
        filter::{design::QFactor, Biquad, Coefficients, Filter, Filtered},
        freq::*,
        mix::*,
        pan::{LinearPanner, MixedPanner, Panner, PowerPanner},
        trailing::*,
        vol::*,
        *,
    },
    generators::{
        poly::Polyphony,
        sequence::*,
        unison::{DetuneCurveSgn, DetuneSgn, Unison, UnisonCurve, UnisonRef},
        Val, *,
    },
    map::*,
    pos,
    sample::{ArrayLike, Audio, Env, Mono, Sample, SampleLike, Stereo},
    sgn,
    signal::*,
    units::{midi::MidiNote, Freq, Interval, RawFreq, RawTime, SampleRate, Time, Timer},
};
