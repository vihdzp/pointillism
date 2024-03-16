use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;

/// Length of a beat.
const BEAT: unt::RawTime = unt::RawTime::new_sec(1.0 / 2.0);
/// Delay time.
const DELAY: unt::RawTime = unt::RawTime::new_sec(BEAT.seconds * 2.0 / 3.0);
/// Time for the song end (not counting the fade-out).
const END: unt::RawTime = unt::RawTime::new_sec(45.0);

/// The sequence of intervals.
const INTERVALS: [f64; 6] = [
    9.0 / 8.0,
    3.0 / 2.0,
    3.0 / 5.0,
    3.0 / 4.0,
    7.0 / 4.0,
    3.0 / 4.0,
];

/// The product of all intervals.
const COMMA: f64 = {
    let mut i = 0;
    let mut prod = 1.0;
    while i < 6 {
        prod *= INTERVALS[i];
        i += 1;
    }
    prod
};

/// Pluck arpeggio.
fn pluck() -> impl SignalMut<Sample = smp::Mono> {
    let beat = unt::Time::from_raw(BEAT, SAMPLE_RATE);

    // Soft sine wave.
    let sgn = eff::env::ArEnv::new_ar(
        gen::Loop::<smp::Mono, _>::new(
            crv::Morph::new(crv::Sin, crv::Tri, unt::Val::new(0.1)),
            unt::Freq::from_raw(unt::RawFreq::G3, SAMPLE_RATE),
        ),
        eff::env::Ar::new(beat * 0.1, beat * 0.8),
    );

    let mut note = 0;
    ctr::Loop::new(
        vec![beat],
        sgn,
        map::Func::new(move |sgn: &mut eff::env::ArEnv<_>| {
            *sgn.freq_mut() *= INTERVALS[note % INTERVALS.len()];
            if beat * (note as u8) < unt::Time::from_raw(END, SAMPLE_RATE) {
                sgn.retrigger();
            }
            note += 1;
        }),
    )
}

/// Background saw.
fn saw() -> impl SignalMut<Sample = smp::Mono> {
    // Soft sine wave.
    let sgn = eff::env::ArEnv::new_ar(
        gen::Loop::<smp::Mono, _>::new(
            crv::SawTri::new(unt::Val::new(0.6)),
            unt::Freq::from_raw(unt::RawFreq::G3, SAMPLE_RATE),
        ),
        eff::env::Ar::new(beat * 0.1, beat * 0.8),
    );

    let mut note = 0;
    ctr::Loop::new(
        vec![beat],
        sgn,
        map::Func::new(move |sgn: &mut eff::env::ArEnv<_>| {
            *sgn.freq_mut() *= INTERVALS[note % INTERVALS.len()];
            if beat * (note as u8) < unt::Time::from_raw(END, SAMPLE_RATE) {
                sgn.retrigger();
            }
            note += 1;
        }),
    )
}

fn main() {
    let sample_rate = unt::SampleRate::default();
    let sec = unt::Time::from_sec(1.0, sample_rate);
    let length = sec * 50u32;

    let pluck = pluck();
    let mix = pluck;

    let delay = unt::Time::from_raw(DELAY, SAMPLE_RATE);
    let mut delay_sgn = eff::dly::Exp::new_exp_owned(mix, delay, unt::Vol::ZERO);

    Song::new(length, sample_rate, |time| {
        delay_sgn.vol_mut().gain = 0.75 * time / length + 0.1;
        delay_sgn.next() / 3.0
    })
    .export("pointillism/examples/boron.wav");
}
