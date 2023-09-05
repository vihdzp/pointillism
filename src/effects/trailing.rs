//! Defines structs that add extra trait functionality to other signals.

use crate::prelude::*;

/// A trailing signal.
///
/// It can be stopped, but doing so won't actually change the output.
///
/// **Important note**: Using this is somewhat of a hack. If used repeatedly in a
/// [`Polyphony`](crate::prelude::Polyphony) struct, it will greatly slow down the code.
pub struct Trailing<S: SignalMut> {
    /// The inner signal.
    pub sgn: S,
}

impl<S: SignalMut> Trailing<S> {
    /// Initializes a new [`Trailing`] signal.
    pub const fn new(sgn: S) -> Self {
        Self { sgn }
    }
}

impl<S: SignalMut> Signal for Trailing<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn.get()
    }
}

impl<S: SignalMut> SignalMut for Trailing<S> {
    fn advance(&mut self) {
        self.sgn.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
    }
}

impl<S: Frequency> Frequency for Trailing<S> {
    fn freq(&self) -> RawFreq {
        self.sgn.freq()
    }

    fn freq_mut(&mut self) -> &mut RawFreq {
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

impl<S: SignalMut> Done for Trailing<S> {
    fn is_done(&self) -> bool {
        false
    }
}

impl<S: SignalMut> Stop for Trailing<S> {
    fn stop(&mut self) {}
}

/// Makes a signal stop immediately.
///
/// After a signal is stopped, all subsequent outputs will be the zero sample. The signal will need
/// to be retriggered in order to produce other outputs.
pub struct Stopping<S: SignalMut> {
    /// The inner signal.
    pub sgn: S,

    /// Is the signal done?
    done: bool,
}

impl<S: SignalMut> Stopping<S> {
    /// Initializes a new [`Stopping`] signal.
    pub const fn new(sgn: S) -> Self {
        Self { sgn, done: false }
    }
}

impl<S: SignalMut> Signal for Stopping<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        if self.done {
            S::Sample::ZERO
        } else {
            self.sgn.get()
        }
    }
}

impl<S: SignalMut> SignalMut for Stopping<S> {
    fn advance(&mut self) {
        self.sgn.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.done = false;
    }
}

impl<S: Frequency> Frequency for Stopping<S> {
    fn freq(&self) -> RawFreq {
        self.sgn.freq()
    }

    fn freq_mut(&mut self) -> &mut RawFreq {
        self.sgn.freq_mut()
    }
}

impl<S: Base> Base for Stopping<S> {
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn.base_mut()
    }
}

impl<S: SignalMut> Done for Stopping<S> {
    fn is_done(&self) -> bool {
        self.done
    }
}

impl<S: SignalMut> Stop for Stopping<S> {
    fn stop(&mut self) {
        self.done = true;
    }
}

/// Represents the function that retriggers a signal.
///
/// This exists for convenience use in loops or sequences, such as:
///
/// ```
/// # use pointillism::{prelude::*, effects::trailing::Retrigger};
/// # let osc = LoopGen::<Mono, Sin>::default();
/// // Retriggers the oscillator `osc` once per second.
/// let mut song_loop = Loop::new(vec![Time::SEC], osc, Retrigger);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Retrigger;

impl<S: SignalMut> Mut<S> for Retrigger {
    fn modify(&mut self, sgn: &mut S) {
        sgn.retrigger();
    }
}
