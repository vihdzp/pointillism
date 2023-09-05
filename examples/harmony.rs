//! Creepy chords.

use pointillism::prelude::*;

fn main() {
    let base = Freq::from_raw_default(RawFreq::A3);
    let sgn = |freq| {
        AdsrEnvelope::new(
            LoopGen::<Stereo, _>::new(Tri, freq),
            Adsr::new(
                Time::from_sec_default(4.0),
                Time::from_sec_default(6.0),
                Vol::new(0.25),
                Time::from_sec_default(3.0),
            ),
        )
    };

    let chords = [
        [1.0, 6.0 / 5.0, 7.0 / 5.0],
        [5.0 / 6.0, 7.0 / 6.0, 3.0 / 2.0],
        [11.0 / 10.0, 7.0 / 5.0, 9.0 / 5.0],
        [1.0, 8.0 / 5.0, 6.0 / 5.0],
        [
            (4.0 / 3.0) * 5.0 / 6.0,
            (4.0 / 3.0) * 9.0 / 8.0,
            (4.0 / 3.0) * 3.0 / 2.0,
        ],
        [
            (4.0 / 3.0) * 11.0 / 10.0,
            (4.0 / 3.0) * 7.0 / 5.0,
            (4.0 / 3.0) * 9.0 / 5.0,
        ],
        [21.0 / 20.0, (7.0 / 5.0) * 21.0 / 20.0, 13.0 / 10.0],
    ];

    let poly = Polyphony::new();

    let mut idx = 0;
    let mut seq = Sequence::new(
        vec![Time::from_sec_default(10.0); chords.len() + 1],
        poly,
        FnWrapper::new(|poly: &mut Polyphony<_, _>| {
            if idx != chords.len() {
                for (i, c) in chords[idx].iter().enumerate() {
                    poly.add(3 * idx + i, sgn(base * *c));
                }
            }

            if idx != 0 {
                for i in 0..3 {
                    poly.stop(&(3 * (idx - 1) + i));
                }
            }

            idx += 1;
        }),
    );
    seq.skip_to_next();

    pointillism::create(
        "examples/harmony.wav",
        Time::from_sec_default(10.0) * chords.len() as f64 + Time::from_sec_default(3.0),
        |_| seq.next() / 6.0,
    )
    .unwrap();
}
