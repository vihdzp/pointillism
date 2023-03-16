//! Imports the most commmon traits and structures.

pub use crate::{
    basic::*,
    effects::{
        adsr::{Adsr, AdsrEnvelope},
        Envelope,
    },
    generators::{curves::*, CurveEnv, CurveGen, LoopCurveEnv},
    sample::{AudioSample, Env, Mono, Sample, Stereo},
    signal::{Signal, StopSignal},
};
