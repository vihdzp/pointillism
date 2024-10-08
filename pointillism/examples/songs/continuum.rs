//! A wall of generative ambient synth noise.
//!
//! Layering multiple notes like this leads to interesting emergent effects, such as constructive
//! and destructive interference, and beating from the [septimal
//! comma](https://en.wikipedia.org/wiki/Septimal_comma) 64/63 = 4/3 × 4/3 × 4/7.
//!
//! Sounds even smoother after some post-processing reverb.

use pointillism::prelude::*;
use rand::Rng;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;

/// Possible values to multiply a frequency by.
const MULTS: [f64; 6] = [
    4.0 / 3.0,
    3.0 / 4.0,
    3.0 / 2.0,
    2.0 / 3.0,
    7.0 / 4.0,
    4.0 / 7.0,
];

fn main() {
    // Frequency of base note.
    const BASE: unt::RawFreq = unt::RawFreq::new(400.0);
    // Length of each note.
    const NOTE_LEN: unt::RawTime = unt::RawTime::new(3.0);
    // Length of released note.
    const RELEASE_LEN: unt::RawTime = unt::RawTime::new(45.0);

    // Length of song in notes.
    const NOTE_COUNT_LEN: u16 = 200;
    // Number of notes actually played (accounting for fade-out).
    const NOTE_COUNT: u16 = 185;

    let note_len = unt::Time::from_raw(NOTE_LEN, SAMPLE_RATE);

    // Envelope for the wave shape.
    let shape_env = map::Comp::new(crv::Saw, map::Linear::rescale_sgn(0.75, 0.5));

    // Each oscillator is a function of frequency and panning angle.
    let osc = |freq, angle| {
        pointillism::effects::pan::Panner::mixed(
            eff::MutSgn::new(
                eff::env::AdsrEnv::new_adsr(
                    // Saw-triangle wave with specified frequency.
                    gen::Loop::new(crv::SawTri::saw(), freq),
                    // ADSR envelope with long attack, very long release.
                    eff::env::Adsr::new(
                        note_len,
                        unt::Time::ZERO,
                        unt::Vol::FULL,
                        unt::Time::from_raw(RELEASE_LEN, SAMPLE_RATE),
                    ),
                ),
                gen::Once::new(shape_env, note_len),
                // Smoothly interpolates between a saw and a triangle wave.
                map::Func::new(
                    |sgn: &mut eff::env::AdsrEnv<gen::Loop<smp::Stereo, crv::SawTri>>,
                     val: smp::Env| {
                        sgn.sgn_mut().curve_mut().shape = unt::Val::new(val.0);
                    },
                ),
            ),
            angle,
        )
    };

    // Base frequency.
    let base = unt::Freq::from_raw(BASE, SAMPLE_RATE);
    // Frequency of note being played.
    let mut freq = base;

    // Initializes a new `Polyphony` object, plays a single note, centered.
    let mut poly = poly::Polyphony::new();
    let mut index = 0;
    poly.add(index, osc(freq, 0.5));

    let note_len = unt::Time::from_raw(NOTE_LEN, SAMPLE_RATE);

    // The song loop.
    let poly_loop = ctr::Loop::new(
        vec![note_len],
        poly,
        map::Func::new(|poly: &mut poly::Polyphony<_, _>| {
            // Stops the previous note.
            poly.stop(&index);
            index += 1;

            // Plays a new note, as long as the song isn't about to end.
            if index < NOTE_COUNT {
                // Changes the frequency randomly.
                freq *= MULTS[rand::thread_rng().gen_range(0..MULTS.len())];

                // Clamps the frequency between two octaves.
                if freq >= 2.0 * base {
                    freq /= 2.0;
                } else if freq <= base / 2.0 {
                    freq *= 2.0;
                }

                poly.add(index, osc(freq, rand::thread_rng().gen()));
            }
        }),
    );

    Song::new(
        NOTE_COUNT_LEN * note_len,
        SAMPLE_RATE,
        // 10.0 might be too much, but just to be safe from clipping.
        eff::Volume::new(poly_loop, unt::Vol::new(1.0 / 10.0)),
    )
    .export("pointillism/examples/continuum.wav");
}
