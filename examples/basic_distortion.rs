//! Testing out distortion.
//!
//! We play various random 5-EDO notes in succession, and apply heavy distortion to the output for
//! fun effects.

use pointillism::prelude::*;
use rand::Rng;

/// Project sample rate.
///
/// Lower sample rates make for an even scrunchier sound.
const SAMPLE_RATE: SampleRate = SampleRate::TELEPHONE;

fn main() {
    // Base frequency.
    const BASE: RawFreq = RawFreq::new(350.0);
    // Length of each note.
    const NOTE_LEN: RawTime = RawTime::new(5.0);

    let base = Freq::from_raw(BASE, SAMPLE_RATE);
    let note_len = Time::from_raw(NOTE_LEN, SAMPLE_RATE);

    // Each oscillator is a function of frequency.
    let osc = |freq| {
        AdsrEnvelope::new(
            // Sine wave with specified frequency.
            LoopGen::<Stereo, _>::new(Sin, freq),
            // ADSR envelope with long attack, long release.
            Adsr::new(
                0.8 * note_len,
                0.2 * note_len,
                Vol::new(0.8),
                1.5 * note_len,
            ),
        )
    };

    // Initializes a new `Polyphony` object, plays a single note.
    let mut poly = Polyphony::new();
    let mut index = 0;
    poly.add(index, osc(base));

    // The song loop.
    let poly_loop = Loop::new(
        // Every NOTE_LEN seconds,
        vec![note_len],
        // we modify the signal `poly`,
        poly,
        FnWrapper::new(|poly: &mut Polyphony<_, _>| {
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
    let mut dist = PwMapSgn::cubic(poly_loop);
    pointillism::create_from_sgn(
        "examples/distortion.wav",
        10u8 * note_len,
        SAMPLE_RATE,
        &mut dist,
    )
    .unwrap();
}
