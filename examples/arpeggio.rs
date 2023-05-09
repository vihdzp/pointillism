//! A simple arpeggio demo.

use pointillism::prelude::*;

// Quarter notes at 160 BPM.
const NOTE_TIME: Time = Time::new(3.0 / 32.0);

// The length of each arpeggio "phrase".
const LENGTH: Time = Time::new(6.0);

fn main() {
    // The notes played in the arpeggio.
    let notes = vec![Freq::C4, Freq::E4, Freq::G4, Freq::A4];

    // Initializes the arpeggio.
    let mut arp = Arpeggio::new_arp(
        vec![NOTE_TIME],
        LoopGen::<Mono, _>::new(Tri, Freq::C0),
        notes,
    );

    // `C0` is a dummy value that gets replaced here.
    arp.skip_to_next();

    let mut timer = Timer::new(LENGTH);

    pointillism::create("examples/arpeggio.wav", 2.0 * LENGTH, |time| {
        // We switch up the arpeggio after the first phrase.
        if timer.tick(time) {
            arp.arp_mut().notes[2] = Freq::F4;
        }

        arp.next()
    })
    .unwrap();
}
