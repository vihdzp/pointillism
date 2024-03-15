//! A cover of Claude Debussy's
//! [https://en.wikipedia.org/wiki/Suite_bergamasque#3._Clair_de_lune](Clair de Lune).
//!
//! We load a [MIDI file](https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1778), give it our
//! own instrumentation, and export it.

#[cfg(feature = "midly")]
fn main() {
    use pointillism::prelude::*;

    // Play at a normal speed.
    let tick_time = unt::Time::from_sec_default(2.2e-3);

    // Soft triangle waves with a piano-like envelope.
    let release = unt::Time::from_sec_default(0.3);
    let func = |data: ctr::MidiNoteData| {
        // Add a bit of octave compression just for fun.
        let mut raw = unt::RawFreq::new_midi(data.key.into()).bend(0.5);
        raw.hz = raw.hz.powf(0.995);

        eff::env::AdsrEnv::new_adsr(
            gen::Loop::<smp::Stereo, _>::new(
                crv::Morph::half(crv::Sin, crv::Tri),
                unt::Freq::from_raw_default(raw),
            ),
            eff::env::Adsr::new(
                unt::Time::from_sec_default(0.05),
                unt::Time::from_sec_default(2.5),
                unt::Vol::ZERO,
                release,
            ),
        )
    };

    let mut tracks = midly::parse(include_bytes!("clair_de_lune.mid"))
        .expect("could not parse MIDI")
        .1;

    // The first track is empty.
    tracks.next();
    let main_track = tracks.next().unwrap().unwrap();
    let bass_track = tracks.next().unwrap().unwrap();

    // There will not be more than 256 notes playing at once.
    let idx_cast = |idx: usize| idx as u8;

    let mut main_melody = ctr::MelSeq::new_melody(
        ctr::Melody::from_midi(main_track, tick_time, idx_cast).unwrap(),
        map::Func::new(func),
    );
    let mut bass_melody = ctr::MelSeq::new_melody(
        ctr::Melody::from_midi(bass_track, tick_time, idx_cast).unwrap(),
        map::Func::new(func),
    );

    // Length of the file.
    let length = main_melody.total_time() + release;

    Song::new(length, unt::SampleRate::default(), |_| {
        (main_melody.next() + bass_melody.next() * 0.8) * 0.15
    })
    .export("pointillism/examples/clair_de_lune.wav");
}

#[cfg(not(feature = "midly"))]
fn main() {
    println!("This example must be run with the midly feature.")
}
