//! Imports the most commmon traits and structures.

pub use crate::{
    curves::*,
    effects::{
        adsr::{Adsr, Envelope},
        freq::*,
        mix::*,
        sequence::{Loop, Sequence},
        vol::*,
        MutSgn,
    },
    freq::*,
    generators::{curves::*, poly::Polyphony},
    map::*,
    pos,
    sample::{Audio, Env, Mono, Sample, Stereo},
    sgn,
    signal::*,
    time::*,
    A4, SAMPLE_RATE, SAMPLE_RATE_F64,
};
