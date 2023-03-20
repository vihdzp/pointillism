use std::collections::BTreeMap;

use crate::signal::{Signal, StopSignal};

// Todo: use a generic hashmap instead.

/// A polyphonic signal.
///
/// This stores multiple instances of a signal `S`, which can be added and
/// stopped.
#[derive(Clone, Debug)]
pub struct Polyphony<S: StopSignal> {
    /// The signals currently playing.
    signals: BTreeMap<usize, S>,

    /// The number of signals that have been added in total.
    idx: usize,
}

impl<S: StopSignal> Default for Polyphony<S> {
    fn default() -> Self {
        Self {
            signals: BTreeMap::default(),
            idx: 0,
        }
    }
}

impl<S: StopSignal> Polyphony<S> {
    /// Initializes a new polyphonic signal, playing nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the structure of currently played signals.
    pub fn signals(&self) -> &BTreeMap<usize, S> {
        &self.signals
    }

    /// Adds a signal, returns its index.
    pub fn add(&mut self, sgn: S) -> usize {
        let idx = self.idx;
        self.signals.insert(idx, sgn);
        self.idx += 1;
        idx
    }

    /// Gets a reference to a particular signal.
    pub fn get(&self, idx: usize) -> Option<&S> {
        self.signals.get(&idx)
    }

    /// Gets a mutable reference to a particular signal.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut S> {
        self.signals.get_mut(&idx)
    }

    /// Stops a given signal, returns whether it was successful.
    pub fn stop(&mut self, idx: usize) -> bool {
        if let Some(sgn) = self.get_mut(idx) {
            sgn.stop();
            true
        } else {
            false
        }
    }
}

impl<S: StopSignal> Signal for Polyphony<S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.signals.values().map(Signal::get).sum()
    }

    fn advance(&mut self) {
        // Generators to clear.
        let mut clear = Vec::new();

        for (&idx, sgn) in self.signals.iter_mut() {
            sgn.advance();

            if sgn.is_done() {
                clear.push(idx);
            }
        }

        for idx in clear {
            self.signals.remove(&idx);
        }
    }

    fn retrigger(&mut self) {
        self.signals.clear();
    }
}
