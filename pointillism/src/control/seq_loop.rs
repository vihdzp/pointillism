//! Declares the [`Seq`] and [`Loop`] types. These can be used to modify a [`Signal`] at regular
//! time intervals.
//!
//! Note that the [`Signal`] won't be immediately modified when the [`Seq`] or [`Loop`] is
//! initialized. It will only be modified after the first time interval transpires. You can call
//! [`Seq::skip`] or [`Loop::skip`] in order to immediately skip to and apply the first event.
//!
//! Also note that the time interval between events can be zero. The effect of this is to execute
//! these events simultaneously.
//!
//! ## Type aliases
//!
//! This file also defines various useful type aliases. [`Arpeggio`] serves to arpeggiate a signal
//! by changing its frequency in periodic intervals. [`MelodySeq`] and [`MelodyLoop`] both
//! functionally serve as piano rolls for a polyphonic signal.

use crate::prelude::*;

/// Changes a signal according to a specified function, at specified times.
///
/// See the [module docs](self) for more information.
#[derive(Clone, Debug)]
pub struct Seq<S: SignalMut, F: map::Mut<S>> {
    /// A list of time intervals between an event and the next.
    pub times: Vec<unt::Time>,
    /// Time since last event.
    since: unt::Time,
    /// The current event being read.
    index: usize,

    /// A signal being modified.
    sgn: S,
    /// The function modifying the signal.
    func: F,
}

impl<S: SignalMut, F: map::Mut<S>> Seq<S, F> {
    /// Initializes a new sequence.
    pub const fn new(times: Vec<unt::Time>, sgn: S, func: F) -> Self {
        Self {
            times,
            since: unt::Time::ZERO,
            index: 0,
            sgn,
            func,
        }
    }

    /// Time since last event.
    pub const fn since(&self) -> unt::Time {
        self.since
    }

    /// The current event index.
    pub const fn index(&self) -> usize {
        self.index
    }

    /// The number of events.
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Whether there are no events in the sequence.
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }

    /// Returns the time for the current event.
    pub fn current_time(&self) -> Option<unt::Time> {
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

    /// Skips to the next event and applies it, returns whether it was successful.
    ///
    /// This can be used right after initializing a [`Seq`] so that the first event is applied
    /// immediately.
    pub fn skip(&mut self) -> bool {
        let res = self.index < self.len();
        if res {
            self.since = unt::Time::ZERO;
            self.modify();
            self.index += 1;
        }
        res
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

    /// The total time this sequence takes to complete.
    ///
    /// This method is expensive, as it must add all the times together.
    pub fn total_time(&self) -> unt::Time {
        self.times.iter().copied().sum()
    }
}

impl<S: SignalMut, F: map::Mut<S>> Signal for Seq<S, F> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.sgn.get()
    }
}

impl<S: SignalMut, F: map::Mut<S>> SignalMut for Seq<S, F> {
    fn advance(&mut self) {
        self.sgn.advance();
        self.since.advance();
        self.read_events();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.index = 0;
        self.since = unt::Time::ZERO;
    }
}

/// Changes a signal according to a specified function, at specified times. These times are looped.
///
/// Although it is not undefined behavior to initialize an empty loop, doing so will lead to panics
/// in other methods.
///
/// See the [module docs](self) for more information.
#[derive(Clone, Debug)]
pub struct Loop<S: SignalMut, F: map::Mut<S>> {
    /// The internal sequence.
    seq: Seq<S, F>,
}

impl<S: SignalMut, F: map::Mut<S>> Loop<S, F> {
    /// Turns a sequence into a loop.
    ///
    /// ## Panics
    ///
    /// This method panics if the sequence is empty.
    pub fn new_seq(seq: Seq<S, F>) -> Self {
        assert!(!seq.is_empty());
        Self { seq }
    }

    /// Initializes a new loop.
    ///
    /// ## Panics
    ///
    /// This method panics if the `times` vector is empty.
    pub fn new(times: Vec<unt::Time>, sgn: S, func: F) -> Self {
        Self::new_seq(Seq::new(times, sgn, func))
    }

    /// Time since last event.
    pub const fn since(&self) -> unt::Time {
        self.seq.since
    }

    /// The current event index.
    ///
    /// This index should, as a runtime invariant, always be less than the length of the loop,
    /// unless the loop is empty.
    pub const fn index(&self) -> usize {
        self.seq.index
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

    /// Returns the time for the current event.
    ///
    /// ## Panics
    ///
    /// Panics if the loop is empty.
    pub fn current_time(&self) -> unt::Time {
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

    /// Skips to the next event and applies it, returns whether it was successful.
    ///
    /// This can be used right after initializing a [`Loop`] so that the first event is applied
    /// immediately.
    ///
    /// ## Panics
    ///
    /// Panics if the loop is empty.
    pub fn skip(&mut self) {
        self.seq.since = unt::Time::ZERO;
        self.modify();
        crate::mod_inc(self.len(), &mut self.seq.index);
    }

    /// The total time this loop takes to complete.
    ///
    /// This method is expensive, as it must add all the times together.
    pub fn total_time(&self) -> unt::Time {
        self.seq.total_time()
    }
}

impl<S: SignalMut, F: map::Mut<S>> Signal for Loop<S, F> {
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.seq.sgn.get()
    }
}

impl<S: SignalMut, F: map::Mut<S>> SignalMut for Loop<S, F> {
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
///
/// This is used to implement [`Arpeggio`].
#[derive(Clone, Debug)]
pub struct Arp {
    /// The notes to play, in order.
    pub notes: Vec<unt::Freq>,

    /// The index of the note currently playing.
    pub index: usize,
}

impl Arp {
    /// Initializes a new arpeggio with the given notes.
    #[must_use]
    pub const fn new(notes: Vec<unt::Freq>) -> Self {
        Self { notes, index: 0 }
    }

    /// The currently played note.
    #[must_use]
    pub fn current(&self) -> unt::Freq {
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
}

impl<S: Frequency> map::Mut<S> for Arp {
    fn modify(&mut self, sgn: &mut S) {
        *sgn.freq_mut() = self.current();
        crate::mod_inc(self.len(), &mut self.index);
    }
}

/// An arpeggiated signal.
///
/// ## Example
///
/// We create a single arpeggio which plays two chords.
///
/// ```
/// # use pointillism::{prelude::*, traits::*};
/// // Basic parameters.
/// const SAMPLE_RATE: unt::SampleRate = unt::SampleRate::CD;
/// const NOTE_TIME: unt::RawTime = unt::RawTime::new(3.0 / 32.0);
/// const LENGTH: unt::RawTime = unt::RawTime::new(3.0);
///
/// let note_time = unt::Time::from_raw(NOTE_TIME, SAMPLE_RATE);
/// let length = unt::Time::from_raw(LENGTH, SAMPLE_RATE);
///
/// // The notes played in the arpeggio.
/// let notes = [unt::RawFreq::C4, unt::RawFreq::E4, unt::RawFreq::G4, unt::RawFreq::A4]
///     .map(|raw| unt::Freq::from_raw(raw, SAMPLE_RATE))
///     .to_vec();
///
/// // Initializes the arpeggio.
/// let mut arp = ctr::Arpeggio::new_arp(
///     vec![note_time],
///     gen::Loop::<smp::Mono, _>::new(crv::Tri, unt::Freq::ZERO),
///     notes,
/// );
///
/// // Zero is a dummy value that gets replaced here.
/// arp.skip();
///
/// let mut timer = ctr::Timer::new(length);
/// pointillism::create("examples/arpeggio.wav", 2u8 * length, SAMPLE_RATE, |time| {
///     // We switch up the arpeggio after the first phrase.
///     if timer.tick(time) {
///         arp.notes_mut()[2] = unt::Freq::from_raw(unt::RawFreq::F4, SAMPLE_RATE);
///     }
///
///     arp.next()
/// })
/// .expect("IO error");
/// ```
pub type Arpeggio<S> = Loop<S, Arp>;

impl<S: Frequency> Arpeggio<S> {
    /// Initializes a new [`Arpeggio`].
    ///
    /// Note that the note being played by the signal won't be updated until the first time interval
    /// transpires, unless you call [`Self::skip`].
    ///
    /// ## Panics
    ///
    /// This method panics if the `times` vector is empty.
    pub fn new_arp(times: Vec<unt::Time>, sgn: S, notes: Vec<unt::Freq>) -> Self {
        Self::new(times, sgn, Arp::new(notes))
    }

    /// Returns a reference to the [`Arp`].
    pub const fn arp(&self) -> &Arp {
        self.func()
    }

    /// Returns a mutable reference to the [`Arp`].
    pub fn arp_mut(&mut self) -> &mut Arp {
        self.func_mut()
    }

    /// Returns a reference to the notes.
    pub fn notes(&self) -> &[unt::Freq] {
        &self.arp().notes
    }

    /// Returns a mutable reference to the notes.
    pub fn notes_mut(&mut self) -> &mut [unt::Freq] {
        &mut self.arp_mut().notes
    }
}
