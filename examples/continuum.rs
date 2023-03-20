//! Generative ambient music.
//!
//! Layering multiple notes like this leads to interesting emergent effects,
//! such as constructive and destructive interference, and beating from the
//! [septimal comma](https://en.wikipedia.org/wiki/Septimal_comma) 64/63 = 4/3 ×
//! 4/3 × 4/7.
//!
//! Sounds even smoother after some post-processing reverb.

use pointillism::prelude::*;
use rand::Rng;

fn main() {
    // Frequency of base note.
    const BASE: Freq = Freq::new(400.0);

    // Length of each note.
    const NOTE_LEN: Time = Time::new(3.0);

    // Length of released note.
    const RELEASE_LEN: Time = Time::new(45.0);

    // Number of notes in song.
    const NOTE_COUNT: u16 = 200;

    // Envelope for the wave shape.
    let shape_env = Comp::new_generic(Saw::new(), Linear::rescale(-1.0, 1.0, 0.75, 0.5));

    // Each oscillator is a function of frequency and panning angle.
    let osc = |freq, angle| {
        pointillism::effects::pan::MixedPanner::new(
            Envelope::new_generic(
                AdsrEnvelope::new(
                    // Saw-triangle wave with specified frequency.
                    CurveGen::new(SawTri::saw(), freq),
                    // ADSR envelope with long attack, very long release.
                    Adsr::new(NOTE_LEN, Time::ZERO, 1.0, RELEASE_LEN),
                ),
                CurveEnv::new(shape_env, NOTE_LEN),
                // Smoothly interpolates between a saw and a triangle wave.
                FnWrapper::new(|sgn: &mut AdsrEnvelope<CurveGen<SawTri>>, val: f64| {
                    sgn.sgn_mut().curve_mut().shape = val;
                }),
            ),
            angle,
        )
    };

    // Frequency of note being played.
    let mut freq = BASE;

    // Possible values to multiply a frequency by,
    let mults = [
        4.0 / 3.0,
        3.0 / 4.0,
        3.0 / 2.0,
        2.0 / 3.0,
        7.0 / 4.0,
        4.0 / 7.0,
    ];

    // Initializes a new `Polyphony` object, plays a single note, centered.
    let mut poly = Polyphony::new();
    poly.add(osc(freq, 0.5));

    // The song loop.
    let mut poly_loop = Loop::new(
        vec![NOTE_LEN],
        poly,
        FnWrapper::new(|poly: &mut Polyphony<_>, event: Event| {
            // Stops the previous note.
            poly.stop(event.idx);

            // Changes the frequency randomly.
            freq *= mults[rand::thread_rng().gen_range(0..mults.len())];

            // Clamps the frequency between two octaves.
            if freq >= 2.0 * BASE {
                freq /= 2.0;
            } else if freq <= BASE / 2.0 {
                freq *= 2.0;
            }

            // Plays a new note, as long as the song isn't about to end.
            if event.time < (NOTE_COUNT as f64 - 1.0) * NOTE_LEN - RELEASE_LEN {
                poly.add(osc(freq, rand::thread_rng().gen()));
            }
        }),
    );

    pointillism::create(
        "examples/continuum.wav",
        NOTE_COUNT as f64 * NOTE_LEN,
        |_| {
            // 10.0 might be too much, but just to be safe from clipping.
            poly_loop.next() / 10.0
        },
    )
    .unwrap();
}
