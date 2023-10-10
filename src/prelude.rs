//! Imports the most commmon traits and structures.

#[cfg(feature = "hound")]
pub use crate::buffer::wav::*;

pub use crate::{
    buffer as buf, control as ctr,
    curves::{
        interpolate::{
            CubicStretch, DropStretch, HermiteStretch, Interpolate, LinearStretch, Stretch,
        },
        *,
    },
    effects::{
        adsr::{Adsr, AdsrEnvelope},
        delay::*,
        filter::{Biquad, Coefficients, Filter, Filtered},
        freq::*,
        mix::*,
        pan::{LinearPanner, MixedPanner, Panner, PowerPanner},
        trailing::*,
        vol::*,
        *,
    },
    gen::poly as ply,
    generators as gen,
    generators::*,
    map, pos, sample as smp, sgn,
    signal::*,
    units as unt,
};
