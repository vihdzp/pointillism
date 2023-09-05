//! A simple arpeggio demo.

use pointillism::prelude::*;

/// Quarter notes at 160 BPM.
const NOTE_TIME: RawTime = RawTime::new(3.0 / 32.0);
/// The length of each arpeggio "phrase".
const LENGTH: RawTime = RawTime::new(6.0);

/// Sample rate of song.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

fn main() {
    let note_time = Time::from_raw_default(NOTE_TIME);
    let length = Time::from_raw_default(LENGTH);

    // The notes played in the arpeggio.
    let notes: Vec<_> = [RawFreq::C4, RawFreq::E4, RawFreq::G4, RawFreq::A4]
        .map(Freq::from_raw_default)
        .into();

    // Initializes the arpeggio.
    let mut arp = Arpeggio::new_arp(
        vec![note_time],
        LoopGen::<Mono, _>::new(Tri, Freq::from_raw_default(RawFreq::C0)),
        notes,
    );

    // `C0` is a dummy value that gets replaced here.
    arp.skip_to_next();

    let mut timer = Timer::new(Time::from_raw_default(LENGTH));

    pointillism::create("examples/arpeggio.wav", 2.0 * length, |time| {
        // We switch up the arpeggio after the first phrase.
        if timer.tick(time) {
            arp.arp_mut().notes[2] = Freq::from_raw_default(RawFreq::F4);
        }

        arp.next()
    })
    .unwrap();
}
