//! Imports the most commmon traits and structures.

pub use crate::{
    effects::{
        adsr::{Adsr, AdsrEnvelope},
        freq::*,
        sequence::{Event, Loop, Sequence},
        vol::*,
        Envelope,
    },
    freq::*,
    generators::{curves::*, poly::Polyphony, CurveEnv, CurveGen, LoopCurveEnv},
    map::*,
    pos,
    sample::{AudioSample, Env, Mono, Sample, Stereo},
    sgn,
    signal::{MapSgn, Signal, StopSignal},
    time::*,
    A4, SAMPLE_RATE,
};
