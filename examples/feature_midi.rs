#[cfg(feature = "midly")]
fn main() {
    use pointillism::prelude::*;

    // These are just best guesses.
    let length = 250_000u32;
    let tick_time = Time::from_sec_default(2.2e-3);

    // Soft triangle waves with a piano-like envelope.
    let func = |data: MidiNoteData| {
        // Add a bit of octave stretching just for fun.
        let mut raw = RawFreq::new_midi((data.key - 1.into()).into());
        raw.hz = raw.hz.powf(1.01);

        AdsrEnvelope::new_adsr(
            LoopGen::<Stereo, _>::new(Tri, Freq::from_raw_default(raw)),
            Time::from_sec_default(0.05),
            Time::from_sec_default(3.0),
            Vol::ZERO,
            Time::from_sec_default(0.5),
        )
    };

    // https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1778
    let smf = midly::parse(include_bytes!("clair_de_lune.mid")).unwrap();
    let melody =
        MelodySeq::from_midi(smf.1, tick_time, FnWrapper::new(func), |x| x as u16).unwrap();

    pointillism::create_from_sgn(
        "output/test.wav",
        length * tick_time,
        SampleRate::default(),
        Volume::new(melody, Vol::new(0.2)),
    )
    .unwrap();
}

#[cfg(not(feature = "midly"))]
fn main() {
    println!("This example must be run with the midly feature.")
}
