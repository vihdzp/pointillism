//! A simple arpeggio demo.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: SampleRate = SampleRate::CD;
/// Quarter notes at 160 BPM.
const NOTE_TIME: RawTime = RawTime::new(3.0 / 32.0);
/// The length of each arpeggio "phrase".
const LENGTH: RawTime = RawTime::new(3.0);

fn main() {
    let note_time = Time::from_raw(NOTE_TIME, SAMPLE_RATE);
    let length = Time::from_raw(LENGTH, SAMPLE_RATE);

    // The notes played in the arpeggio.
    let notes = [RawFreq::C4, RawFreq::E4, RawFreq::G4, RawFreq::A4]
        .map(|raw| Freq::from_raw(raw, SAMPLE_RATE))
        .to_vec();

    // Initializes the arpeggio.
    let mut arp = Arpeggio::new_arp(
        vec![note_time],
        LoopGen::<Mono, _>::new(Tri, Freq::ZERO),
        notes,
    );

    // Zero is a dummy value that gets replaced here.
    arp.skip();

    let mut timer = Timer::new(length);

    pointillism::create("examples/arpeggio.wav", 2u8 * length, SAMPLE_RATE, |time| {
        // We switch up the arpeggio after the first phrase.
        if timer.tick(time) {
            arp.arp_mut().notes[2] = Freq::from_raw(RawFreq::F4, SAMPLE_RATE);
        }

        arp.next()
    })
    .unwrap();
}
