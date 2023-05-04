//! Tests some signal detuning.

use pointillism::prelude::{unison::DetuneSgn, *};

/// The number of waves playing.
const NUM: u8 = 5;
const SCALE: f64 = 1.0 / NUM as f64;

fn main() {
    const LEN: Time = Time::MIN;
    let mut unison = DetuneSgn::<Mono, _, _>::new_detune(
        Saw,
        Freq::A3,
        NUM,
        OnceGen::new(Comp::new(Saw, Linear::rescale_sgn(0.0, SCALE)), LEN),
    );

    // Try removing this!
    unison.sgn_mut().randomize_phases();

    pointillism::create_from_sgn(
        "examples/detune.wav",
        1.2 * LEN,
        Volume::new(unison, Vol::new(SCALE)),
    )
    .unwrap();
}
