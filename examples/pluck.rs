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
        LoopGen::<Stereo, _>::new_rand_phase(Saw, unt::Freq::from_raw(freq, SAMPLE_RATE))
    };

    // Play a C major chord.
    let chord = Volume::new(
        Mix::new(
            Mix::new(saw(unt::RawFreq::C4), saw(unt::RawFreq::E4)),
            saw(unt::RawFreq::G4),
        ),
        unt::Vol::MDB10,
    );

    // Low-pass filter the chord.
    //
    // The zero coefficients are dummy values that get replaced by the envelope.
    let filter = Filtered::new(chord, Biquad::zero());

    // An envelope that closes the filter.
    let env = MutSgn::new(
        filter,
        OnceGen::new(PosInvSaw, unt::Time::from_sec(PLUCK_TIME, SAMPLE_RATE)),
        Func::new(|filter: &mut Filtered<_, 3, 2>, env: Env| {
            let hz = (15.0 * env.0 * env.0 + 1.0) * 100.0;
            *filter.coefficients_mut() =
                Biquad::low_pass(unt::Freq::from_hz(hz, SAMPLE_RATE), unt::QFactor(1.0));
        }),
    );

    // Retrigger the pluck in a loop.
    let note_time = unt::Time::from_sec(NOTE_TIME, SAMPLE_RATE);
    let mut env_loop = Loop::new(
        vec![note_time],
        env,
        Func::new(|sgn: &mut MutSgn<_, _, _>| sgn.retrigger()),
    );

    pointillism::create_from_sgn(
        "examples/pluck.wav",
        NOTES * note_time,
        SAMPLE_RATE,
        &mut env_loop,
    )
    .unwrap();
}