//! Testing out distortion.
//!
//! We play various random 5-EDO notes in succession, and apply heavy distortion to the output for
//! fun effects.

use pointillism::prelude::*;
use rand::Rng;

/// Project sample rate.
///
/// Lower sample rates make for an even scrunchier sound.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::TELEPHONE;

fn main() {
    // Base frequency.
    const BASE: unt::RawFreq = unt::RawFreq::new(350.0);
    // Length of each note.
    const NOTE_LEN: unt::RawTime = unt::RawTime::new(5.0);

    let base = unt::Freq::from_raw(BASE, SAMPLE_RATE);
    let note_len = unt::Time::from_raw(NOTE_LEN, SAMPLE_RATE);

    // Each oscillator is a function of frequency.
    let osc = |freq| {
        eff::env::AdsrEnv::new(
            // Sine wave with specified frequency.
            gen::Loop::<smp::Stereo, _>::new(crv::Sin, freq),
            // ADSR envelope with long attack, long release.
            eff::env::Adsr::new(
                0.8 * note_len,
                0.2 * note_len,
                unt::Vol::new(0.8),
                1.5 * note_len,
            ),
        )
    };

    // Initializes a new `Polyphony` object, plays a single note.
    let mut poly = gen::Polyphony::new();
    let mut index = 0;
    poly.add(index, osc(base));

    // The song loop.
    let poly_loop = ctr::Loop::new(
        // Every NOTE_LEN seconds,
        vec![note_len],
        // we modify the signal `poly`,
        poly,
        map::Func::new(|poly: &mut gen::Polyphony<_, _>| {
            // by stopping the note we just played,
            poly.stop(&index);
            index += 1;

            // and adding a new one.
            poly.add(
                index,
                osc(base.bend_edo(5, rand::thread_rng().gen_range(0..=7) as f64)),
            );
        }),
    );

    // This gives a really weird effect.
    let mut dist = eff::PwMapSgn::cubic(poly_loop);
    Song::new_sgn(10u8 * note_len, SAMPLE_RATE, &mut dist)
        .export("pointillism/examples/distortion.wav");
}
