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

fn main() {}
