//! Source code for the [viiii – Primavera C](https://viiii.bandcamp.com/track/primavera-c) backing
//! track.
//!
//! Post-processing: ≈ 1kHz lowpass, field recording noise.

use pointillism::prelude::*;

// Base note for binaural beats.
const BASE: RawFreq = RawFreq::new(222.2);
// Fade-in / fade-out time for instruments.
const FADE: RawTime = RawTime::new(20.0);
// Period for the vibrato.
const VIB_FREQ: RawFreq = RawFreq::new(1.0 / 40.0);

// RawTime until the melody starts.
const MELODY_TIME: RawTime = RawTime::new(120.0);
// Length of the song.
const LENGTH: RawTime = RawTime::new(5.0 * 60.0);

/// A fade-in and fade-out effect.
///
/// TODO: implement this directly in pointillism.
fn fade(time: Time, length: Time, fade: Time) -> f64 {
    if time < fade {
        time.samples / fade.samples
    } else if time > length - fade {
        (length - time).samples / fade.samples
    } else {
        1.0
    }
}

/// Binaural beat generator.
fn binaural() -> impl SignalMut<Sample = Stereo> {
    let base = Freq::from_raw_default(BASE);
    let vib_freq = Freq::from_raw_default(VIB_FREQ);

    // A sine wave.
    let wave = |freq| LoopGen::new(Sin, freq);

    // Vibrato sine wave.
    let vib = |freq| {
        Vibrato::new(
            wave(freq),
            freq,
            PwMapSgn::new_pw(LoopGen::new(Sin, vib_freq), Linear::rescale_sgn(0.99, 1.01)),
        )
    };

    // Binaural beats.
    StereoMix::new(wave(base * 0.985), vib(base))
}

/// The melody that starts two minutes in.
fn melody() -> impl SignalMut<Sample = Mono> {
    // The melody is lowered by a chromatic semitone 24/25 every repetition.
    let mut freq = 2.0 * Freq::from_raw_default(RawFreq::A4);
    let intervals = [3.0 / 2.0, 4.0 / 5.0, 4.0 / 3.0, 3.0 / 5.0];

    // Our waves are saw-triangle morphs.
    let wave = |freq| LoopGen::new(SawTri::tri(), freq);

    // Their shape is abruptly turned from a saw into a triangle, which results in a rudimentary
    // "pluck" sound.
    let shape = move |freq| {
        MutSgn::new(
            wave(freq),
            OnceGen::new(PosSaw, Time::from_sec_default(5.0)),
            FnWrapper::new(|sgn: &mut LoopGen<_, SawTri>, val: Env| {
                sgn.curve_mut().shape = Val::new(1.0 - val.0.powf(0.2) / 2.0);
            }),
        )
    };

    // Make each note fade out.
    let trem = move |freq| {
        StopTremolo::new(
            shape(freq),
            OnceGen::new(PosInvSaw, Time::from_sec_default(10.0)),
        )
    };

    let poly = Polyphony::new();
    let mut index = 0;

    // Play a new note every four seconds.
    Loop::new(
        vec![Time::from_sec_default(4.0)],
        poly,
        FnWrapper::new(move |poly: &mut Polyphony<_, _>| {
            freq *= intervals[index % intervals.len()];
            poly.add(index, trem(freq));
            index += 1;
        }),
    )
}

fn main() {
    let mut binaural = binaural();
    let mut melody = melody();

    let length = Time::from_raw_default(LENGTH);
    let melody_time = Time::from_raw_default(MELODY_TIME);
    let fade_time = Time::from_raw_default(FADE);

    pointillism::create("examples/primavera_c.wav", length, SampleRate::CD, |time| {
        let mut sample = binaural.next() * fade(time, length, fade_time);

        // The triangle waves start playing 2 minutes in.
        if time > melody_time {
            sample += (melody.next() * fade(time - melody_time, length - melody_time, fade_time))
                .duplicate()
                / 10.0;
        }

        sample / 2.0
    })
    .unwrap();
}
