//! Declares the [`Polyphony`] struct, which allows for multiple signals of the
//! same type to play at the same time.

use std::{collections::HashMap, hash::Hash};

use crate::prelude::*;

// Todo: use a generic hashmap instead.

/// A polyphonic signal.
///
/// This stores multiple instances of a signal `S`, which can be added and
/// stopped. Signals are internally removed as they are done, to save processing
/// power.
#[derive(Clone, Debug)]
pub struct Polyphony<K: Eq + Hash + Clone, S: Done> {
    /// The signals currently playing.
    signals: HashMap<K, S>,

    /// The number of signals that have been added in total.
    idx: usize,
}

impl<K: Eq + Hash + Clone, S: Done> Default for Polyphony<K, S> {
    fn default() -> Self {
        Self {
            signals: HashMap::default(),
            idx: 0,
        }
    }
}

impl<K: Eq + Hash + Clone, S: Done> Polyphony<K, S> {
    /// Initializes a new polyphonic signal, playing nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the structure of currently played signals.
    ///
    /// **Note that this is subject to change.**
    #[must_use]
    pub fn signals(&self) -> &HashMap<K, S> {
        &self.signals
    }

    /// Adds a signal, using a given key.
    pub fn add(&mut self, key: K, sgn: S) -> usize {
        let idx = self.idx;
        self.signals.insert(key, sgn);
        self.idx += 1;
        idx
    }

    /// Gets a reference to a particular signal.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&S> {
        self.signals.get(key)
    }

    /// Gets a mutable reference to a particular signal.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut S> {
        self.signals.get_mut(key)
    }

    /// Modifies a signal with the given key using the specified function.
    /// Returns whether the signal was found.
    pub fn modify<F: Fn(&mut S)>(&mut self, key: &K, f: F) -> bool {
        if let Some(sgn) = self.get_mut(key) {
            f(sgn);
            true
        } else {
            false
        }
    }

    /// Stops a given signal, returns whether it was successful.
    pub fn stop(&mut self, key: &K) -> bool
    where
        S: Stop,
    {
        self.modify(key, S::stop)
    }
}

impl<K: Eq + Hash + Clone, S: Done> Signal for Polyphony<K, S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.signals.values().map(Signal::get).sum()
    }

    fn advance(&mut self) {
        // Generators to clear.
        let mut clear = Vec::new();

        for (idx, sgn) in &mut self.signals {
            sgn.advance();

            if sgn.is_done() {
                clear.push(idx.clone());
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

impl<K: Eq + Hash + Clone, S: Done> Base for Polyphony<K, S> {
    type Base = Self;

    fn base(&self) -> &Self::Base {
        self
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self
    }
}

impl<K: Eq + Hash + Clone, S: Done> Panic for Polyphony<K, S> {
    fn panic(&mut self) {
        self.retrigger();
    }
}
