use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;

/// Length of a beat.
const BEAT: unt::RawTime = unt::RawTime::new_sec(1.0 / 2.0);
/// Delay time.
const DELAY: unt::RawTime = unt::RawTime::new_sec(BEAT.seconds * 2.0 / 3.0);

/// Time for the song end (not counting the fade-out).
const LENGTH: unt::RawTime = unt::RawTime::new_sec(45.0);
/// Fade-out length.
const FADE: unt::RawTime = unt::RawTime::new_sec(5.0);

/// The base frequency for the song.
const BASE_FREQ: unt::RawFreq = unt::RawFreq::G3;

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

/// One beat.
fn beat() -> unt::Time {
    unt::Time::from_raw(BEAT, SAMPLE_RATE)
}

/// Pluck arpeggio.
fn pluck() -> impl SignalMut<Sample = smp::Mono> {
    let beat = beat();

    // Soft sine wave.
    let sgn = move |freq| {
        eff::env::ArEnv::new_ar(
            gen::Loop::<smp::Mono, _>::new(
                crv::Morph::new(crv::Sin, crv::Tri, unt::Val::new(0.2)),
                freq,
            ),
            eff::env::Ar::new(beat * 0.05, beat * 1.2),
        )
    };

    // Play in a loop.
    let mut freq = unt::Freq::from_raw(BASE_FREQ, SAMPLE_RATE);
    let mut poly = poly::Polyphony::new();
    poly.add(u8::MAX, sgn(freq));
    let mut note = 0;

    let seq = ctr::Loop::new(
        vec![beat],
        poly,
        map::Func::new(move |poly: &mut poly::Polyphony<_, _>| {
            freq *= INTERVALS[note % INTERVALS.len()];

            let n = note as u8;
            if beat * n < unt::Time::from_raw(LENGTH, SAMPLE_RATE) {
                poly.add(n, sgn(freq));
            }

            note += 1;
        }),
    );

    // Gentle low pass.
    eff::flt::LoFiltered::new_coefs(
        seq,
        eff::flt::Biquad::low_pass(
            unt::Freq::from_hz(5000.0, SAMPLE_RATE),
            unt::QFactor::default(),
        ),
    )
}

/// Bass saw.
fn bass() -> impl SignalMut<Sample = smp::Mono> {
    let length = unt::Time::from_raw(LENGTH, SAMPLE_RATE);
    let fade = unt::Time::from_raw(FADE, SAMPLE_RATE);

    // Deep saw wave.
    let saw = |ratio: f64| {
        gen::Loop::<smp::Mono, _>::new(
            crv::Saw,
            unt::Freq::from_raw(BASE_FREQ / ratio, SAMPLE_RATE),
        )
    };
    let mix = rtn::Mix::new(saw(2.0), saw(1.5));

    // Lowers the pitch according to the main melody.
    let env = ctr::Time::new(
        mix,
        map::Func::new(
            |mix: &mut rtn::Mix<gen::Loop<_, _>, gen::Loop<_, _>>, time| {
                let exp = time / (6.0 * unt::Time::from_raw(BEAT, SAMPLE_RATE));
                for sgn in [&mut mix.0, &mut mix.1] {
                    *sgn.freq_mut() =
                        unt::Freq::from_raw((BASE_FREQ / 2.0) * COMMA.powf(exp), SAMPLE_RATE);
                }
            },
        ),
    );

    // Subtle low-pass.
    eff::flt::LoFiltered::new_coefs(
        eff::env::ArEnv::new_ar(env, eff::env::Ar::new(length, fade)),
        eff::flt::SingleZero::single_zero(-1.0).with_gain(unt::Vol::from_db(-15.0)),
    )
}

fn main() {
    let sample_rate = unt::SampleRate::default();
    let length = unt::Time::from_raw(LENGTH, SAMPLE_RATE);
    let fade = unt::Time::from_raw(FADE, SAMPLE_RATE);

    let pluck = pluck();
    let bass = bass();
    let mix = rtn::Mix::new(pluck, bass);

    let delay = unt::Time::from_raw(DELAY, SAMPLE_RATE);
    let mut delay_sgn = eff::dly::Exp::new_exp_owned(mix, delay, unt::Vol::ZERO);

    Song::new_func(length + fade, sample_rate, |time| {
        let delay_gain = 0.8 * (time / length).sqrt();
        delay_sgn.vol_mut().gain = delay_gain;

        // Normalizes volume to 1.
        let gain = 1.0 - delay_gain;
        let mut res = delay_sgn.next() * gain / 3.0;

        // Fade out.
        if time > length {
            res *= 1.0 - (time - length) / fade;
        }
        res
    })
    .export("pointillism/examples/boron.wav");
}
