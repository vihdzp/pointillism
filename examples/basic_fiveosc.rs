//! A basic example.
//!
//! We play five triangle waves on top of each other, in oscillating pitches.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

fn main() {
    // Number of oscillators.
    const NUM_OSC: usize = 5;
    // Base frequency.
    const BASE: RawFreq = RawFreq::new(400.0);
    // RawTime to complete a cycle.
    const TIME: RawTime = RawTime::new(10.0);

    let base = Freq::from_raw(BASE, SAMPLE_RATE);
    let time = Time::from_raw(TIME, SAMPLE_RATE);

    // Each of our oscillators is a function of phase.
    let osc = |phase| {
        MutSgn::new(
            // A triangle wave with a placeholder frequency.
            LoopGen::new(Tri, base),
            // A sine wave, which controls the pitch of the triangle wave.
            LoopGen::new_phase(Sin, (NUM_OSC * time).freq(), phase),
            // The frequency of the triangle wave is a function of the sine wave
            // envelope value.
            FnWrapper::new(|sgn: &mut LoopGen<_, _>, val: Env| {
                *sgn.freq_mut() = base * (val.0 / 2.0 + 1.0);
            }),
        )
    };

    // Initialize oscillators with equally-spaced phases.
    let mut oscillators: [_; NUM_OSC] =
        std::array::from_fn(|i| osc(Val::new(i as f64 / NUM_OSC as f64)));

    pointillism::create("output/fiveosc.wav", 2u8 * time, SAMPLE_RATE, |_| {
        oscillators.iter_mut().map(|osc| osc.next()).sum::<Mono>() / NUM_OSC as f64
    })
    .unwrap();
}