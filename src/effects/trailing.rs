use std::marker::PhantomData;

use crate::prelude::*;

/// A trailing signal.
///
/// It can be stopped, but doing so won't actually change the output.
///
/// **Important note**: Using this is somewhat of a hack. If used repeatedly in
/// a [`Polyphony`](crate::prelude::Polyphony) struct, it will greatly slow down
/// the code.
pub struct Trailing<S: Signal> {
    /// The inner signal.
    pub sgn: S,
}

impl<S: Signal> Trailing<S> {
    /// Initializes a new [`Trailing`] signal.
    pub const fn new(sgn: S) -> Self {
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

impl<S: Frequency> Frequency for Trailing<S> {
    fn freq(&self) -> Freq {
        self.sgn.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn.freq_mut()
    }
}

impl<S: Base> Base for Trailing<S> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn.base_mut()
    }
}

impl<S: Signal> Done for Trailing<S> {
    fn is_done(&self) -> bool {
        false
    }
}

impl<S: Signal> Stop for Trailing<S> {
    fn stop(&mut self) {}
}

/// Represents the function that retriggers a signal.
///
/// This exists for convenience use in loops or sequences, such as:
///
/// ```
/// # use pointillism::{prelude::*, signal::Retrigger};
/// # let osc = LoopGen::<Sin>::default();
/// // Retriggers the oscillator `osc` once per second.
/// let mut song_loop = Loop::new(vec![Time::new(1.0)], osc, Retrigger::new());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Retrigger<Y> {
    /// Dummy value.
    phantom: PhantomData<Y>,
}

impl<Y> Retrigger<Y> {
    /// Initializes the [`Retrigger`] function.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Y> Default for Retrigger<Y> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Signal, Y> Mut<S, Y> for Retrigger<Y> {
    fn modify(&mut self, sgn: &mut S, _: Y) {
        sgn.retrigger();
    }
}