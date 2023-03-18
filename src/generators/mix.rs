//! Functions for mixing signals together.

use crate::{
    map::Map,
    prelude::AudioSample,
    sample::{Mono, Stereo},
    signal::{MapSgn, Signal, StopSignal},
};

/// Combines two [`Mono`] signals into a [`Stereo`] signal.
pub struct StereoGen<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>>(pub X, pub Y);

impl<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>> StereoGen<X, Y> {
    /// Initializes a new [`StereoGen`].
    pub const fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: Signal<Sample = Mono> + Clone> StereoGen<X, X> {
    /// Duplicates a [`Mono`] signal.
    pub fn duplicate(x: X) -> Self {
        Self(x.clone(), x)
    }
}

impl<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>> Signal for StereoGen<X, Y> {
    type Sample = Stereo;

    fn get(&self) -> Self::Sample {
        Stereo(self.0.get().0, self.1.get().0)
    }

    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: StopSignal<Sample = Mono>, Y: StopSignal<Sample = Mono>> StopSignal for StereoGen<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }

    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

/// Adds two signals together.
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

    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: StopSignal, Y: StopSignal<Sample = X::Sample>> StopSignal for Mix<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }

    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

pub struct Dup;

impl Map for Dup {
    type Input = Mono;
    type Output = Stereo;

    fn eval(&self, x: Mono) -> Stereo {
        x.duplicate()
    }
}

pub type Duplicate<S> = MapSgn<S, Stereo, Dup>;

impl<S: Signal<Sample = Mono>> Duplicate<S> {
    pub const fn new(sgn: S) -> Self {
        Self::new_generic(sgn, Dup)
    }
}
