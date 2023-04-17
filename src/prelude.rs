//! Imports the most commmon traits and structures.

pub use crate::{
    curves::*,
    effects::{
        adsr::{Adsr, Envelope},
        freq::*,
        mix::*,
        vol::*,
        MapSgn, ModSgn, MutSgn, Pw, PwMapSgn,
    },
    freq::*,
    generators::{
        curves::*,
        poly::Polyphony,
        sequence::{Loop, Sequence},
    },
    map::*,
    pos,
    sample::{Audio, Env, Mono, Sample, Stereo},
    sgn,
    signal::*,
    time::*,
    A4, SAMPLE_RATE, SAMPLE_RATE_F64,
};
