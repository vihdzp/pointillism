use std::collections::BTreeMap;

use crate::signal::{Signal, StopSignal};

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, sgn: S) -> usize {
        let idx = self.idx;
        self.signals.insert(idx, sgn);
        self.idx += 1;
        idx
    }

    pub fn get(&self, idx: usize) -> Option<&S> {
        self.signals.get(&idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut S> {
        self.signals.get_mut(&idx)
    }

    pub fn stop(&mut self, idx: usize) {
        if let Some(sgn) = self.get_mut(idx) {
            sgn.stop();
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
