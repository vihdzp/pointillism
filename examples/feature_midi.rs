//! Loads and plays a MIDI file.

#[cfg(feature = "midly")]
fn main() {
    use pointillism::prelude::*;

    // Play at a normal speed.
    let tick_time = Time::from_sec_default(2.2e-3);

    // Soft triangle waves with a piano-like envelope.
    let main_release = Time::from_sec_default(0.3);
    let main_func = |data: MidiNoteData| {
        // Add a bit of octave stretching just for fun.
        let mut raw = RawFreq::new_midi((data.key - 1.into()).into());
        raw.hz = raw.hz.powf(1.01);

        AdsrEnvelope::new_adsr(
            LoopGen::<Stereo, _>::new(Tri, Freq::from_raw_default(raw)),
            Time::from_sec_default(0.05),
            Time::from_sec_default(2.5),
            Vol::ZERO,
            main_release,
        )
    };

    // A nice sine bass.
    let bass_release = Time::from_sec_default(1.0);
    let bass_func = |data: MidiNoteData| {
        // Add a bit of octave stretching just for fun.
        let mut raw = RawFreq::new_midi((data.key - 1.into()).into());
        raw.hz = raw.hz.powf(1.01);

        AdsrEnvelope::new_adsr(
            LoopGen::<Stereo, _>::new(Sin, Freq::from_raw_default(raw)),
            Time::from_sec_default(0.2),
            Time::from_sec_default(2.0),
            Vol::HALF,
            bass_release,
        )
    };

    // https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1778
    let mut tracks = midly::parse(include_bytes!("clair_de_lune.mid")).unwrap().1;

    // The first track is empty.
    tracks.next();

    let main_track = tracks.next().unwrap().unwrap();
    let bass_track = tracks.next().unwrap().unwrap();

    // There will not be more than 256 notes playing at once.
    let idx_cast = |idx: usize| idx as u8;

    let mut main_melody =
        MelodySeq::from_midi(main_track, tick_time, FnWrapper::new(main_func), idx_cast).unwrap();
    let mut bass_melody =
        MelodySeq::from_midi(bass_track, tick_time, FnWrapper::new(bass_func), idx_cast).unwrap();

    // Length of the file.
    let length =
        main_melody.total_time().max(bass_melody.total_time()) + main_release.max(bass_release);

    pointillism::create(
        "output/clair_de_lune.wav",
        length,
        SampleRate::default(),
        |_| 0.15 * (main_melody.next() + 0.8 * bass_melody.next()),
    )
    .unwrap();
}

#[cfg(not(feature = "midly"))]
fn main() {
    println!("This example must be run with the midly feature.")
}
