//! Declares [`Sequences`](Sequence) and [`Loops`](Loop). These can be used to modify a [`Signal`]
//! at regular time intervals.
//!
//! Note that the [`Signal`] won't, by default, be immediately modified when the [`Sequence`] or
//! [`Loop`] is initialized. It will only be modified after the first time interval transpires. You
//! can call [`Sequence::skip`] or [`Loop::skip`] in order to immediately skip to and apply the
//! first event.
//!
//! Also note that the time interval between events can be zero. The effect of this is to execute
//! these events simultaneously.
//!
//! ## Type aliases
//!
//! This file also defines various useful type aliases. [`Arpeggio`] serves to arpeggiate a signal
//! by changing its frequency in periodic intervals. [`MelodySeq`] and [`MelodyLoop`] both
//! functionally serve as piano rolls for a polyphonic signal.

#[cfg(feature = "midly")]
use midly::num::{u4, u7};

use crate::prelude::*;
use std::hash::Hash;

/// Changes a signal according to a specified function, at specified times.
///
/// See the [module docs](self) for more information.
#[derive(Clone, Debug)]
pub struct Sequence<S: SignalMut, F: Mut<S>> {
    /// A list of time intervals between an event and the next.
    pub times: Vec<Time>,
    /// Time since last event.
    since: Time,
    /// The current event being read.
    index: usize,

    /// A signal being modified.
    sgn: S,
    /// The function modifying the signal.
    func: F,
}

impl<S: SignalMut, F: Mut<S>> Sequence<S, F> {
    /// Initializes a new sequence.
    pub const fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self {
            times,
            since: Time::ZERO,
            index: 0,
            sgn,
            func,
        }
    }

    /// Time since last event.
    pub const fn since(&self) -> Time {
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

    /// Skips to the next event and applies it, returns whether it was successful.
    ///
    /// This can be used right after initializing a [`Sequence`] so that the first event is applied
    /// immediately.
    pub fn skip(&mut self) -> bool {
        let res = self.index < self.len();
        if res {
            self.since = Time::ZERO;
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
    pub fn total_time(&self) -> Time {
        self.times.iter().copied().sum()
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
    /// ## Panics
    ///
    /// This method panics if the sequence is empty.
    pub fn new_seq(seq: Sequence<S, F>) -> Self {
        assert!(!seq.is_empty());
        Self { seq }
    }

    /// Initializes a new loop.
    ///
    /// ## Panics
    ///
    /// This method panics if the `times` vector is empty.
    pub fn new(times: Vec<Time>, sgn: S, func: F) -> Self {
        Self::new_seq(Sequence::new(times, sgn, func))
    }

    /// Time since last event.
    pub const fn since(&self) -> Time {
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

    /// Skips to the next event and applies it, returns whether it was successful.
    ///
    /// This can be used right after initializing a [`Loop`] so that the first event is applied
    /// immediately.
    ///
    /// ## Panics
    ///
    /// Panics if the loop is empty.
    pub fn skip(&mut self) {
        self.seq.since = Time::ZERO;
        self.modify();
        crate::mod_inc(self.len(), &mut self.seq.index);
    }

    /// The total time this loop takes to complete.
    ///
    /// This method is expensive, as it must add all the times together.
    pub fn total_time(&self) -> Time {
        self.seq.total_time()
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
///
/// This is used to implement [`Arpeggio`].
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
}

impl<S: Frequency> Mut<S> for Arp {
    fn modify(&mut self, sgn: &mut S) {
        *sgn.freq_mut() = self.current();
        crate::mod_inc(self.len(), &mut self.index);
    }
}

/// An arpeggiated signal.
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

/// A note event in a piano roll.
///
/// This means either that a note with a certain index and some data is added, or that a note with a
/// certain index is stopped.
///
/// See [`Note`] for ideas on what the data might represent.
#[derive(Clone, Copy, Debug)]
pub enum NoteEvent<K: Eq + Hash + Clone, D> {
    /// Adds a note with a certain index and certain data.
    Add { key: K, data: D },
    /// Stops a note with a certain index.
    Stop { key: K },

    /// Does nothing. This exists so that loops can work properly.
    Skip,
}

/// A note in a piano roll, which has a start time, some length, and some associated data.
///
/// This data can be frequency, volume, velocity, or anything else you might associate with a note
/// on a piano roll.
#[derive(Clone, Copy)]
pub struct Note<D: Clone> {
    /// Note start time.
    pub start: Time,
    /// Note length.
    pub length: Time,
    /// Note data.
    pub data: D,
}

impl<D: Clone> Note<D> {
    /// Initializes a new note.
    pub const fn new(start: Time, length: Time, data: D) -> Self {
        Self {
            start,
            length,
            data,
        }
    }

    /// The time at which the note ends.
    pub fn end(&self) -> Time {
        self.length + self.start
    }

    /// Maps the data of the note through the function.
    pub fn map_data<E: Clone, F: FnOnce(D) -> E>(self, func: F) -> Note<E> {
        Note::new(self.start, self.length, func(self.data))
    }

    /// Returns the two note events associated with the note, bundled with their start time.
    #[rustfmt::skip]
    pub fn events<K: Eq + Hash + Clone>(&self, key: K) -> [(Time, NoteEvent<K, D>); 2] {
        [
            (
                self.start,
                NoteEvent::Add {
                    key: key.clone(),
                    data: self.data.clone(),
                },
            ),
            (
                self.end(),
                NoteEvent::Stop { key }
            ),
        ]
    }
}

/// The data pertaining to a MIDI note.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "midly")]
pub struct MidiNoteData {
    /// The channel number this note comes from.
    pub channel: u4,

    /// The MIDI note being played.
    pub key: u7,
    /// The velocity of the note.
    pub vel: u7,
}

#[cfg(feature = "midly")]
impl MidiNoteData {
    /// Initializes a new [`MidiNoteData`].
    pub const fn new(channel: u4, key: u7, vel: u7) -> Self {
        Self { channel, key, vel }
    }
}

/// A "note reader" function that reads through different note events in order.
///
/// This is used to implement [`MelodySeq`] and [`MelodyLoop`].
#[derive(Clone, Debug)]
pub struct Mel<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>>
where
    F::Output: Frequency + Stop + Done,
{
    /// The note events.
    pub events: Vec<NoteEvent<K, D>>,

    /// The index of the note currently playing.
    pub index: usize,

    /// The function that builds a new signal from the specified note data.
    pub func: F,
}

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> Mel<K, D, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Initializes a new note reader.
    ///
    /// This function takes a list of note events, and a function that builds signals from the given
    /// note data.
    ///
    /// ## Recommendations
    ///
    /// Every `Add` event should be matched with a `Stop` event. If you're building a sequence, then
    /// every `Add` event should go before the corresponding `Stop` event. In a loop, it's ok to
    /// have them backwards, if you have a note that starts in one iteration and ends in the next.
    ///
    /// You should try not to reuse indexes. If you add a note with a certain index while another is
    /// already playing, it will immediately overwrite it, resulting in clicking.
    ///
    /// Moreover, if you're going to use this in a loop, the list of notes should be nonempty.
    #[must_use]
    pub const fn new(events: Vec<NoteEvent<K, D>>, func: F) -> Self {
        Self {
            events,
            index: 0,
            func,
        }
    }

    /// The current note event.
    #[must_use]
    pub fn current(&self) -> &NoteEvent<K, D> {
        &self.events[self.index]
    }

    /// The length of the note reader.
    #[must_use]
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether the note reader has no notes.
    ///
    /// Note that this will generally result in other methods panicking, and thus should be avoided.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> Mut<Polyphony<K, F::Output>>
    for Mel<K, D, F>
where
    F::Output: Frequency + Stop + Done,
{
    fn modify(&mut self, sgn: &mut Polyphony<K, F::Output>) {
        match self.current() {
            NoteEvent::Add { key, data } => sgn.add(key.clone(), self.func.eval(data.clone())),
            NoteEvent::Stop { key } => {
                sgn.stop(key);
            }
            NoteEvent::Skip => {}
        }

        crate::mod_inc(self.len(), &mut self.index);
    }
}

/// A melody that plays from start to end.
pub type MelodySeq<K, D, F> = Sequence<Polyphony<K, <F as Map>::Output>, Mel<K, D, F>>;
/// A melody that loops.
pub type MelodyLoop<K, D, F> = Loop<Polyphony<K, <F as Map>::Output>, Mel<K, D, F>>;

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> MelodySeq<K, D, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Turns a [`Mel`] into a [`MelodySeq`].
    pub fn new_mel(times: Vec<Time>, mel: Mel<K, D, F>) -> Self {
        Self::new(times, Polyphony::new(), mel)
    }

    /// Initializes a new [`MelodySeq`].
    ///
    /// The passed function builds signals from the given note data.
    ///
    /// This isn't the most intuitive way to build the melody. You might want to use
    /// [`Self::piano_roll`] instead.
    pub fn new_melody(times: Vec<Time>, events: Vec<NoteEvent<K, D>>, func: F) -> Self {
        Self::new(times, Polyphony::new(), Mel::new(events, func))
    }

    /// Returns a reference to the underlying note reader.
    pub const fn mel(&self) -> &Mel<K, D, F> {
        self.func()
    }

    /// Returns a mutable reference to the underlying note reader.
    pub fn mel_mut(&mut self) -> &mut Mel<K, D, F> {
        self.func_mut()
    }

    /// Initializes a [`MelodySeq`] from the following data:
    ///
    /// - A list of all [`Notes`](Note) in the song.
    /// - A function that builds signals from the note data.
    /// - A function that casts the note indices into their keys.
    ///
    /// This is somewhat expensive to initialize, but is the easiest way to build a complex melody.
    /// Alternatively, you can input the raw [`NoteEvents`](NoteEvent) by using
    /// [`Self::new_melody`].
    pub fn piano_roll<G: FnMut(usize) -> K>(notes: Vec<Note<D>>, func: F, mut idx_cast: G) -> Self {
        let event_len = 2 * notes.len();

        // We first get all note events and sort them by time.
        let mut time_events = Vec::with_capacity(event_len);

        for (idx, note) in notes.into_iter().enumerate() {
            for pair in note.events(idx_cast(idx)) {
                time_events.push(pair);
            }
        }

        // We use a stable sort so that the event where a note gets added always goes before the
        // event where it's stopped, even in the degenerate case of a very short note.
        time_events.sort_by_key(|(time, _)| *time);

        // We can now retrieve the events and the time intervals between them.
        let mut last_time = Time::ZERO;
        let mut times = Vec::with_capacity(event_len);
        let mut events = Vec::with_capacity(event_len);

        for (time, event) in time_events {
            times.push(time - last_time);
            last_time = time;
            events.push(event);
        }

        Self::new_melody(times, events, func)
    }
}

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> MelodyLoop<K, D, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Turns a [`Mel`] into a [`MelodyLoop`].
    ///
    /// ## Panics
    ///
    /// This method panics if the `times` vector is empty.
    pub fn new_mel(times: Vec<Time>, mel: Mel<K, D, F>) -> Self {
        Self::new(times, Polyphony::new(), mel)
    }

    /// Initializes a new [`MelodyLoop`].
    ///
    /// The passed function builds signals from the given note data.
    ///
    /// This isn't the most intuitive way to build the melody. You might want to use
    /// [`Self::piano_roll`] instead.
    ///
    /// ## Panics
    ///
    /// This method panics if the `times` vector is empty.
    pub fn new_melody(times: Vec<Time>, events: Vec<NoteEvent<K, D>>, func: F) -> Self {
        Self::new(times, Polyphony::new(), Mel::new(events, func))
    }

    /// Returns a reference to the underlying note reader.
    pub const fn mel(&self) -> &Mel<K, D, F> {
        self.func()
    }

    /// Returns a mutable reference to the underlying note reader.
    pub fn mel_mut(&mut self) -> &mut Mel<K, D, F> {
        self.func_mut()
    }

    /// Initializes a [`MelodyLoop`] from the following data:
    ///
    /// - A list of all [`Notes`](Note) in the loop.
    /// - The length of the loop.
    /// - A function that builds signals from the note data.
    /// - A function that casts the note indices into their keys.
    ///
    /// This is somewhat expensive to initialize, but is the easiest way to build a complex melody.
    /// Alternatively, you can input the raw [`NoteEvents`](NoteEvent) by using
    /// [`Self::new_melody`].
    ///
    /// ## Panics
    ///
    /// This method panics if the `notes` vector is empty, if the length is zero, or if some note
    /// starts later than the loop ends.
    pub fn piano_roll<G: FnMut(usize) -> K>(
        notes: Vec<Note<D>>,
        length: Time,
        func: F,
        mut idx_cast: G,
    ) -> Self {
        let event_len = 2 * notes.len();

        // We first get all note events and sort them by time.
        let mut time_events = Vec::with_capacity(event_len);

        for (idx, note) in notes.into_iter().enumerate() {
            let [(start_time, start_event), (end_time, end_event)] = note.events(idx_cast(idx));
            time_events.push((start_time, start_event));
            time_events.push((end_time % length, end_event));
        }

        // We use a stable sort so that the event where a note gets added always goes before the
        // event where it's stopped, even in the degenerate case of a very short note.
        time_events.sort_by_key(|(time, _)| *time);

        // We can now retrieve the events and the time intervals between them.
        let mut last_time = Time::ZERO;
        let mut times = Vec::with_capacity(event_len + 1);
        let mut events = Vec::with_capacity(event_len + 1);

        for (time, event) in time_events {
            times.push(time - last_time);
            last_time = time;
            events.push(event);
        }

        // We add a dummy `Skip` event at the end, so the loop has the appropriate length.
        times.push(length - last_time);
        events.push(NoteEvent::Skip);

        Self::new_melody(times, events, func)
    }
}

/// Builds the times and the events for a MIDI file.
#[cfg(feature = "midly")]
fn midi_times_events<'a, K: Eq + Hash + Clone, G: FnMut(usize) -> K>(
    event_iter: midly::EventIter<'a>,
    tick_time: Time,
    mut idx_cast: G,
) -> midly::Result<(Vec<Time>, Vec<NoteEvent<K, MidiNoteData>>)> {
    // The things we want to return.
    let mut times = Vec::new();
    let mut events = Vec::new();

    // A simple but fast hash table that assigns every key + channel combination an index for the
    // latest played note.
    let mut latest = vec![usize::MAX; 128 * 16];

    // A unique note index, time since last event.
    let mut idx = 0;
    let mut since_last = 0;

    // Go over every event.
    for event in event_iter {
        let event = event?;
        since_last += event.delta.as_int();

        // We only read MIDI events.
        if let midly::TrackEventKind::Midi { channel, message } = event.kind {
            // Gets an index in our "hash map".
            let index = |key: u7| 128 * (channel.as_int() as usize) + key.as_int() as usize;

            // Stops the specified key.
            let mut stop = |key: u7| {
                events.push(NoteEvent::Stop {
                    key: idx_cast(latest[index(key)]),
                });
                times.push(since_last * tick_time);
                since_last = 0;
            };

            match message {
                // Note on event.
                midly::MidiMessage::NoteOn { key, vel } => {
                    stop(key);

                    // A note-on with velocity 0 turns the note off.
                    if vel != u7::new(0) {
                        // Add new note.
                        events.push(NoteEvent::Add {
                            key: idx_cast(idx),
                            data: MidiNoteData::new(channel, key, vel),
                        });
                        times.push(Time::ZERO);

                        latest[index(key)] = idx;
                        idx += 1;
                    }
                }

                // Note off event.
                midly::MidiMessage::NoteOff { key, vel: _ } => {
                    stop(key);
                }

                // Ignore anything else.
                _ => {}
            }
        }
    }

    Ok((times, events))
}

#[cfg(feature = "midly")]
impl<K: Eq + Hash + Clone, F: Map<Input = MidiNoteData>> MelodySeq<K, MidiNoteData, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Initializes a [`MelodySeq`] from the following data:
    ///
    /// - An iterator over all events in a single track.
    /// - A measure of the MIDI tick time.
    /// - A function that builds signals from the note data.
    /// - A function that casts the note indices into their keys.
    ///
    /// Note that MIDI files with changing BPM are unsupported. All events other than note on / note
    /// off are ignored.
    pub fn from_midi<'a, G: FnMut(usize) -> K>(
        event_iter: midly::EventIter<'a>,
        tick_time: Time,
        func: F,
        idx_cast: G,
    ) -> midly::Result<Self> {
        midi_times_events(event_iter, tick_time, idx_cast)
            .map(|(times, events)| Self::new_melody(times, events, func))
    }
}

#[cfg(feature = "midly")]
impl<K: Eq + Hash + Clone, F: Map<Input = MidiNoteData>> MelodyLoop<K, MidiNoteData, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Initializes a [`MelodyLoop`] from the following data:
    ///
    /// - An iterator over all events in a single track.
    /// - The length of the loop.
    /// - A measure of the MIDI tick time.
    /// - A function that builds signals from the note data.
    /// - A function that casts the note indices into their keys.
    ///
    /// Note that MIDI files with changing BPM are unsupported. All events other than note on / note
    /// off are ignored.
    ///
    /// ## Panics
    ///
    /// This method panics if the `notes` vector is empty, if the length is zero, or if some note
    /// starts later than the loop ends.
    pub fn from_midi<'a, G: FnMut(usize) -> K>(
        event_iter: midly::EventIter<'a>,
        length: Time,
        tick_time: Time,
        func: F,
        idx_cast: G,
    ) -> midly::Result<Self> {
        let (mut times, mut events) = midi_times_events(event_iter, tick_time, idx_cast)?;

        // We add a dummy `Skip` event at the end, so the loop has the appropriate length.
        times.push(length - *times.last().unwrap());
        events.push(NoteEvent::Skip);

        Ok(Self::new_melody(times, events, func))
    }
}
