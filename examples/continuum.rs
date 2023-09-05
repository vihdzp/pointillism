//! Generative ambient music.
//!
//! Layering multiple notes like this leads to interesting emergent effects, such as constructive
//! and destructive interference, and beating from the [septimal
//! comma](https://en.wikipedia.org/wiki/Septimal_comma) 64/63 = 4/3 × 4/3 × 4/7.
//!
//! Sounds even smoother after some post-processing reverb.

use pointillism::prelude::*;
use rand::Rng;

fn main() {
    // Frequency of base note.
    const BASE: RawFreq = RawFreq::new(400.0);
    // Length of each note.
    const NOTE_LEN: RawTime = RawTime::new(3.0);
    // Length of released note.
    const RELEASE_LEN: RawTime = RawTime::new(45.0);

    // Length of song in notes.
    const NOTE_COUNT_LEN: u16 = 200;
    // Number of notes actually played (accounting for fade-out).
    const NOTE_COUNT: u16 = 185;

    let note_len = Time::from_raw_default(NOTE_LEN);

    // Envelope for the wave shape.
    let shape_env = Comp::new(Saw, Linear::rescale_sgn(0.75, 0.5));

    // Each oscillator is a function of frequency and panning angle.
    let osc = |freq, angle| {
        pointillism::effects::pan::Panner::mixed(
            MutSgn::new(
                AdsrEnvelope::new_adsr(
                    // Saw-triangle wave with specified frequency.
                    LoopGen::new(SawTri::saw(), freq),
                    // ADSR envelope with long attack, very long release.
                    note_len,
                    Time::ZERO,
                    Vol::FULL,
                    Time::from_raw_default(RELEASE_LEN),
                ),
                OnceGen::new(shape_env, note_len),
                // Smoothly interpolates between a saw and a triangle wave.
                FnWrapper::new(
                    |sgn: &mut AdsrEnvelope<LoopGen<Stereo, SawTri>>, val: Env| {
                        sgn.sgn_mut().curve_mut().shape = val.0;
                    },
                ),
            ),
            angle,
        )
    };

    // Base frequency.
    let base = Freq::from_raw_default(BASE);
    // Frequency of note being played.
    let mut freq = base;

    // Possible values to multiply a frequency by.
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
    let mut index = 0;
    poly.add(index, osc(freq, 0.5));

    let note_len = Time::from_raw_default(NOTE_LEN);

    // The song loop.
    let poly_loop = Loop::new(
        vec![note_len],
        poly,
        FnWrapper::new(|poly: &mut Polyphony<_, _>| {
            // Stops the previous note.
            poly.stop(&index);
            index += 1;

            // Changes the frequency randomly.
            freq *= mults[rand::thread_rng().gen_range(0..mults.len())];

            // Clamps the frequency between two octaves.
            if freq >= 2.0 * base {
                freq /= 2.0;
            } else if freq <= base / 2.0 {
                freq *= 2.0;
            }

            // Plays a new note, as long as the song isn't about to end.
            if index < NOTE_COUNT {
                poly.add(index, osc(freq, rand::thread_rng().gen()));
            }
        }),
    );

    pointillism::create_from_sgn(
        "examples/continuum.wav",
        NOTE_COUNT_LEN * note_len,
        // 10.0 might be too much, but just to be safe from clipping.
        Volume::new(poly_loop, Vol::new(1.0 / 10.0)),
    )
    .unwrap();
}
