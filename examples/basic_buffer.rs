//! We test saving a WAV file, loading it back, and time-stretching it.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

fn main() {
    const FILENAME: &str = "output/buffer.wav";

    // Creates some dummy wave file. In this case, a 440 Hz sine wave for 1s.
    pointillism::create_from_sgn(
        FILENAME,
        Time::from_raw_default(RawTime::SEC),
        SAMPLE_RATE,
        LoopGen::<Mono, Sin>::default(),
    )
    .unwrap();

    // Read back the file, stretch it to 5 seconds.
    //
    // This lowers the pitch, and may introduce some artifacts depending on the
    // interpolation method.
    const FACTOR: f64 = 5.0;
    let buf_sgn = OnceBufGen::new(Buffer::<Mono>::from_wav(FILENAME).unwrap());
    let time = buf_sgn.buffer().time();

    // We can change the interpolation method here.
    let sgn = Stretch::new_drop(buf_sgn, 1.0 / FACTOR);
    pointillism::create_from_sgn(FILENAME, time * FACTOR, SAMPLE_RATE, sgn).unwrap();
}
