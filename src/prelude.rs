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
    freq::{midi::MidiNote, *},
    generators::{
        poly::Polyphony,
        sequence::{Loop, Sequence},
        unison::{DetuneCurveSgn, DetuneSgn, Unison, UnisonCurve},
        Val, *,
    },
    map::*,
    pos,
    sample::{Audio, Env, Mono, Sample, SampleLike, Stereo},
    sgn,
    signal::*,
    time::*,
    SAMPLE_RATE, SAMPLE_RATE_F64,
};
