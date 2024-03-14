//! An implementation of a Schroeder filter. Directly adapted from [Natural Sounding Artificial
//! Reverberation](https://www.aes.org/e-lib/browse.cfm?elib=343) by M. R. Schroeder.

use pointillism::prelude::*;

struct SchroederUnit<S: Signal>(eff::dly::ExpDelay<S, buf::Dyn<S::Sample>>)
where
    S::Sample: smp::Audio;

impl<S: Signal> SchroederUnit<S>
where
    S::Sample: smp::Audio,
{
    /// Initializes a Schroeder reverberation unit.
    fn new(sgn: S, gain: unt::Vol, delay: unt::Time) -> Self {
        Self(eff::dly::ExpDelay::new_exp_owned(sgn, delay, gain))
    }

    /// A reference to the inner signal.
    fn sgn(&self) -> &S {
        &self.0.sgn
    }

    /// A mutable reference to the inner signal.
    fn sgn_mut(&mut self) -> &mut S {
        &mut self.0.sgn
    }

    /// The gain amount.
    fn gain(&self) -> unt::Vol {
        self.0.vol()
    }
}

impl<S: Signal> Signal for SchroederUnit<S>
where
    S::Sample: smp::Audio,
{
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        let g = self.gain().gain;
        self.0._get() * (1.0 - g * g) - self.sgn().get() * g
    }
}

impl<S: SignalMut> SignalMut for SchroederUnit<S>
where
    S::Sample: smp::Audio,
{
    fn advance(&mut self) {
        self.0.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
    }
}

impl<S: Base> Base for SchroederUnit<S>
where
    S::Sample: smp::Audio,
{
    type Base = S::Base;

    fn base(&self) -> &Self::Base {
        self.sgn().base()
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self.sgn_mut().base_mut()
    }
}

fn main() {
    // We start with a harmonically complex waveform.
    // Something like a laser sound.
    let signal = eff::Vibrato::new(
        gen::Loop::<smp::Stereo, _>::new(crv::Saw, unt::Freq::default()),
        unt::Freq::from_raw_default(unt::RawFreq::A4),
        gen::Loop::new(
            map::Comp::new(crv::PosSaw, map::Linear::rescale_unit(0.5, 2.0)),
            unt::Freq::from_hz_default(10.0),
        ),
    );

    // Then we compose various reverbs on top of each other.
    let vol = unt::Vol::new(0.7);
    let delays: [unt::Time; 4] = [439, 139, 43, 13].map_array(|&s| unt::Time::from_samples(s));
    let reverb_0 = SchroederUnit::new(signal, vol, delays[0]);
    let reverb_1 = SchroederUnit::new(reverb_0, vol, delays[1]);
    let reverb_2 = SchroederUnit::new(reverb_1, vol, delays[2]);
    let mut reverb_3 = SchroederUnit::new(reverb_2, vol, delays[3]);

    // Fades from dry to wet.
    let length = unt::Time::from_sec_default(10.0);
    pointillism::create(
        "pointillism/examples/schroeder.wav",
        length,
        unt::SampleRate::CD,
        |time| {
            let p = time / length;
            (reverb_3.base().get() * p + reverb_3.next() * (1.0 - p)) / 2.0
        },
    )
    .unwrap();
}
