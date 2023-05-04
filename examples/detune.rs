use pointillism::prelude::{unison::DetuneSgn, *};

const NUM: u8 = 5;
const SCALE: f64 = 1.0 / NUM as f64;

fn main() {
    let time = 60.0 * Time::SEC;
    let mut unison = DetuneSgn::<Mono, _, _>::new_detune(
        Saw,
        Freq::A3,
        NUM,
        OnceGen::new(Comp::new(Saw, Linear::rescale_sgn(0.0, SCALE)), time),
    );

    // You'll get a lot of interference without this step.
    unison.sgn_mut().randomize_phases();

    pointillism::create_from_sgn(
        "examples/detune.wav",
        1.2 * time,
        Volume::new(unison, Vol::new(SCALE)),
    )
    .unwrap();
}
