use pointillism::prelude::*;

/// Project sample rate.
const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;
/// Pluck duration.
const PLUCK_TIME: f64 = 0.45;
/// Note duration.
const NOTE_TIME: f64 = 0.5;
/// Number of notes.
const NOTES: u16 = 8;

fn main() {
    // Saw wave with random phase.
    let saw = |freq: unt::RawFreq| {
        gen::Loop::<smp::Stereo, _>::new_rand_phase(
            crv::Saw,
            unt::Freq::from_raw(freq, SAMPLE_RATE),
        )
    };

    // Play a C major chord.
    let chord = eff::Volume::new(
        rtn::Mix::new(
            rtn::Mix::new(saw(unt::RawFreq::C3), saw(unt::RawFreq::E3)),
            saw(unt::RawFreq::G3),
        ),
        unt::Vol::MDB10,
    );

    // Low-pass filter the chord.
    //
    // The zero coefficients are dummy values that get replaced by the envelope.
    let filter = eff::flt::LoFiltered::new_coefs(chord, eff::flt::Biquad::default());

    // An envelope that closes the filter.
    let env = eff::MutSgn::new(
        filter,
        gen::Once::new(crv::PosInvSaw, unt::Time::from_sec(PLUCK_TIME, SAMPLE_RATE)),
        map::Func::new(
            |filter: &mut eff::flt::LoFiltered<_, 3, 2>, env: smp::Env| {
                let hz = (15.0 * env.0 * env.0 + 1.0) * 100.0;
                *filter.coefs_mut() = eff::flt::Biquad::low_pass(
                    unt::Freq::from_hz(hz, SAMPLE_RATE),
                    unt::QFactor(1.0),
                );
            },
        ),
    );

    // Retrigger the pluck in a loop.
    let note_time = unt::Time::from_sec(NOTE_TIME, SAMPLE_RATE);
    let mut env_loop = ctr::Loop::new(
        vec![note_time],
        env,
        map::Func::new(|sgn: &mut eff::MutSgn<_, _, _>| sgn.retrigger()),
    );

    pointillism::create_from_sgn(
        "pointillism/examples/pluck.wav",
        NOTES * note_time,
        SAMPLE_RATE,
        &mut env_loop,
    )
    .expect(pointillism::IO_ERROR);
}
