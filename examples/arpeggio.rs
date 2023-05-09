//! A simple arpeggio demo.

use pointillism::prelude::*;

// Quarter notes at 160 BPM.
const NOTE_TIME: Time = Time::new(3.0 / 32.0);

// The length of each arpeggio "phrase".
const LENGTH: Time = Time::new(6.0);

fn main() {
    // The times for which the arpeggio notes play.
    let times = vec![NOTE_TIME; 4];

    // The notes played in the arpeggio.
    let notes = vec![Freq::C4, Freq::E4, Freq::G4, Freq::A4];

    // Initializes the arpeggio.
    let mut arp = Arpeggio::new_arp(times, LoopGen::<Mono, _>::new(Saw, Freq::C0), notes);

    // `C0` is a dummy value that gets replaced here.
    arp.skip_to_next();

    // We switch up the arpeggio after the first phrase.
    let seq = Sequence::new(
        vec![LENGTH],
        arp,
        FnWrapper::new(|sgn: &mut Arpeggio<_>| {
            sgn.arp_mut().notes[2] = Freq::F4;
        }),
    );

    pointillism::create_from_sgn("examples/arpeggio.wav", 2.0 * LENGTH, seq).unwrap();
}
