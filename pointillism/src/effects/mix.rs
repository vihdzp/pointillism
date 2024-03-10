//! Functions for mixing signals together.

use crate::{prelude::*, traits::*};

/// Combines two [`Mono`] signals into a [`Stereo`] signal. One signal plays on each channel.
pub struct Stereo<X: Signal<Sample = smp::Mono>, Y: Signal<Sample = smp::Mono>>(pub X, pub Y);

impl<X: Signal<Sample = smp::Mono>, Y: Signal<Sample = smp::Mono>> Stereo<X, Y> {
    /// Initializes a new [`StereoMix`].
    pub const fn new(sgn1: X, sgn2: Y) -> Self {
        Self(sgn1, sgn2)
    }
}

impl<Z: Signal<Sample = smp::Mono> + Clone> Stereo<Z, Z> {
    /// Duplicates a [`Mono`] signal.
    pub fn dup(sgn: Z) -> Self {
        Self(sgn.clone(), sgn)
    }
}

impl<X: Signal<Sample = smp::Mono>, Y: Signal<Sample = smp::Mono>> Signal for Stereo<X, Y> {
    type Sample = smp::Stereo;

    fn get(&self) -> Self::Sample {
        smp::Stereo(self.0.get().0, self.1.get().0)
    }
}

impl<X: SignalMut<Sample = smp::Mono>, Y: SignalMut<Sample = smp::Mono>> SignalMut
    for Stereo<X, Y>
{
    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: Done<Sample = smp::Mono>, Y: Done<Sample = smp::Mono>> Done for Stereo<X, Y> {
    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

impl<X: Stop<Sample = smp::Mono>, Y: Stop<Sample = smp::Mono>> Stop for Stereo<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }
}

impl<X: Panic<Sample = smp::Mono>, Y: Panic<Sample = smp::Mono>> Panic for Stereo<X, Y> {
    fn panic(&mut self) {
        self.0.panic();
        self.1.panic();
    }
}

/// Adds two signals together.
///
/// If you want to mix more signals together (e.g. an entire song), it might be easier to manually
/// add the samples instead.
pub struct Mix<X: Signal, Y: Signal<Sample = X::Sample>>(pub X, pub Y);

impl<X: Signal, Y: Signal<Sample = X::Sample>> Mix<X, Y> {
    /// Initializes a new [`Mix`].
    pub const fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: Signal, Y: Signal<Sample = X::Sample>> Signal for Mix<X, Y> {
    type Sample = X::Sample;

    fn get(&self) -> Self::Sample {
        self.0.get() + self.1.get()
    }
}

impl<X: SignalMut, Y: SignalMut<Sample = X::Sample>> SignalMut for Mix<X, Y> {
    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: Done, Y: Done<Sample = X::Sample>> Done for Mix<X, Y> {
    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

impl<X: Stop, Y: Stop<Sample = X::Sample>> Stop for Mix<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }
}

impl<X: Panic, Y: Panic<Sample = X::Sample>> Panic for Mix<X, Y> {
    fn panic(&mut self) {
        self.0.panic();
        self.1.panic();
    }
}

/// The function that duplicates a [`Mono`] sample in both channels.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dup;

impl map::Map for Dup {
    type Input = smp::Mono;
    type Output = smp::Stereo;

    fn eval(&self, x: smp::Mono) -> smp::Stereo {
        smp::Audio::duplicate(&x)
    }
}

/// Duplicates a [`Mono`] signal to create a [`Stereo`] signal.
pub type Duplicate<S> = eff::MapSgn<S, Dup>;

impl<S: Signal<Sample = smp::Mono>> Duplicate<S> {
    /// Duplicates a [`Mono`] signal in both channels.
    pub const fn new_dup(sgn: S) -> Self {
        Self::new(sgn, Dup)
    }
}

/// A reference to another signal.
///
/// This can be used as an efficient way to "clone" a signal, in order to then use its output across
/// various other signals.
///
/// ## Example
///
/// In this simple example, we apply two different effects to a simple saw wave, and play them in
/// both ears.
///
/// This creates a weird pulsing effect.
///
/// ```
/// # use pointillism::{prelude::*, traits::*};
/// // The original signals.
/// let mut signal = gen::Loop::new(crv::Saw, unt::Freq::from_raw_default(unt::RawFreq::A3));
/// let mut trem_env = gen::LoopCurve::new(crv::PosSaw, unt::Freq::from_hz_default(1.5));
///
/// pointillism::create(
///     "examples/routing.wav",
///     unt::Time::from_sec_default(5.0), unt::SampleRate::default(),
///     |_| {
///         // Thanks to `Ref`, we're able to re-use these signals.
///         let sgn1 = eff::PwMapSgn::inf_clip(eff::mix::Ref::new(&signal));
///         let sgn2 = eff::Tremolo::new(eff::mix::Ref::new(&signal), eff::mix::Ref::new(&trem_env));
///         let stereo = eff::mix::Stereo::new(sgn1, sgn2);
///
///         // However, we must manually advance them.
///         let res = stereo.get();
///         signal.advance();
///         trem_env.advance();
///         res
///     }
/// )
/// .unwrap();
/// ```
pub struct Ref<'a, S: Signal>(pub &'a S);

impl<'a, S: Signal> Ref<'a, S> {
    /// Initializes a new [`Ref`].
    pub const fn new(sgn: &'a S) -> Self {
        Self(sgn)
    }
}

impl<'a, S: Signal> Signal for Ref<'a, S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.0.get()
    }
}

impl<'a, S: Done> Done for Ref<'a, S> {
    fn is_done(&self) -> bool {
        self.0.is_done()
    }
}
