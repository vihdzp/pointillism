//! A simple arpeggio demo.

use pointillism::prelude::*;

/// Quarter notes at 160 BPM.
const NOTE_TIME: Time = Time::new(3.0 / 32.0);
/// The length of each arpeggio "phrase".
const LENGTH: Time = Time::new(6.0);

/// Sample rate of song.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

fn main() {
    // The notes played in the arpeggio.
    let notes: Vec<_> = [RawFreq::C4, RawFreq::E4, RawFreq::G4, RawFreq::A4]
        .map(Freq::from_raw_default)
        .into();

    // Initializes the arpeggio.
    let mut arp = Arpeggio::new_arp(
        vec![NOTE_TIME],
        LoopGen::<Mono, _>::new(Tri, Freq::from_raw_default(RawFreq::C0)),
        notes,
    );

    // `C0` is a dummy value that gets replaced here.
    arp.skip_to_next();

    let mut timer = Timer::new(LENGTH);

    pointillism::create("examples/arpeggio.wav", 2.0 * LENGTH, |time| {
        // We switch up the arpeggio after the first phrase.
        if timer.tick(time) {
            arp.arp_mut().notes[2] = Freq::from_raw_default(RawFreq::F4);
        }

        arp.next()
    })
    .unwrap();
}
