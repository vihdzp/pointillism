//! Imports the most commmon traits and structures.

pub use crate::{
    curves::{
        buffer::{Buffer, LoopBufGen, OnceBufGen},
        interpolate::{
            CubicStretch, DropStretch, HermiteStretch, Interpolate, LinearStretch, Stretch,
        },
        *,
    },
    effects::{
        adsr::{Adsr, AdsrEnvelope},
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
    units::{
        freq::{Freq, Interval},
        midi::MidiNote,
        time::{Time, Timer},
    },
    SAMPLE_RATE, SAMPLE_RATE_F64,
};
