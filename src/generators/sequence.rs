//! Declares [`Sequences`](Sequence) and [`Loops`](Loop). These can be used to modify a [`Signal`]
//! at regular time intervals.
//!
//! Note that the [`Signal`] won't, by default, be immediately modified when the [`Sequence`] or
//! [`Loop`] is initialized. It will only be modified after the first time interval transpires. You
//! can call [`Sequence::skip_to_next`] or [`Loop::skip_to_next`] in order to immediately skip to
//! and apply the first event.
//!
//! Also note that the time intervals between events can be zero. The effect of this is to execute
//! these events simultaneously.

use crate::prelude::*;

/// Changes a signal according to a specified function, at specified times.
///
/// See the [module docs](self) for more information.
#[derive(Clone, Debug)]
pub struct Sequence<S: SignalMut, F: Mut<S>> {
    /// A list of time intervals between an event and the next.
    times: Vec<Time>,

    /// A signal being modified.
    sgn: S,

    /// The function modifying the signal.
    func: F,

    /// The current event being read.
    index: usize,

    /// Time since last event.
    since: Time,
}

impl<S: SignalMut, F: Mut<S>> Sequence<S, F> {
    /// Initializes a new sequence.
    pub const fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self {
            times,
            sgn,
            func,
            index: 0,
            since: Time::ZERO,
        }
    }

    /// Returns a reference to the list of time intervals between events.
    pub fn times(&self) -> &[Time] {
        &self.times
    }

    /// Returns a reference to the modified signal.
    pub const fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the modified signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// Returns a reference to the function modifying the signal.
    pub const fn func(&self) -> &F {
        &self.func
    }

    /// Returns a mutable reference to the function modifying the signal.
    pub fn func_mut(&mut self) -> &mut F {
        &mut self.func
    }

    /// The current event index.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Time since last event.
    pub const fn since(&self) -> Time {
        self.since
    }

    /// The number of events.
    pub fn len(&self) -> usize {
        self.times().len()
    }

    /// Whether there are no events in the sequence.
    pub fn is_empty(&self) -> bool {
        self.times().is_empty()
    }

    /// Skips to the next event and applies it, returns whether it was successful.
    pub fn skip_to_next(&mut self) -> bool {
        match self.times.get(self.index()) {
            Some(_) => {
                self.since = Time::ZERO;
                self.func.modify(&mut self.sgn);
                self.index += 1;
                true
            }

            None => false,
        }
    }

    /// Attempts to read a single event, returns whether it was successful.
    fn read_event(&mut self) -> bool {
        match self.times.get(self.index()) {
            Some(&event_time) => {
                let read = self.since() >= event_time;
                if read {
                    self.since -= event_time;
                    self.func.modify(&mut self.sgn);
                    self.index += 1;
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

impl<S: SignalMut, F: Mut<S>> Signal for Sequence<S, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn.get()
    }
}

impl<S: SignalMut, F: Mut<S>> SignalMut for Sequence<S, F> {
    fn advance(&mut self) {
        self.sgn.advance();
        self.since.advance();
        self.read_events();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.index = 0;
        self.since = Time::ZERO;
    }
}

/// Changes a signal according to a specified function, at specified times. These times are looped.
///
/// See the [module docs](self) for more information.
#[derive(Clone, Debug)]
pub struct Loop<S: SignalMut, F: Mut<S>> {
    /// The internal sequence.
    seq: Sequence<S, F>,
}

/// Initializes a new [`Loop`] from a [`Sequence`].
impl<S: SignalMut, F: Mut<S>> From<Sequence<S, F>> for Loop<S, F> {
    fn from(seq: Sequence<S, F>) -> Self {
        Self::new_seq(seq)
    }
}

impl<S: SignalMut, F: Mut<S>> Loop<S, F> {
    /// Initializes a new loop from a sequence.
    pub const fn new_seq(seq: Sequence<S, F>) -> Self {
        Self { seq }
    }

    /// Initializes a new loop.
    pub const fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self::new_seq(Sequence::new(times, sgn, func))
    }

    /// Returns a reference to the list of time intervals between events.
    pub fn times(&self) -> &[Time] {
        self.seq.times()
    }

    /// Returns a reference to the modified signal.
    pub const fn sgn(&self) -> &S {
        &self.seq.sgn
    }

    /// Returns a mutable reference to the modified signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.seq.sgn
    }

    /// Returns a reference to the function modifying the signal.
    pub const fn func(&self) -> &F {
        &self.seq.func
    }

    /// Returns a mutable reference to the function modifying the signal.
    pub fn func_mut(&mut self) -> &mut F {
        &mut self.seq.func
    }

    /// The current event index.
    pub const fn index(&self) -> usize {
        self.seq.index
    }

    /// Time since last event.
    pub const fn since(&self) -> Time {
        self.seq.since
    }

    /// The number of events in the loop.
    pub fn len(&self) -> usize {
        self.seq.times.len()
    }

    /// Whether there are no events in the loop.
    pub fn is_empty(&self) -> bool {
        self.seq.times.is_empty()
    }

    /// Skips to the next event and applies it, returns whether it was successful.
    pub fn skip_to_next(&mut self) -> bool {
        let res = self.seq.skip_to_next();
        self.seq.index %= self.seq.len();
        res
    }
}

impl<S: SignalMut, F: Mut<S>> Signal for Loop<S, F> {
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.seq.sgn.get()
    }
}

impl<S: SignalMut, F: Mut<S>> SignalMut for Loop<S, F> {
    fn advance(&mut self) {
        self.seq.advance();

        if self.seq.index >= self.len() {
            self.seq.index = 0;
        }
    }

    fn retrigger(&mut self) {
        self.seq.retrigger();
    }
}

/// The function that arpeggiates a signal.
pub struct Arp {
    /// The notes to play, in order.
    pub notes: Vec<Freq>,

    /// The index of the note currently playing.
    pub index: usize,
}

impl Arp {
    /// Initializes a new arpeggio with the given notes.
    pub const fn new(notes: Vec<Freq>) -> Self {
        Self { notes, index: 0 }
    }

    /// The currently played note.
    pub fn current(&self) -> Freq {
        self.notes[self.index]
    }

    /// The length of the arpeggio.
    pub fn len(&self) -> usize {
        self.notes.len()
    }

    /// Whether the arpeggio has no notes.
    ///
    /// Note that this will generally result in other methods panicking, and thus should be avoided.
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Advances to the next note in the arpeggio.
    pub fn advance(&mut self) {
        self.index = (self.index + 1) % self.len();
    }

    // pub fn set_arp(&mut self, notes:Vec<Freq>)
}

impl<S: Frequency> Mut<S> for Arp {
    fn modify(&mut self, sgn: &mut S) {
        *sgn.freq_mut() = self.current();
        self.advance();
    }
}

/// An arpeggiated signal.
pub type Arpeggio<S> = Loop<S, Arp>;

impl<S: Frequency> Arpeggio<S> {
    /// Initializes a new [`Arpeggio`].
    ///
    /// Note that the note being played by the signal won't be updated until the first time interval
    /// transpires, unless you call [`Self::skip_to_next`].
    pub const fn new_arp(sgn: S, times: Vec<Time>, notes: Vec<Freq>) -> Self {
        Self::new(times, sgn, Arp::new(notes))
    }
}
