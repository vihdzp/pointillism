//! Imports the most commmon traits and structures.

pub use crate::{
    basic::*,
    effects::{
        adsr::{Adsr, AdsrEnvelope},
        pan::Volume,
        sequence::{Event, Loop, Sequence},
        Envelope,
    },
    generators::{curves::*, poly::Polyphony, CurveEnv, CurveGen, LoopCurveEnv},
    sample::{AudioSample, Env, Mono, Sample, Stereo},
    signal::{Signal, StopSignal},
};
