//! Functions for mixing signals together.

use crate::prelude::*;

/// Combines two [`Mono`] signals into a [`Stereo`] signal. One signal plays on each channel.
pub struct StereoMix<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>>(pub X, pub Y);

impl<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>> StereoMix<X, Y> {
    /// Initializes a new [`StereoMix`].
    pub const fn new(sgn1: X, sgn2: Y) -> Self {
        Self(sgn1, sgn2)
    }
}

impl<Z: Signal<Sample = Mono> + Clone> StereoMix<Z, Z> {
    /// Duplicates a [`Mono`] signal.
    pub fn dup(sgn: Z) -> Self {
        Self(sgn.clone(), sgn)
    }
}

impl<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>> Signal for StereoMix<X, Y> {
    type Sample = Stereo;

    fn get(&self) -> Self::Sample {
        Stereo(self.0.get().0, self.1.get().0)
    }
}

impl<X: SignalMut<Sample = Mono>, Y: SignalMut<Sample = Mono>> SignalMut for StereoMix<X, Y> {
    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: Done<Sample = Mono>, Y: Done<Sample = Mono>> Done for StereoMix<X, Y> {
    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

impl<X: Stop<Sample = Mono>, Y: Stop<Sample = Mono>> Stop for StereoMix<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }
}

impl<X: Panic<Sample = Mono>, Y: Panic<Sample = Mono>> Panic for StereoMix<X, Y> {
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

impl Map for Dup {
    type Input = Mono;
    type Output = Stereo;

    fn eval(&self, x: Mono) -> Stereo {
        x.duplicate()
    }
}

impl<S: SignalMut<Sample = Mono>> MapSgn<S, Dup> {
    /// Duplicates a [`Mono`] signal in both channels.
    pub const fn dup(sgn: S) -> Self {
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
/// ```
/// # use pointillism::prelude::*;
/// // The original signals.
/// let mut signal = LoopGen::new(Saw, Freq::A3);
/// let mut trem_env = LoopCurveGen::new(PosSaw, Freq::new(3.0));
///
/// pointillism::create("examples/routing.wav", 5.0 * Time::SEC, |_| {
///     // Thanks to `Ref`, we're able to re-use these signals.
///     let sgn1 = PwMapSgn::inf_clip(Ref::new(&signal));
///     let sgn2 = Tremolo::new(Ref::new(&signal), Ref::new(&trem_env));
///     let stereo = StereoMix::new(sgn1, sgn2);
///
///     // However, we must manually advance them.
///     let res = stereo.get();
///     signal.advance();
///     trem_env.advance();
///     res
///   })
///   .unwrap();
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
