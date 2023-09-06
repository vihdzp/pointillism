//! We use [`MelodyLoop`] to play a simple melody.

use pointillism::prelude::*;
use rand::Rng;

/// Project sample rate.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

fn main() {
    // A quarter note.
    let q = Time::from_sec(0.5, SAMPLE_RATE).floor();
    // The loop length.
    let length = 16u8 * q;
    // Release time for each note.
    let release = 2u8 * q;

    // Twinkle twinkle little star...
    let notes = [
        Note::new(Time::ZERO, q, RawFreq::C3),
        Note::new(q, q, RawFreq::C3),
        Note::new(2u8 * q, q, RawFreq::G3),
        Note::new(3u8 * q, q, RawFreq::G3),
        Note::new(4u8 * q, q, RawFreq::A3),
        Note::new(5u8 * q, q, RawFreq::A3),
        Note::new(6u8 * q, 2u8 * q, RawFreq::G3),
        Note::new(8u8 * q, q, RawFreq::F3),
        Note::new(9u8 * q, q, RawFreq::F3),
        Note::new(10u8 * q, q, RawFreq::E3),
        Note::new(11u8 * q, q, RawFreq::E3),
        Note::new(12u8 * q, q, RawFreq::D3),
        Note::new(13u8 * q, q, RawFreq::D3),
        Note::new(14u8 * q, 2u8 * q, RawFreq::C3),
        Note::new(14u8 * q, 2u8 * q, RawFreq::G3),
    ]
    .map(|note| note.map_data(|raw| Freq::from_raw(raw, SAMPLE_RATE)))
    .to_vec();

    // We randomly bend each note a little, just for fun.
    const BEND: f64 = 3.0;
    let rand_bend = || rand::thread_rng().gen::<f64>() / BEND - 0.5 / BEND;

    // Each note is a triangle wave, with a simple ADSR envelope, playing the corresponding note.
    let func = |freq: Freq| {
        AdsrEnvelope::new_adsr(
            LoopGen::<Mono, _>::new(Tri, freq.bend(rand_bend())),
            Time::from_sec(0.1, SAMPLE_RATE),
            Time::from_sec(0.2, SAMPLE_RATE),
            Vol::HALF,
            release,
        )
    };

    let mut melody = MelodyLoop::piano_roll(notes, length, FnWrapper::new(func), |idx| idx as u8);
    let mut timer = Timer::new(2u8 * length);

    // We play the melody twice.
    pointillism::create(
        "examples/simple_melody.wav",
        2u8 * length + release,
        SAMPLE_RATE,
        |time| {
            // After the melody has been played twice, stop all voices.
            if timer.tick(time) {
                melody.sgn_mut().stop_all();
            }

            0.5 * if time < 2u8 * length {
                // Play as usual.
                melody.next()
            } else {
                // Stop the loop, just play the inner fading signal instead.
                melody.sgn_mut().next()
            }
        },
    )
    .unwrap();
}
