//! Imports the most commmon traits and structures.

pub use crate::{
    curves::*,
    effects::{
        adsr::{Adsr, Envelope},
        freq::*,
        mix::*,
        trailing::*,
        vol::*,
        *,
    },
    freq::*,
    generators::{
        poly::Polyphony,
        sequence::{Loop, Sequence},
        *,
    },
    map::*,
    pos,
    sample::{Audio, Env, Mono, Sample, Stereo},
    sgn,
    signal::*,
    time::*,
    SAMPLE_RATE, SAMPLE_RATE_F64,
};
