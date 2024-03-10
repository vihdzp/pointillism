//! Defines maps that act on samples.

use crate::prelude::*;
use std::marker::PhantomData;

/// The function that [flips](smp::Stereo::flip) a [`smp::Stereo`] signal.
#[derive(Copy, Clone, Debug, Default)]
pub struct Flip;

impl map::Map for Flip {
    type Input = smp::Stereo;
    type Output = smp::Stereo;

    fn eval(&self, x: smp::Stereo) -> smp::Stereo {
        x.flip()
    }
}

/// Converts a function into one applied pointwise to the entries of a [`Sample`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Pw<S: smp::Sample, F: map::Map<Input = f64, Output = f64>> {
    /// The function to apply.
    pub func: F,

    /// Dummy value.
    phantom: PhantomData<S>,
}

impl<S: smp::Sample, F: map::Map<Input = f64, Output = f64>> Pw<S, F> {
    /// Initializes a new [`Pw`] function.
    pub const fn new(func: F) -> Self {
        Self {
            func,
            phantom: PhantomData,
        }
    }
}

impl<S: smp::Sample, F: map::Map<Input = f64, Output = f64>> map::Map for Pw<S, F> {
    type Input = S;
    type Output = S;

    fn eval(&self, x: S) -> S {
        x.map(|y| self.func.eval(*y))
    }
}
