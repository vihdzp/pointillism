//! Source code for
//! [viiii – Primavera C](https://viiii.bandcamp.com/track/primavera-c).
//!
//! Post-processing: ≈ 1kHz lowpass, field recording noise.

// Todo: comment and clean up code.

use pointillism::prelude::*;

// Base note for binaural beats.
const BASE: Freq = Freq::new(222.2);

// Fade-in / fade-out time for instruments.
const FADE: Time = Time::new(20.0);

// Period for the vibrato.
const VIB_FREQ: Freq = Freq::new(1.0 / 40.0);

// Time until the melody starts.
const MELODY_TIME: Time = Time::new(120.0);

// Length of the song.
const LENGTH: Time = Time::new(5.0 * 60.0);

/// A fade-in and fade-out effect.
///
/// TODO: implement this directly in pointillism.
fn fade(time: Time, length: Time, fade: Time) -> f32 {
    if time < fade {
        time.seconds / fade.seconds
    } else if time > length - fade {
        (length - time).seconds / fade.seconds
    } else {
        1.0
    }
}

fn binaural() -> impl Signal<Sample = Stereo> {
    // A sine wave.
    let wave = |freq| LoopGen::new(Sin::sin(), freq);

    // Vibrato sine wave.
    let vib = |freq| {
        Vibrato::new(
            wave(freq),
            freq,
            PwMapSgn::new_pw(
                LoopGen::new(Sin::sin(), VIB_FREQ),
                Linear::rescale(-1.0, 1.0, 0.99, 1.01),
            ),
        )
    };

    // Binaural beats.
    StereoMix::new(wave(BASE * 0.985), vib(BASE))
}

fn melody() -> impl Signal<Sample = Mono> {
    // The melody is lowered by a chromatic semitone 24/25 every repetition.
    let mut freq = 2.0 * Freq::A4;
    let notes = [3.0 / 2.0, 4.0 / 5.0, 4.0 / 3.0, 3.0 / 5.0];

    let wave = |freq| LoopGen::new(SawTri::tri(), freq);
    let shape = move |freq| {
        MutSgn::new(
            wave(freq),
            OneshotGen::new(PosSaw::new(), Time::new(5.0)),
            FnWrapper::new(|sgn: &mut LoopGen<_, SawTri>, val: f32| {
                sgn.curve_mut().shape = 1.0 - val.powf(0.2) / 2.0;
            }),
        )
    };
    let trem = move |freq| {
        StopTremolo::new(
            shape(freq),
            OneshotGen::new(PosInvSaw::new(), Time::new(10.0)),
            Vol::FULL,
        )
    };

    let poly = Polyphony::new();
    let mut idx = 0;

    Loop::new(
        vec![Time::new(4.0)],
        poly,
        FnWrapper::new(move |poly: &mut Polyphony<_, _>, _| {
            freq *= notes[idx % notes.len()];
            poly.add(idx, trem(freq));
            idx += 1;
        }),
    )
}

fn main() {
    let mut binaural = binaural();
    let mut melody = melody();

    pointillism::create("examples/primavera_c.wav", LENGTH, |time| {
        let mut sample = binaural.next() * fade(time, LENGTH, FADE);

        if time > MELODY_TIME {
            sample += (melody.next() * fade(time - MELODY_TIME, LENGTH - MELODY_TIME, FADE))
                .duplicate()
                / 10.0;
        }

        sample / 2.0
    })
    .unwrap();
}
