//! An implementation of a Schroeder filter. Directly adapted from [Natural Sounding Artificial
//! Reverberation](https://www.aes.org/e-lib/browse.cfm?elib=343) by M. R. Schroeder.

use pointillism::prelude::*;

/// A single Schroeder all-pass filter.
struct SchroederUnit<S: Signal>(eff::dly::Exp<S, buf::Dyn<S::Sample>>)
where
    S::Sample: Audio;

impl<S: Signal> SchroederUnit<S>
where
    S::Sample: Audio,
{
    /// Initializes a Schroeder reverberation unit.
    fn new(sgn: S, gain: unt::Vol, delay: unt::Time) -> Self {
        Self(eff::dly::Exp::new_exp_owned(sgn, delay, gain))
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
    S::Sample: Audio,
{
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        let g = self.gain().gain;
        self.0._get() * (1.0 - g * g) - self.sgn().get() * g
    }
}

impl<S: SignalMut> SignalMut for SchroederUnit<S>
where
    S::Sample: Audio,
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
    S::Sample: Audio,
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
    // A laser sound should work.
    let signal = eff::Vibrato::new(
        gen::Loop::<smp::Stereo, _>::new(crv::Tri, unt::Freq::default()),
        unt::Freq::from_raw_default(unt::RawFreq::A4),
        gen::Loop::new(
            map::Comp::new(crv::PosSaw, map::Linear::rescale_unit(0.5, 2.0)),
            unt::Freq::from_hz_default(10.0),
        ),
    );

    // We set some basic settings.
    // The delays are set to random amounts, so that each is about 3x as long as the next.
    let gain = unt::Vol::MDB3;
    let decay = 10.0;
    let delays: [unt::Time; 4] =
        [347.17, 113.29, 37.31, 13.42].map_array(|&t| unt::Time::from_samples((t * decay) as u64));

    // Then we compose various reverbs on top of each other.
    let reverb_0 = SchroederUnit::new(signal, gain, delays[0]);
    let reverb_1 = SchroederUnit::new(reverb_0, gain, delays[1]);
    let reverb_2 = SchroederUnit::new(reverb_1, gain, delays[2]);
    let mut reverb_3 = SchroederUnit::new(reverb_2, gain, delays[3]);

    // Fades from dry to wet.
    let length = unt::Time::from_sec_default(10.0);
    Song::new(length, unt::SampleRate::CD, |time| {
        let p = time / length;
        (reverb_3.base().get() * p + reverb_3.next() * (1.0 - p)) / 2.0
    })
    .export("pointillism/examples/schroeder.wav");
}
