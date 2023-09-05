//! A basic example.
//!
//! We play five triangle waves on top of each other, in oscillating pitches.

use pointillism::prelude::*;

fn main() {
    // Number of oscillators.
    const NUM_OSC: u8 = 5;
    // Base frequency.
    const BASE: RawFreq = RawFreq::new(400.0);
    // RawTime to complete a cycle.
    const TIME: RawTime = RawTime::new(10.0);

    let base = Freq::from_raw_default(BASE);
    let time = Time::from_raw_default(TIME);

    // Each of our oscillators is a function of phase.
    let osc = |phase| {
        MutSgn::new(
            // A triangle wave with a placeholder frequency.
            LoopGen::new(Tri, base),
            // A sine wave, which controls the pitch of the triangle wave.
            LoopGen::new_phase(Sin, Freq::from(NUM_OSC * time), phase),
            // The frequency of the triangle wave is a function of the sine wave
            // envelope value.
            FnWrapper::new(|sgn: &mut LoopGen<_, _>, val: Env| {
                *sgn.freq_mut() = base * (val.0 / 2.0 + 1.0);
            }),
        )
    };

    // Initialize oscillators with equally-spaced phases.
    let mut oscillators = Vec::new();
    for i in 0..NUM_OSC {
        oscillators.push(osc(Val::new(i as f64 / NUM_OSC as f64)));
    }

    pointillism::create("examples/five_osc.wav", 2u8 * time, |_| {
        oscillators.iter_mut().map(|osc| osc.next()).sum::<Mono>() / NUM_OSC as f64
    })
    .unwrap();
}
