//! A basic example.
//!
//! We play five triangle waves on top of each other, in oscillating pitches.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;

fn main() {
    // Number of oscillators.
    const NUM_OSC: usize = 5;
    // Base frequency.
    const BASE: unt::RawFreq = unt::RawFreq::new(400.0);
    // RawTime to complete a cycle.
    const TIME: unt::RawTime = unt::RawTime::new(10.0);

    let base = unt::Freq::from_raw(BASE, SAMPLE_RATE);
    let time = unt::Time::from_raw(TIME, SAMPLE_RATE);

    // Each of our oscillators is a function of phase.
    let osc = |phase| {
        eff::MutSgn::new(
            // A triangle wave with a placeholder frequency.
            gen::Loop::new(crv::Tri, base),
            // A sine wave, which controls the pitch of the triangle wave.
            gen::Loop::new_phase(crv::Sin, (NUM_OSC * time).freq(), phase),
            // The frequency of the triangle wave is a function of the sine wave
            // envelope value.
            map::Func::new(|sgn: &mut gen::Loop<_, _>, val: smp::Env| {
                *sgn.freq_mut() = base * (val.0 / 2.0 + 1.0);
            }),
        )
    };

    // Initialize oscillators with equally-spaced phases.
    let mut oscillators: [_; NUM_OSC] =
        std::array::from_fn(|i| osc(unt::Val::new(i as f64 / NUM_OSC as f64)));

    Song::new(2u8 * time, SAMPLE_RATE, |_| {
        oscillators
            .iter_mut()
            .map(|osc| osc.next())
            .sum::<smp::Mono>()
            / NUM_OSC as f64
    })
    .export("pointillism/examples/fiveosc.wav");
}
