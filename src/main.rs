mod basic;
pub mod effects;
pub mod generators;

pub use basic::*;

use effects::{
    adsr::{Adsr, AdsrEnvelope},
    events::{Event, Loop},
};
use generators::{
    curve::{CurveGen, SawTri},
    poly::Polyphony,
};
use sample::AudioSample;
use signal::{Signal, StopSignal};

use hound::*;
use rand::Rng;

/// The sample rate for the audio file, in samples per second.
const SAMPLE_RATE: u32 = 44100;

/// The number of channels in the output file. Only 1 and 2 are supported.
const CHANNELS: u8 = 2;

/// The specification for the output file.
const SPEC: WavSpec = WavSpec {
    channels: CHANNELS as u16,
    sample_rate: SAMPLE_RATE,
    bits_per_sample: 32,
    sample_format: SampleFormat::Float,
};

/// Pitch for the base note A4.
const A4: Freq = Freq::new(440.0);

fn main() {
    // Main variables.
    let mut global = Time::zero();
    let mut writer = WavWriter::create("melody.wav", SPEC).unwrap();
    let beat = Time::new_beat(120.0);

    // Oscillators.
    let pad = |note| {
        AdsrEnvelope::new(
            CurveGen::new(SawTri::new(0.75), Freq::new_edo(A4, 5, note)),
            Adsr::new(Time::new(3.0), Time::new(2.0), 0.8, Time::new(10.0)),
        )
    };
    let mut poly = Polyphony::new();
    poly.add(pad(0));

    // Melody config.
    let times = vec![8.0 * beat];

    // Initialize sequence.
    let mut seq = Loop::new(
        times,
        poly,
        FnWrapper(|sgn: &mut Polyphony<_>, event: Event| {
            sgn.stop(event.idx);
            sgn.add(pad(rand::thread_rng().gen_range(0..=7)));
        }),
    );

    // Save to audio file.
    while global < 128.0 * beat {
        seq.next().duplicate().write(&mut writer, 2).unwrap();
        global.advance();
    }

    writer.finalize().unwrap();
}
