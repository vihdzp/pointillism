use pointillism::prelude::*;

fn main() {
    let base = Freq::A3;
    let sgn = |freq| {
        AdsrEnvelope::new(
            LoopGen::<Stereo, _>::new(Tri, freq),
            Adsr::new(
                Time::new(4.0),
                Time::new(6.0),
                Vol::new(0.25),
                Time::new(3.0),
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
        vec![Time::new(10.0); chords.len() + 1],
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
        Time::new(10.0) * chords.len() as f64 + Time::new(3.0),
        |_| seq.next() / 6.0,
    )
    .unwrap();
}
