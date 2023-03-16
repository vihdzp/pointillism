//! Declares the [`Signal`] trait and implements it for simple types.

use std::marker::PhantomData;

use crate::{sample::*, Map, MapMut};

/// A trait for a stream of data, generated every frame.
///
/// This data can either be audio data, or envelope data.
pub trait Signal {
    /// The type of sample generated by the signal.
    type Sample: Sample;

    /// Gets the next sample from the signal.
    fn get(&self) -> Self::Sample;

    /// Advances the state of the signal to the next sample.
    fn advance(&mut self);

    /// Resets the stream to its initial state.
    fn retrigger(&mut self);

    /// Gets the next sample and advances the state of the signal.
    fn next(&mut self) -> Self::Sample {
        let res = self.get();
        self.advance();
        res
    }
}

/// Represents the function that retriggers a signal.
#[derive(Clone, Copy, Debug)]
pub struct Retrigger<Y> {
    /// Dummy value.
    phantom: PhantomData<Y>,
}

impl<Y> Default for Retrigger<Y> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Y> Retrigger<Y> {
    /// Initializes the [`Retrigger`] function.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Signal, Y> MapMut<S, Y> for Retrigger<Y> {
    fn modify(&mut self, sgn: &mut S, _: Y) {
        sgn.retrigger();
    }
}

/// Represents a signal that can be stopped.
pub trait StopSignal: Signal {
    /// Releases a note.
    fn stop(&mut self);

    /// Returns whether the signal has stopped producing any sound altogether.
    fn is_done(&self) -> bool;
}

/// A trailing signal. It can be stopped, but won't actually stop producing an
/// output.
pub struct Trailing<S: Signal> {
    /// The inner signal.
    pub sgn: S,
}

impl<S: Signal> Trailing<S> {
    pub fn new(sgn: S) -> Self {
        Self { sgn }
    }
}

impl<S: Signal> Signal for Trailing<S> {
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.sgn.get()
    }

    fn advance(&mut self) {
        self.sgn.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
    }
}

impl<S: Signal> StopSignal for Trailing<S> {
    fn stop(&mut self) {}

    fn is_done(&self) -> bool {
        false
    }
}

/// Maps a signal to another via a specified map.
#[derive(Clone, Copy, Debug)]
pub struct MapSgn<S: Signal, Y: Sample, F: Map<S::Sample, Y>> {
    /// The signal being mapped.
    pub sgn: S,

    /// The map being applied.
    pub map: F,

    /// Dummy variable.
    phantom: PhantomData<Y>,
}

impl<S: Signal, Y: Sample, F: Map<S::Sample, Y>> MapSgn<S, Y, F> {
    /// Initializes a generic [`MapSgn`].
    ///
    /// There are many type aliases for specific subtypes of [`MapSgn`], and
    /// these will often provide more convenient instantiations via `new`.
    pub const fn new_generic(sgn: S, map: F) -> Self {
        Self {
            sgn,
            map,
            phantom: PhantomData,
        }
    }

    /// Returns a reference to the original signal.
    pub const fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the original signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }
}

impl<S: Signal, Y: Sample, F: Map<S::Sample, Y>> Signal for MapSgn<S, Y, F> {
    type Sample = Y;

    fn get(&self) -> Y {
        self.map.eval(self.sgn.get())
    }

    fn advance(&mut self) {
        self.sgn.advance()
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
    }
}

impl<S: StopSignal, Y: Sample, F: Map<S::Sample, Y>> StopSignal for MapSgn<S, Y, F> {
    fn stop(&mut self) {
        self.sgn.stop();
    }

    fn is_done(&self) -> bool {
        self.sgn.is_done()
    }
}

/// Applies a function pointwise to the entries of a [`Sample`].
#[derive(Clone, Debug)]
pub struct Pointwise<S: Sample, F: Map<f64, f64>> {
    /// The function to apply.
    pub func: F,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: Sample, F: Map<f64, f64>> Pointwise<S, F> {
    /// Initializes a new [`Pointwise`] function.
    pub const fn new(func: F) -> Self {
        Self {
            func,
            phantom: PhantomData,
        }
    }
}

impl<S: Sample, F: Map<f64, f64>> Map<S, S> for Pointwise<S, F> {
    fn eval(&self, x: S) -> S {
        x.map(|y| self.func.eval(y))
    }
}

/// Maps a signal through a pointwise function.
pub type PointwiseMapSgn<S, F> =
    MapSgn<S, <S as Signal>::Sample, Pointwise<<S as Signal>::Sample, F>>;

impl<S: Signal, F: Map<f64, f64>> PointwiseMapSgn<S, F> {
    /// Initializes a new [`PointwiseMapSgn`].
    pub const fn new_pointwise(sgn: S, func: F) -> Self {
        Self::new_generic(sgn, Pointwise::new(func))
    }

    /// Returns a reference to the function modifying the signal.
    pub const fn func(&self) -> &F {
        &self.map.func
    }

    /// Returns a mutable reference to the function modifying the signal.
    pub fn func_mut(&mut self) -> &mut F {
        &mut self.map.func
    }
}

/// A map which converts an envelope into mono audio.
#[derive(Clone, Copy, Debug, Default)]
pub struct EnvToMono;

impl Map<Env, Mono> for EnvToMono {
    fn eval(&self, x: Env) -> Mono {
        x.into()
    }
}

/// Plays an envelope as a [`Mono`] audio file.
///
/// For very low-frequency envelopes, this might lead to undesirable sounds.
pub type EnvGen<S> = MapSgn<S, Mono, EnvToMono>;

impl<S: Signal<Sample = Env>> EnvGen<S> {
    /// Initializes a new [`EnvGen`].
    pub fn new_env(sgn: S) -> Self {
        Self::new_generic(sgn, EnvToMono)
    }
}
