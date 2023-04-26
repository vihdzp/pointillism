//! Tests loading a WAV file.

use pointillism::prelude::*;

fn main() {
    const FILENAME: &str = "examples/buffer.wav";

    // Creates some dummy wave file. In this case, a 440 Hz sine wave.
    pointillism::create_from_sgn(FILENAME, Time::SEC, LoopCurveGen::<Mono, Sin>::default())
        .unwrap();

    // Read back the file, stretch it to 10 seconds.
    //
    // This lowers the pitch, and may introduce some artifacts depending on the
    // interpolation method.
    const FACTOR: f64 = 0.1;
    let buf_curve = BufCurve::<Mono>::from_wav(FILENAME, Interpolate::Drop).unwrap();
    let time = buf_curve.buffer().time();
    pointillism::create_from_sgn(
        FILENAME,
        time * FACTOR,
        OneshotGen::new(buf_curve, time * FACTOR),
    )
    .unwrap();
}
