//! The most basic example: we play a single sine wave.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

fn main() {
    // File duration.
    let length = Time::from_sec(1.0, SAMPLE_RATE);
    // Sine wave frequency.
    let freq = Freq::from_hz(440.0, SAMPLE_RATE);

    // We create a mono signal that loops through a sine curve at the specified frequency.
    let mut sgn = LoopGen::<Mono, _>::new(Sin, freq);

    // Export to file.
    pointillism::create_from_sgn("examples/sine.wav", length, SAMPLE_RATE, &mut sgn).unwrap();
}
