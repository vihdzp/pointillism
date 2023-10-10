//! We play various chords in succession, showing off somewhat more advanced use of the
//! [`Polyphony`] struct.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;

fn main() {
    /// Length of each note.
    const NOTE_LEN: unt::RawTime = unt::RawTime::new(7.0);
    let note_len = unt::Time::from_raw(NOTE_LEN, SAMPLE_RATE);

    let base = unt::Freq::from_raw(unt::RawFreq::A3, SAMPLE_RATE);
    let sgn = |freq| {
        AdsrEnvelope::new(
            LoopGen::<smp::Stereo, _>::new(Tri, freq),
            Adsr::new(
                unt::Time::from_sec(4.0, SAMPLE_RATE),
                unt::Time::from_sec(6.0, SAMPLE_RATE),
                unt::Vol::new(0.25),
                unt::Time::from_sec(3.0, SAMPLE_RATE),
            ),
        )
    };

    // Weird melody.
    #[rustfmt::skip]
    let chords = [
        [1.0, 6.0 / 5.0, 7.0 / 5.0],
        [5.0 / 6.0, 7.0 / 6.0, 3.0 / 2.0],
        [11.0 / 10.0, 7.0 / 5.0, 9.0 / 5.0],
        [1.0, 8.0 / 5.0, 6.0 / 5.0],
        [(4.0 / 3.0) * 5.0 / 6.0, (4.0 / 3.0) * 9.0 / 8.0, (4.0 / 3.0) * 3.0 / 2.0],
        [(4.0 / 3.0) * 11.0 / 10.0, (4.0 / 3.0) * 7.0 / 5.0, (4.0 / 3.0) * 9.0 / 5.0],
        [21.0 / 20.0, (7.0 / 5.0) * 21.0 / 20.0, 13.0 / 10.0],
    ];

    // Initialize the first chord.
    let mut poly = ply::Polyphony::new();
    for (i, &c) in chords[0].iter().enumerate() {
        poly.add(i, sgn(c * base));
    }

    let mut idx = 1;
    let mut seq = ctr::Sequence::new(
        vec![note_len; chords.len()],
        poly,
        map::Func::new(|poly: &mut ply::Polyphony<_, _>| {
            // Add next notes.
            if idx != chords.len() {
                for (i, &c) in chords[idx].iter().enumerate() {
                    poly.add(3 * idx + i, sgn(c * base));
                }
            }

            // Stop previous notes.
            for i in 0..3 {
                poly.stop(&(3 * (idx - 1) + i));
            }

            idx += 1;
        }),
    );

    pointillism::create(
        "examples/harmony.wav",
        note_len * chords.len() as f64 + unt::Time::from_sec(3.0, SAMPLE_RATE),
        SAMPLE_RATE,
        |_| seq.next() / 6.0,
    )
    .unwrap();
}
