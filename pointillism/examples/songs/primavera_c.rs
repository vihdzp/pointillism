//! Source code for the [viiii – Primavera C](https://viiii.bandcamp.com/track/primavera-c) backing
//! track.
//!
//! Post-processing: ≈ 1kHz lowpass, field recording noise.

use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;

/// A fade-in and fade-out effect.
///
/// TODO: implement this directly in pointillism.
fn fade(time: unt::Time, length: unt::Time, fade: unt::Time) -> f64 {
    if time < fade {
        time.samples / fade.samples
    } else if time > length - fade {
        (length - time).samples / fade.samples
    } else {
        1.0
    }
}

/// Binaural beat generator.
fn binaural() -> impl SignalMut<Sample = smp::Stereo> {
    let base = unt::Freq::from_hz(222.2, SAMPLE_RATE);
    let vib_freq = unt::Freq::from_hz(1.0 / 40.0, SAMPLE_RATE);

    // A sine wave.
    let wave = |freq| gen::Loop::new(crv::Sin, freq);

    // Vibrato sine wave.
    let vib = |freq| {
        eff::Vibrato::new(
            wave(freq),
            freq,
            eff::PwMapSgn::new_pw(
                gen::Loop::new(crv::Sin, vib_freq),
                map::Linear::rescale_sgn(0.99, 1.01),
            ),
        )
    };

    // Binaural beats.
    rtn::Stereo::new(wave(base * 0.985), vib(base))
}

/// The melody that starts two minutes in.
fn melody() -> impl SignalMut<Sample = smp::Mono> {
    // The melody is lowered by a chromatic semitone 24/25 every repetition.
    const INTERVALS: [f64; 4] = [3.0 / 2.0, 4.0 / 5.0, 4.0 / 3.0, 3.0 / 5.0];
    let mut freq = 2.0 * unt::Freq::from_raw(unt::RawFreq::A4, SAMPLE_RATE);
    let sec = unt::Time::from_sec(1.0, SAMPLE_RATE);

    // Our waves are saw-triangle morphs.
    let wave = |freq| gen::Loop::new(crv::SawTri::tri(), freq);

    // Their shape is abruptly turned from a saw into a triangle, which results in a rudimentary
    // "pluck" sound.
    let shape = move |freq| {
        eff::MutSgn::new(
            wave(freq),
            gen::Once::new(crv::PosSaw, sec * 5u32),
            map::Func::new(|sgn: &mut gen::Loop<_, crv::SawTri>, val: smp::Env| {
                sgn.curve_mut().shape = unt::Val::new(1.0 - val.0.powf(0.2) / 2.0);
            }),
        )
    };

    // Make each note fade out.
    let trem =
        move |freq| eff::StopTremolo::new(shape(freq), gen::Once::new(crv::PosInvSaw, sec * 10u32));

    // Play a new note every four seconds.
    let mut index = 0;
    ctr::Loop::new(
        vec![unt::Time::from_sec_default(4.0)],
        gen::Polyphony::new(),
        map::Func::new(move |poly: &mut gen::Polyphony<_, _>| {
            freq *= INTERVALS[index % INTERVALS.len()];
            poly.add(index, trem(freq));
            index += 1;
        }),
    )
}

fn main() {
    let mut binaural = binaural();
    let mut melody = melody();

    let length = unt::Time::from_min(5.0, SAMPLE_RATE);
    let melody_time = unt::Time::from_sec(120.0, SAMPLE_RATE);
    let fade_time = unt::Time::from_sec(20.0, SAMPLE_RATE);

    Song::new_func(length, SAMPLE_RATE, |time| {
        let mut sample = binaural.next() * fade(time, length, fade_time);

        // The triangle waves start playing 2 minutes in.
        if time > melody_time {
            sample += (melody.next() * fade(time - melody_time, length - melody_time, fade_time))
                .duplicate()
                / 10.0;
        }

        sample / 2.0
    })
    .export("pointillism/examples/primavera_c.wav");
}
