//! Testing out polyphonic signals.
//!
//! We play various random 5-EDO notes in succession, and apply heavy distortion
//! to the output for fun effects.

use pointillism::prelude::*;
use rand::Rng;

fn main() {
    // Base frequency.
    const BASE: Freq = Freq::new(350.0);

    // Length of each note.
    const NOTE_LEN: Time = Time::new(5.0);

    // Each oscillator is a function of frequency.
    let osc = |freq| {
        Envelope::new(
            // Sine wave with specified frequency.
            LoopGen::<Stereo, _>::new(Sin::sin(), freq),
            // ADSR envelope with long attack, long release.
            Adsr::new(0.8 * NOTE_LEN, 0.2 * NOTE_LEN, 0.8, 1.5 * NOTE_LEN),
        )
    };

    // Initializes a new `Polyphony` object, plays a single note.
    let mut poly = Polyphony::new();
    let mut idx = 0;
    poly.add(idx, osc(BASE));

    // The song loop.
    let poly_loop = Loop::new(
        // Every NOTE_LEN seconds,
        vec![NOTE_LEN],
        // we modify the signal `poly`,
        poly,
        FnWrapper::new(|poly: &mut Polyphony<_, _>, _| {
            // by stopping the note we just played,
            poly.stop(&idx);
            idx += 1;

            // and adding a new one.
            poly.add(
                idx,
                osc(Freq::new_edo_note(
                    BASE,
                    5,
                    rand::thread_rng().gen_range(0..=7) as f32,
                )),
            );
        }),
    );

    // This gives a really weird effect.
    let mut dist = PwMapSgn::cubic(poly_loop);

    pointillism::create("examples/poly_5edo.wav", 10.0 * NOTE_LEN, |_| dist.next()).unwrap();
}
