use pointillism::prelude::{
    filter::{Biquad, Filtered},
    *,
};

fn main() {
    let saw =
        |freq: RawFreq| LoopGen::<Stereo, _>::new_rand_phase(Saw, Freq::from_raw_default(freq));

    let chord = Mix::new(
        Mix::new(saw(RawFreq::C4), saw(RawFreq::E4)),
        saw(RawFreq::G4),
    );

    let mut filter = Filtered::new(
        chord,
        Biquad::all_pass(
            Freq::from_raw_default(RawFreq::C6),
            filter::design::QFactor::from_bw(Interval::OCTAVE),
        ),
    );

    pointillism::create(
        "examples/filter_test.wav",
        Time::from_sec_default(1.0),
        SampleRate::default(),
        |_| (filter.sgn.get() + filter.next()) / 2.0,
    )
    .unwrap();
}
