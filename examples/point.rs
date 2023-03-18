use pointillism::{
    effects::{vol::Tremolo, Gate},
    generators::{
        mix::{Duplicate, Mix, StereoGen},
        noise::NoiseGen,
    },
    prelude::*,
};
use rand::Rng;

fn scrunch_fn(val: f64) -> f64 {
    -sgn(0.001 + 1.2 * val.powi(15))
}

pub struct Kick<S: Signal, V: Signal<Sample = Env>, F: Signal<Sample = Env>> {
    /// The signal controlling the waveform for the kick.
    pub sgn: S,

    /// The envelope controlling the volume of the kick.
    pub vol_env: V,

    /// The initial pitch of the kick.
    pub freq: Freq,

    /// The envelope controlling the frequency of the kick.
    pub freq_env: F,
}

impl<S: Signal, V: Signal<Sample = Env>, F: Signal<Sample = Env>> Kick<S, V, F> {
    pub fn new(sgn: S, vol_env: V, freq: Freq, freq_env: F) -> Self {
        Self {
            sgn,
            vol_env,
            freq,
            freq_env,
        }
    }
}

fn main() {
    let scrunch_sgn = CurveGen::new(Saw::new(), Freq::new(200.0));
    let scrunch_mono = Gate::new(scrunch_sgn, NoiseGen::new(), 1.0);
    let mut scrunch = StereoGen::duplicate(scrunch_mono);

    const KICK_FREQ: Freq = Freq::new(200.0);
    let kick_sgn = CurveGen::new(SawTri::tri(), KICK_FREQ);
    let kick_vib = Envelope::new_generic(
        kick_sgn,
        CurveEnv::new(Saw::new(), Time::new(0.06)),
        FnWrapper::new(|kick: &mut CurveGen<_>, val: f64| {
            *kick.freq_mut() = KICK_FREQ * (2.0 - val) / 3.0;
        }),
    );
    let kick = Tremolo::new(
        kick_vib,
        CurveEnv::new(PosComp::new_pos(SawTri::new(0.15)), Time::new(0.06)),
    );

    let snare_sgn = NoiseGen::<Stereo>::new();
    let snare = Tremolo::new(
        snare_sgn,
        CurveEnv::new(PosComp::new_pos(SawTri::new(0.15)), Time::new(0.06)),
    );

    let drums = Mix(Duplicate::new(kick), snare);

    let mut kick_loop = Loop::new(
        vec![Time::new(0.075)],
        drums,
        FnWrapper::new(|drums: &mut Mix<Duplicate<_>, Tremolo<_, _>>, _| {
            if rand::thread_rng().gen::<f64>() < 0.75 {
                drums.0.retrigger();
            } else {
                drums.1.retrigger();
            }
        }),
    );

    pointillism::create("examples/point.wav", Time::new(32.0), |time| {
        let threshold = scrunch_fn((time.seconds / 8.0) % 1.0);
        scrunch.0.threshold = threshold;
        scrunch.1.threshold = threshold;
        let mut signal = scrunch.next() / 2.0;

        if time >= Time::new(16.0) {
            signal += kick_loop.next().duplicate();
        }

        signal
    })
}
