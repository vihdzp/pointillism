//! Declares the [`Polyphony`] struct, which allows for multiple signals of the
//! same type to play at the same time.

use std::{collections::HashMap, hash::Hash};

use crate::prelude::*;

/// A polyphonic signal.
///
/// This stores multiple instances of a signal `S`, which can be added and stopped. Signals are
/// internally removed as they are done, to save processing power.
///
/// We currently use a [`HashMap`] to store these signals, but this is subject to change. Likewise,
/// the exact trait requirements on `K` may change in the future, although unsigned integers and the
/// like should always be supported.
#[derive(Clone, Debug)]
pub struct Polyphony<K: Eq + Hash + Clone, S: Done> {
    /// The signals currently playing.
    signals: HashMap<K, S>,
}

impl<K: Eq + Hash + Clone, S: Done> Default for Polyphony<K, S> {
    fn default() -> Self {
        Self {
            signals: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash + Clone, S: Done> Polyphony<K, S> {
    /// Initializes a new polyphonic signal, playing nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an iterator over the inner signals and their keys.
    ///
    /// The exact data structure used to store this is subject to change, and thus not exposed in
    /// the API.
    pub fn signals(&self) -> impl Iterator<Item = (&K, &S)> {
        self.signals.iter()
    }

    /// Returns an iterator over mutable references to the inner signals and their keys.
    ///
    /// The exact data structure used to store this is subject to change, and thus not exposed in
    /// the API.
    pub fn signals_mut(&mut self) -> impl Iterator<Item = (&K, &mut S)> {
        self.signals.iter_mut()
    }

    /// Adds a signal, using a given key.
    ///
    /// If the key was already in use, the signal is overwritten.
    pub fn add(&mut self, key: K, sgn: S) {
        self.signals.insert(key, sgn);
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
    ///
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

    // We don't implement `panic` as it would clash with the `Panic` impl.

    /// Stops all signals currently playing.
    pub fn stop_all(&mut self)
    where
        S: Stop,
    {
        for (_, signal) in self.signals_mut() {
            signal.stop();
        }
    }
}

impl<K: Eq + Hash + Clone, S: Done> Signal for Polyphony<K, S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.signals.values().map(Signal::get).sum()
    }
}

impl<K: Eq + Hash + Clone, S: SignalMut + Done> SignalMut for Polyphony<K, S> {
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

impl<K: Eq + Hash + Clone, S: SignalMut + Done> Base for Polyphony<K, S> {
    impl_base!();
}

impl<K: Eq + Hash + Clone, S: SignalMut + Done> Panic for Polyphony<K, S> {
    fn panic(&mut self) {
        self.retrigger();
    }
}
