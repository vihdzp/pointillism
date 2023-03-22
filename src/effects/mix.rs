//! Functions for mixing signals together.

use crate::{
    map::Map,
    prelude::*,
    sample::{Mono, Stereo},
};

/// Combines two [`Mono`] signals into a [`Stereo`] signal.
pub struct StereoGen<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>>(pub X, pub Y);

impl<X: Signal<Sample = Mono>, Y: Signal<Sample = Mono>> StereoGen<X, Y> {
    /// Initializes a new [`StereoGen`].
    pub const fn new(sgn1: X, sgn2: Y) -> Self {
        Self(sgn1, sgn2)
    }
}

impl<Z: Signal<Sample = Mono> + Clone> StereoGen<Z, Z> {
    /// Duplicates a [`Mono`] signal.
    pub fn duplicate(sgn: Z) -> Self {
        Self(sgn.clone(), sgn)
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

impl<X: Stop<Sample = Mono>, Y: Stop<Sample = Mono>> Stop for StereoGen<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }

    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

/// Adds two signals together.
///
/// If you want to mix more signals together (e.g. an entire song), it might be
/// easier to manually add the samples instead.
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

impl<X: Stop, Y: Stop<Sample = X::Sample>> Stop for Mix<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }

    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
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

/// Duplicates a [`Mono`] signal in both channels.
pub type Duplicate<S> = MapSgn<S, Dup>;

impl<S: Signal<Sample = Mono>> Duplicate<S> {
    /// Initializes a new [`Duplicate`].
    pub const fn new(sgn: S) -> Self {
        Self::new_generic(sgn, Dup)
    }
}
