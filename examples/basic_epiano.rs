//! Creates a basic electric piano synth, plays a major chord.
//!
//! Adapted from
//! <https://docs.rs/twang/latest/twang/index.html#a3-220-hz-minor-piano-example>.

use pointillism::prelude::*;
use rand::Rng;

/// Project sample rate.
const SAMPLE_RATE: SampleRate = SampleRate::CD;

/// First ten harmonic volumes of a piano sample.
const HARMONICS: [f64; 10] = [
    0.700, 0.243, 0.229, 0.095, 0.139, 0.087, 0.288, 0.199, 0.124, 0.090,
];

/// A custom electric piano signal, created from a bunch of sine wave
/// oscillators.
#[derive(Default)]
pub struct EPiano {
    /// Oscillators at the fundamental frequency, twice that, etc.
    oscs: [LoopGen<Mono, Sin>; 10],
}

impl EPiano {
    /// Randomize the phases of the oscillators.
    fn rand_phases(&mut self) {
        for osc in &mut self.oscs {
            *osc.val_mut() = rand::thread_rng().gen();
        }
    }

    /// Create a new electric piano with the given frequency.
    fn new(freq: Freq) -> Self {
        let mut epiano = Self::default();
        epiano.rand_phases();

        for (i, osc) in epiano.oscs.iter_mut().enumerate() {
            *osc.freq_mut() = freq * (i + 1) as f64;
        }

        epiano
    }
}

impl Signal for EPiano {
    type Sample = Mono;

    /// Adds up the values from the inner oscillators, rescaled by the harmonic
    /// volumes.
    fn get(&self) -> Mono {
        self.oscs
            .iter()
            .enumerate()
            .map(|(i, osc)| osc.get() * HARMONICS[i])
            .sum::<Mono>()
            / 10.0
    }
}

impl SignalMut for EPiano {
    /// Advance the inner oscillators.
    fn advance(&mut self) {
        for osc in &mut self.oscs {
            osc.advance();
        }
    }

    /// Retrigger the inner oscillators, randomize phases.
    fn retrigger(&mut self) {
        self.rand_phases();
        for osc in &mut self.oscs {
            osc.retrigger();
        }
    }
}

/// An electric piano with a slight tremolo effect applied.
fn trem_piano(freq: Freq, vib_freq: Freq) -> impl Stop<Sample = Mono> {
    // The volume follows a rescaled sine wave curve.
    let env = LoopGen::new(Comp::new(Sin, Linear::rescale_sgn(0.8, 1.0)), vib_freq);

    // Some subtle ADSR.
    let adsr = Adsr::new(
        Time::from_sec(0.1, SAMPLE_RATE),
        Time::from_sec(0.2, SAMPLE_RATE),
        Vol::new(0.7),
        Time::from_sec(0.1, SAMPLE_RATE),
    );

    AdsrEnvelope::new(Tremolo::new(EPiano::new(freq), env), adsr)
}

fn main() {
    let c3 = Freq::from_raw(RawFreq::C3, SAMPLE_RATE);
    let sec = Time::from_raw(RawTime::SEC, SAMPLE_RATE);

    // One piano for each note.
    let (mut p1, mut p2, mut p3) = (
        trem_piano(c3, Freq::from_hz_default(4.0)),
        trem_piano(5.0 / 4.0 * c3, Freq::from_hz_default(5.0)),
        trem_piano(3.0 / 2.0 * c3, Freq::from_hz_default(6.0)),
    );

    let mut timer = Timer::new(5.0 * sec);
    pointillism::create("output/epiano.wav", 5.2 * sec, SAMPLE_RATE, |time| {
        let mut sgn = p1.next();

        // Play the second note after one second.
        if time > Time::from_raw_default(RawTime::SEC) {
            sgn += p2.next();
        }

        // Play the third note after two seconds.
        if time > 2u8 * sec {
            sgn += p3.next();
        }

        // Stops all notes after five seconds.
        if timer.tick(time) {
            p1.stop();
            p2.stop();
            p3.stop();
        }

        sgn / 3.0
    })
    .unwrap();
}
