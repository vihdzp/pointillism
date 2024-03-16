use pointillism::prelude::*;

fn main() {
    let sample_rate = unt::SampleRate::default();
    let sec = unt::Time::from_sec(1.0, sample_rate);
    let length = sec * 50u32;

    let metronome = ctr::Metronome::new(sec);

    let mut synth_1 = eff::env::ArEnv::new_ar(
        gen::Loop::<smp::Mono, _>::new(crv::Sin, unt::Freq::default()),
        eff::env::Ar::new(sec * 0.1, sec),
    );

    Song::new(length, sample_rate, |time| {
        if metronome.tick(time) {
            synth_1.next()
        } else {
            smp::Mono::ZERO
        }
    })
    .export("pointillism/examples/boron.wav");
}
