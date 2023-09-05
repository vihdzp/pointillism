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

/// Increments a value in `0..len` by one, and wraps it around.
///
/// This should be marginally more efficient than `index = (index + 1) % len`, as it avoids the more
/// costly modulo operation.
fn mod_advance(len: usize, index: &mut usize) {
    *index += 1;
    if *index == len {
        *index = 0;
    }
}

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

    /// Returns the time for the current event.
    pub fn current_time(&self) -> Option<Time> {
        self.times.get(self.index()).copied()
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

    /// Modifies the signal according to the function.
    fn modify(&mut self) {
        self.func.modify(&mut self.sgn);
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

    /// Skips to an event with a given index and applies it, returns whether it was successful.
    ///
    /// Note that the function modifying the signal will only be called once.
    pub fn skip_to(&mut self, index: usize) -> bool {
        self.index = index;
        let res = index < self.len();
        if res {
            self.since = Time::ZERO;
            self.modify();
            self.index += 1;
        }
        res
    }

    /// Skips to the next event and applies it, returns whether it was successful.
    ///
    /// This can be used right after initializing a [`Sequence`] so that the first event is applied
    /// immediately.
    pub fn skip_to_next(&mut self) -> bool {
        self.skip_to(self.index)
    }

    /// Attempts to read a single event, returns whether it was successful.
    fn read_event(&mut self) -> bool {
        match self.current_time() {
            Some(event_time) => {
                let read = self.since() >= event_time;
                if read {
                    self.since -= event_time;
                    self.modify();
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
/// Although it is not undefined behavior to initialize an empty loop, doing so will lead to panics
/// in other methods.
///
/// See the [module docs](self) for more information.
#[derive(Clone, Debug)]
pub struct Loop<S: SignalMut, F: Mut<S>> {
    /// The internal sequence.
    seq: Sequence<S, F>,
}

impl<S: SignalMut, F: Mut<S>> Loop<S, F> {
    /// Turns a sequence into a loop.
    ///
    /// ## Safety
    ///
    /// This method is entirely safe. However, panics can occur in other methods if an empty
    /// sequence is passed.
    pub const fn new_seq_unchecked(seq: Sequence<S, F>) -> Self {
        Self { seq }
    }

    /// Turns a sequence into a loop.
    ///
    /// ## Panics
    ///
    /// This method panics if the sequence is empty.
    pub fn new_seq(seq: Sequence<S, F>) -> Self {
        assert!(!seq.is_empty());
        Self { seq }
    }

    /// Initializes a new loop.
    ///
    /// ## Safety
    ///
    /// This method is entirely safe. However, panics can occur in other methods if an empty `times`
    /// array is passed.
    pub const fn new_unchecked(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self::new_seq_unchecked(Sequence::new(times, sgn, func))
    }

    /// Initializes a new loop.
    ///
    /// ## Panics
    ///
    /// This method panics if the `times` vector is empty.
    pub fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self::new_seq(Sequence::new(times, sgn, func))
    }

    /// Returns a reference to the list of time intervals between events.
    pub fn times(&self) -> &[Time] {
        self.seq.times()
    }

    /// Returns the time for the current event.
    ///
    /// ## Panics
    ///
    /// Panics if the loop is empty.
    pub fn current_time(&self) -> Time {
        self.seq.current_time().unwrap()
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

    /// Modifies the signal according to the function.
    fn modify(&mut self) {
        self.seq.modify();
    }

    /// The current event index.
    ///
    /// This index should, as a runtime invariant, always be less than the length of the loop,
    /// unless the loop is empty.
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
    ///
    /// Note that such a loop might cause other methods to panic.
    pub fn is_empty(&self) -> bool {
        self.seq.times.is_empty()
    }

    /// Skips to an event with a given index and applies it.
    ///
    /// Note that the function modifying the signal will only be called once. If this function keeps
    /// track of some index, for instance, it won't be updated correctly.
    ///
    /// ## Panics
    ///
    /// Panics if the loop is empty.
    pub fn skip_to(&mut self, index: usize) {
        self.seq.since = Time::ZERO;
        self.modify();
        self.seq.index = (index + 1) % self.len();
    }

    /// Skips to the next event and applies it, returns whether it was successful.
    ///
    /// This can be used right after initializing a [`Loop`] so that the first event is applied
    /// immediately.
    ///
    /// ## Panics
    ///
    /// Panics if the loop is empty.
    pub fn skip_to_next(&mut self) {
        self.seq.since = Time::ZERO;
        self.modify();
        mod_advance(self.len(), &mut self.seq.index);
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

        if self.seq.index == self.len() {
            self.seq.index = 0;
        }
    }

    fn retrigger(&mut self) {
        self.seq.retrigger();
    }
}

/// The function that arpeggiates a signal.
#[derive(Clone, Debug)]
pub struct Arp {
    /// The notes to play, in order.
    pub notes: Vec<Freq>,

    /// The index of the note currently playing.
    pub index: usize,
}

impl Arp {
    /// Initializes a new arpeggio with the given notes.
    #[must_use]
    pub const fn new(notes: Vec<Freq>) -> Self {
        Self { notes, index: 0 }
    }

    /// The currently played note.
    #[must_use]
    pub fn current(&self) -> Freq {
        self.notes[self.index]
    }

    /// The length of the arpeggio.
    #[must_use]
    pub fn len(&self) -> usize {
        self.notes.len()
    }

    /// Whether the arpeggio has no notes.
    ///
    /// Note that this will generally result in other methods panicking, and thus should be avoided.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }

    /// Advances to the next note in the arpeggio.
    pub fn advance(&mut self) {
        mod_advance(self.len(), &mut self.index);
    }

    /// Replaces the current arpeggio by a new one.
    ///
    /// We use an iterator in order to avoid duplicate allocations.
    pub fn set_arp<I: IntoIterator<Item = Freq>>(&mut self, notes: I) {
        self.notes.clear();
        self.notes.extend(notes);
        self.index = 0;
    }
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
    ///
    /// ## Safety
    ///
    /// This method is entirely safe. However, panics can occur in other methods if an empty `times`
    /// array is passed.
    pub const fn new_arp_unchecked(times: Vec<Time>, sgn: S, notes: Vec<Freq>) -> Self {
        Self::new_unchecked(times, sgn, Arp::new(notes))
    }

    /// Initializes a new [`Arpeggio`].
    ///
    /// Note that the note being played by the signal won't be updated until the first time interval
    /// transpires, unless you call [`Self::skip_to_next`].
    ///
    /// ## Panics
    ///
    /// his method panics if the times vector is empty.
    pub fn new_arp(times: Vec<Time>, sgn: S, notes: Vec<Freq>) -> Self {
        Self::new(times, sgn, Arp::new(notes))
    }

    /// Returns a reference to the arpeggiated notes.
    pub const fn arp(&self) -> &Arp {
        self.func()
    }

    /// Returns a mutable reference to the arpeggiated notes.
    pub fn arp_mut(&mut self) -> &mut Arp {
        self.func_mut()
    }
}
