//! A basic example.
//!
//! We play five triangle waves on top of each other, in oscillating pitches.

use pointillism::prelude::*;

fn main() {
    // Number of oscillators.
    const NUM_OSC: u8 = 5;

    // Base frequency.
    const BASE: Freq = Freq::new(400.0);

    // Time to complete a cycle.
    const TIME: Time = Time::new(10.0);

    // Each of our oscillators is a function of phase.
    let osc = |phase| {
        MutSgn::new_generic(
            // A triangle wave with a placeholder frequency.
            CurveGen::new(SawTri::tri(), BASE),
            // A sine wave, which controls the pitch of the triangle wave.
            LoopCurveEnv::new(Sin::new(phase), (NUM_OSC as f64 * TIME).into()),
            // The frequency of the triangle wave is a function of the sine wave
            // envelope value.
            FnWrapper::new(|sgn: &mut CurveGen<_>, val| {
                *sgn.freq_mut() = BASE * (val / 2.0 + 1.0);
            }),
        )
    };

    // Initialize oscillators with equally-spaced phases.
    let mut oscillators = Vec::new();
    for i in 0..NUM_OSC {
        oscillators.push(osc(i as f64 / NUM_OSC as f64));
    }

    pointillism::create("examples/five_osc.wav", 2.0 * TIME, |_| {
        oscillators.iter_mut().map(|osc| osc.next()).sum::<Mono>() / NUM_OSC as f64
    })
    .unwrap();
}
