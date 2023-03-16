//! Generative ambient music.

use pointillism::prelude::*;
use rand::Rng;

fn main() {
    // Frequency of base note.
    const BASE: Freq = Freq::new(400.0);

    // Length of each note.
    const NOTE_LEN: Time = Time::new(3.0);

    // Each oscillator is a function of frequency and panning angle.
    let osc = |freq, angle| {
        pointillism::effects::pan::MixedPanner::new(
            Envelope::new_generic(
                AdsrEnvelope::new(
                    // Sine wave with specified frequency.
                    CurveGen::new(SawTri::saw(), freq),
                    // ADSR envelope with long attack, long release.
                    Adsr::new(5.0 * NOTE_LEN, 0.2 * NOTE_LEN, 0.6, 15.0 * NOTE_LEN),
                ),
                CurveEnv::new(InvSaw, NOTE_LEN),
                // Smoothly interpolates between a saw and a triangle wave.
                FnWrapper(|sgn: &mut AdsrEnvelope<CurveGen<SawTri>>, val: f64| {
                    sgn.sgn_mut().sgn.curve.shape = val / 4.0 + 0.75;
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

    // Initializes a new `Polyphony` object, plays a single note.
    let mut poly = Polyphony::new();
    poly.add(osc(freq, 0.5));

    // The song loop.
    let mut poly_loop = Loop::new(
        vec![NOTE_LEN],
        poly,
        FnWrapper(|poly: &mut Polyphony<_>, event: Event| {
            // Stops the previous note.
            poly.stop(event.idx);

            // Changes the frequency randomly.
            freq *= mults[rand::thread_rng().gen_range(0..mults.len())];

            // Cramps the frequency between two octaves.
            if freq >= 2.0 * BASE {
                freq /= 2.0;
            } else if freq <= BASE / 2.0 {
                freq *= 2.0;
            }

            // Plays a new note, as long as the song isn't about to end.
            if event.time < 185.0 * NOTE_LEN {
                poly.add(osc(freq, rand::thread_rng().gen()));
            }
        }),
    );

    pointillism::create("examples/continuum.wav", 200.0 * NOTE_LEN, |_| {
        poly_loop.next() / 2.0
    })
}
