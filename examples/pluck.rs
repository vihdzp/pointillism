use pointillism::prelude::*;

fn main() {
    let saw =
        |freq: RawFreq| LoopGen::<Stereo, _>::new_rand_phase(Saw, Freq::from_raw_default(freq));

    let chord = Volume::new(
        Mix::new(
            Mix::new(saw(RawFreq::C4), saw(RawFreq::E4)),
            saw(RawFreq::G4),
        ),
        Vol::MDB10,
    );

    pointillism::create_from_sgn(
        "examples/pluck.wav",
        Time::from_sec_default(1.0),
        SampleRate::default(),
        &mut Filtered::new(
            chord,
            dbg!(Biquad::hi_shelf(
                Freq::from_raw_default(RawFreq::C5),
                Vol::new(0.01),
                filter::design::QFactor::from_bw(Interval::OCTAVE),
            )),
        ),
    )
    .unwrap();
}
