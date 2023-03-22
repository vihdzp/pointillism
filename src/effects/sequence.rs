//! Declares sequences and loops.

use crate::prelude::*;

/// Changes a signal according to a specified function, at specified times.
#[derive(Clone, Debug)]
pub struct Sequence<S: Signal, F: Mut<S, Time>> {
    /// A list of time intervals between an event and the next.
    pub times: Vec<Time>,

    /// A signal being modified.
    pub sgn: S,

    /// The function modifying the signal.
    pub func: F,

    /// The current event being read.
    idx: usize,

    /// Time since last event.
    since: Time,

    /// Time that has passed since instantiation.
    total: Time,
}

impl<S: Signal, F: Mut<S, Time>> Sequence<S, F> {
    /// Initializes a new sequence.
    pub fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self {
            times,
            sgn,
            func,
            idx: 0,
            since: Time::ZERO,
            total: Time::ZERO,
        }
    }

    /// The current event index.
    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Time since last event.
    pub fn since(&self) -> Time {
        self.since
    }

    /// Time since instantiation.
    pub fn total(&self) -> Time {
        self.total
    }

    /// Returns a reference to the modified signal.
    pub fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the modified signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// The number of events.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Attempts to read a single event, returns whether it was successful.
    fn read_event(&mut self) -> bool {
        match self.times.get(self.idx()) {
            Some(&event_time) => {
                let read = self.since() >= event_time;

                if read {
                    self.since -= event_time;
                    self.func.modify(&mut self.sgn, self.total);
                    self.idx += 1;
                }

                read
            }

            None => false,
        }
    }

    /// Reads all current events.
    fn read_events(&mut self) {
        while self.read_event() {}
    }
}

impl<S: Signal, F: Mut<S, Time>> Signal for Sequence<S, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn.get()
    }

    fn advance(&mut self) {
        self.sgn.advance();
        self.since.advance();
        self.total.advance();
        self.read_events();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.idx = 0;
        self.since = Time::ZERO;
        self.total = Time::ZERO;
    }
}

/// Loops a list of events.
#[derive(Clone, Debug)]
pub struct Loop<S: Signal, F: Mut<S, Time>> {
    /// A list of time intervals between an event and the next.
    pub times: Vec<Time>,

    /// A signal being modified.
    pub sgn: S,

    /// The function modifying the signal.
    pub func: F,

    /// The current event being read.
    idx: usize,

    /// Time since last event.
    since: Time,

    /// Time that has passed since instantiation.
    total: Time,
}

impl<S: Signal, F: Mut<S, Time>> Loop<S, F> {
    /// Initializes a new sequence.
    pub fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self {
            times,
            sgn,
            func,
            idx: 0,
            since: Time::ZERO,
            total: Time::ZERO,
        }
    }

    /// The current event index.
    pub fn idx(&self) -> usize {
        self.idx
    }

    /// Time since last event.
    pub fn since(&self) -> Time {
        self.since
    }

    /// Time since instantiation.
    pub fn total(&self) -> Time {
        self.total
    }

    /// Returns a reference to the modified signal.
    pub fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the modified signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// The number of events in the loop.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Attempts to read a single event, returns whether it was successful.
    fn read_event(&mut self) -> bool {
        let event_time = self.times[self.idx() % self.len()];
        let read = self.since() >= event_time;

        if read {
            self.since -= event_time;
            self.func.modify(&mut self.sgn, self.total);
            self.idx += 1;
        }

        read
    }

    /// Reads all current events.
    fn read_events(&mut self) {
        while self.read_event() {}
    }
}

impl<S: Signal, F: Mut<S, Time>> Signal for Loop<S, F> {
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.sgn.get()
    }

    fn advance(&mut self) {
        self.sgn.advance();
        self.since.advance();
        self.total.advance();
        self.read_events();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.idx = 0;
        self.since = Time::ZERO;
        self.total = Time::ZERO;
    }
}
