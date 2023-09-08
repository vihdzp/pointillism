//! Convenience structures to play melodies.
//!
//! We define the [`MelodySeq`] and [`MelodyLoop`] structures, which play a [`Melody`] from start to
//! finish, or in a loop. A [`Melody`] can be defined from an (unordered) list of [`Notes`](Note) in
//! the obvious way.
//!
//! ## Example
//!
//! We load "Twinkle Twinkle Little Star" as a melody, and play it on a simple synth.
//!
//! ```
//! # use pointillism::prelude::*;
//! // Project sample rate.
//! const SAMPLE_RATE: SampleRate = SampleRate::CD;
//!
//! // A quarter note.
//! let q = Time::from_sec(0.5, SAMPLE_RATE).floor();
//! // The loop length.
//! let length = 16u8 * q;
//! // Release time for each note.
//! let release = q;
//!
//! // The notes that make up the melody.
//! let notes = [
//!     Note::new(Time::ZERO, q, RawFreq::C3),     // Twin-
//!     Note::new(q, q, RawFreq::C3),              // kle
//!     Note::new(2u8 * q, q, RawFreq::G3),        // Twin-
//!     Note::new(3u8 * q, q, RawFreq::G3),        // kle
//!     Note::new(4u8 * q, q, RawFreq::A3),        // Li-
//!     Note::new(5u8 * q, q, RawFreq::A3),        // ttle
//!     Note::new(6u8 * q, 2u8 * q, RawFreq::G3),  // star,
//!     Note::new(8u8 * q, q, RawFreq::F3),        // How
//!     Note::new(9u8 * q, q, RawFreq::F3),        // I
//!     Note::new(10u8 * q, q, RawFreq::E3),       // won-
//!     Note::new(11u8 * q, q, RawFreq::E3),       // der
//!     Note::new(12u8 * q, q, RawFreq::D3),       // what
//!     Note::new(13u8 * q, q, RawFreq::D3),       // you
//!     Note::new(14u8 * q, 2u8 * q, RawFreq::C3), // are.
//!     Note::new(14u8 * q, 2u8 * q, RawFreq::G3),
//! ]
//! .map(|note| note.map_data(|raw| Freq::from_raw(raw, SAMPLE_RATE)));
//!
//! // Each note is a triangle wave, with a simple ADSR envelope, playing the corresponding note.
//! let func = |freq: Freq| {
//!     AdsrEnvelope::new_adsr(
//!         LoopGen::<Mono, _>::new(Tri, freq),
//!         Time::from_sec(0.1, SAMPLE_RATE),
//!         q,
//!         Vol::new(0.2),
//!         release,
//!     )
//! };
//!
//! let melody = Melody::piano_roll(notes, |idx| idx as u8);
//! let mut melody_loop = MelodyLoop::new_melody(melody, FnWrapper::new(func));
//! let mut timer = Timer::new(2u8 * length);
//!
//! // We play the melody twice.
//! pointillism::create(
//!     "examples/twinkle.wav",
//!     2u8 * length + release,
//!     SAMPLE_RATE,
//!     |time| {
//!         // After the melody has been played twice, stop all voices.
//!         if timer.tick(time) {
//!             melody_loop.sgn_mut().stop_all();
//!         }
//!
//!         0.5 * if time < 2u8 * length {
//!             // Play as usual.
//!             melody_loop.next()
//!         } else {
//!             // Stop the loop, just play the inner fading signal instead.
//!             melody_loop.sgn_mut().next()
//!         }
//!     },
//! )
//! .expect("IO error!");
//! ```

use crate::prelude::*;
use std::hash::Hash;

#[cfg(feature = "midly")]
use midly::num::{u4, u7};

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

/// A note in a piano roll, which has a start time, some length, and some associated data. The note
/// can potentially be trailing.
///
/// This data can be [frequency](Freq), [volume](Vol), MIDI data, velocity, or anything else you
/// might associate with a note on a piano roll.
#[derive(Clone, Copy)]
pub struct Note<D: Clone> {
    /// Note start time.
    pub start: Time,

    /// Note length.
    ///
    /// If set to `None`, the note is trailing.
    pub length: Option<Time>,

    /// Note data.
    pub data: D,
}

impl<D: Clone> Note<D> {
    /// Initializes a new note.
    ///
    /// If the length is set to `None`, the note is trailing.
    pub const fn new_raw(start: Time, length: Option<Time>, data: D) -> Self {
        Self {
            start,
            length,
            data,
        }
    }

    /// Initializes a new note.
    pub const fn new(start: Time, length: Time, data: D) -> Self {
        Self::new_raw(start, Some(length), data)
    }

    /// Initializes a new trailing note.
    ///
    /// If you're going to use this, you should probably make sure your note does eventually stop
    /// anyways!
    pub const fn new_trailing(start: Time, data: D) -> Self {
        Self::new_raw(start, None, data)
    }

    /// The time at which the note ends.
    pub fn end(&self) -> Option<Time> {
        self.length.map(|t| t + self.start)
    }

    /// Maps the data of the note through the function.
    pub fn map_data<E: Clone, F: FnOnce(D) -> E>(self, func: F) -> Note<E> {
        Note::new_raw(self.start, self.length, func(self.data))
    }

    /// The time and event associated with the note start.
    pub fn start_event<K: Eq + Hash + Clone>(&self, key: K) -> (Time, NoteEvent<K, D>) {
        (
            self.start,
            NoteEvent::Add {
                key,
                data: self.data.clone(),
            },
        )
    }

    /// The time and event associated with the note end, if applicable.
    pub fn end_event<K: Eq + Hash + Clone>(&self, key: K) -> Option<(Time, NoteEvent<K, D>)> {
        self.end().map(|end| (end, NoteEvent::Stop { key }))
    }
}

/// The data pertaining to a MIDI note.
///
/// You can use this data in varied and creative ways, but the "standard" use is as follows:
///
/// - The key can be converted into a [`RawFreq`] using `RawFreq::new_midi(key.into())`, and this
///   can be then converted into [`Freq`] in the standard ways.
/// - Velocity can be used to attenuate or otherwise modify the sound. This mapping is not specified
///   in the MIDI specification, and you can use whatever you want (or nothing at all), but an
///   obvious choice is to use [`Vol::new_vel`].
/// - The channel can optionally be used to switch between different instruments or sounds.
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "midly")]
pub struct MidiNoteData {
    /// The channel number this note comes from.
    pub channel: u4,

    /// The MIDI note being played.
    pub key: u7,

    /// The velocity of the note.
    ///
    /// This will always be at least 1, since a velocity of 0 is taken to represent a note off event
    /// instead.
    pub vel: u7,
}

#[cfg(feature = "midly")]
impl MidiNoteData {
    /// Initializes a new [`MidiNoteData`].
    #[must_use]
    pub const fn new(channel: u4, key: u7, vel: u7) -> Self {
        Self { channel, key, vel }
    }
}

/// A "note reader" function that reads through different note events in order, and modifies a
/// [`Polyphony`] struct accordingly.
///
/// This is used to implement [`MelodySeq`] and [`MelodyLoop`].
#[derive(Clone, Debug)]
pub struct NoteReader<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>>
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

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> NoteReader<K, D, F>
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
    /// Every `Add` event should be matched with a `Stop` event, unless your notes stop
    /// automatically. If you're building a sequence, then every `Add` event should go before the
    /// corresponding `Stop` event. In a loop, it's okay to have them backwards, if you have a note
    /// that starts in one iteration and ends in the next.
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
    for NoteReader<K, D, F>
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
pub type MelodySeq<K, D, F> = Sequence<Polyphony<K, <F as Map>::Output>, NoteReader<K, D, F>>;
/// A melody that loops.
pub type MelodyLoop<K, D, F> = Loop<Polyphony<K, <F as Map>::Output>, NoteReader<K, D, F>>;

/// A series of timed [`NoteEvents`](NoteEvent). This can be used to build a [`MelodySeq`] or a
/// [`MelodyLoop`].
///
/// There's two main ways to build a [`Melody`]. You can provide the times and
/// [`NoteEvents`](NoteEvent) explicitly through [`Self::new`], though this is not very intuitive.
/// Alternatively, you can provide an (unordered) list of [`Notes`](Note) through
/// [`Self::piano_roll`] or [`Self::piano_roll_loop`]. This is slower but resembles the
/// functionality of a piano roll much more closely.
#[derive(Clone, Debug)]
pub struct Melody<K: Eq + Hash + Clone, D: Clone> {
    /// Times between successive note events.
    pub times: Vec<Time>,
    /// The ordered list of note events.
    pub events: Vec<NoteEvent<K, D>>,
}

impl<K: Eq + Hash + Clone, D: Clone> Melody<K, D> {
    /// Initializes a [`Melody`] given the explicit note events and the times between them.
    ///
    /// It might be easier to use [`Self::piano_roll`] or [`Self::piano_roll_loop`] instead.
    #[must_use]
    pub const fn new(times: Vec<Time>, events: Vec<NoteEvent<K, D>>) -> Self {
        Self { times, events }
    }

    /// Initializes a [`Melody`] from the following data:
    ///
    /// - A list of all [`Notes`](Note) in the song.
    /// - A function that casts the note indices into their keys.
    ///
    /// This is somewhat expensive to initialize, but is the easiest way to build a complex melody.
    /// Alternatively, you can input the raw [`NoteEvents`](NoteEvent) by using [`Self::new`].
    ///
    /// If used in a [`MelodyLoop`], the melody will immediately start from the beginning after the
    /// very last note stops. If this isn't what you want, or if you want notes that can end after
    /// the loop restarts, use [`Self::piano_roll_loop`] instead.
    pub fn piano_roll<N: IntoIterator<Item = Note<D>>, G: FnMut(usize) -> K>(
        notes: N,
        mut idx_cast: G,
    ) -> Self {
        // This only sets the capacity of the vectors.
        let notes = notes.into_iter();
        let event_len = 2 * notes.size_hint().0;

        // We first get all note events and sort them by time.
        let mut time_events = Vec::with_capacity(event_len);

        for (idx, note) in notes.enumerate() {
            let note_idx = idx_cast(idx);

            time_events.push(note.start_event(note_idx.clone()));
            if let Some(end_event) = note.end_event(note_idx) {
                time_events.push(end_event);
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

        Self::new(times, events)
    }

    /// Initializes a [`Melody`] from the following data:
    ///
    /// - A list of all [`Notes`](Note) in the loop.
    /// - The length of the loop.
    /// - A function that casts the note indices into their keys.
    ///
    /// This is somewhat expensive to initialize, but is the easiest way to build a complex melody.
    /// Alternatively, you can input the raw [`NoteEvents`](NoteEvent) by using [`Self::new`].
    ///
    /// Each note should be done before the loop returns to the note's start position, or it will
    /// get cut off by itself. Moreover, if the length of a non-trailing note is longer than the
    /// length of the loop, it will be stopped prematurely.
    ///
    /// ## Panics
    ///
    /// This method panics if the `notes` vector is empty, if the length is zero, or if some note
    /// starts later than the loop ends.
    pub fn piano_roll_loop<N: IntoIterator<Item = Note<D>>, G: FnMut(usize) -> K>(
        notes: N,
        length: Time,
        mut idx_cast: G,
    ) -> Self {
        // This only sets the capacity of the vectors.
        let notes = notes.into_iter();
        let event_len = 2 * notes.size_hint().0;

        // We first get all note events and sort them by time.
        let mut time_events = Vec::with_capacity(event_len);

        for (idx, note) in notes.enumerate() {
            let note_idx = idx_cast(idx);

            time_events.push(note.start_event(note_idx.clone()));
            if let Some((time, event)) = note.end_event(note_idx) {
                time_events.push((time % length, event));
            }
        }

        // We use a stable sort so that the event where a note gets added always goes before the
        // event where it's stopped, even in the degenerate case of a very short note.
        time_events.sort_by_key(|(time, _)| *time);

        // We can now retrieve the events and the time intervals between them.
        let last_time = time_events.last().unwrap().0;
        let mut prev_time = Time::ZERO;
        let mut times = Vec::with_capacity(event_len + 1);
        let mut events = Vec::with_capacity(event_len + 1);

        for (time, event) in time_events {
            times.push(time - prev_time);
            prev_time = time;
            events.push(event);
        }

        // We add a dummy `Skip` event at the end, so the loop has the appropriate length.
        times.push(length - last_time);
        events.push(NoteEvent::Skip);

        Self::new(times, events)
    }
}

#[cfg(feature = "midly")]
impl<K: Eq + Hash + Clone> Melody<K, MidiNoteData> {
    /// Builds a melody from a MIDI file.
    ///
    /// If used in a [`MelodyLoop`], the melody will immediately start from the beginning after the
    /// very last note stops. If this isn't what you want, use [`Self::from_midi_loop`] instead.
    ///
    /// ## Errors
    ///
    /// Any errors returned will result from the event iterator itself.
    pub fn from_midi<G: FnMut(usize) -> K>(
        event_iter: midly::EventIter,
        tick_time: Time,
        mut idx_cast: G,
    ) -> midly::Result<Self> {
        // The things we want to return.
        let mut times = Vec::new();
        let mut events = Vec::new();

        // A simple but fast hash table that assigns every key + channel combination an index for
        // the latest played note.
        //
        // TODO: benchmark against just using a hash table.
        let mut latest = [usize::MAX; 128 * 16];

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

                        // A note-on with velocity 0 just turns the note off.
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

        Ok(Self::new(times, events))
    }

    /// Builds a melody from a MIDI file.
    ///
    /// Each note should be done before the loop returns to the note's start position, or it will
    /// get cut off by itself.
    ///
    /// ## Panics
    ///
    /// This method panics if the `notes` vector is empty, if the length is zero, or if some note
    /// starts later than the loop ends.
    ///
    /// ## Errors
    ///
    /// Any errors returned will result from the event iterator itself.
    pub fn from_midi_loop<G: FnMut(usize) -> K>(
        event_iter: midly::EventIter,
        length: Time,
        tick_time: Time,
        idx_cast: G,
    ) -> midly::Result<Self> {
        let mut melody = Self::from_midi(event_iter, tick_time, idx_cast)?;
        let last_time = melody.times.iter().copied().sum();

        // We add a dummy `Skip` event at the end, so the loop has the appropriate length.
        melody.times.push(length - last_time);
        melody.events.push(NoteEvent::Skip);

        Ok(melody)
    }
}

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> MelodySeq<K, D, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Turns a [`NoteReader`] into a [`MelodySeq`].
    pub fn new_note_reader(times: Vec<Time>, note_reader: NoteReader<K, D, F>) -> Self {
        Self::new(times, Polyphony::new(), note_reader)
    }

    /// Initializes a new [`MelodySeq`] from a [`Melody`].
    ///
    /// The passed function builds signals from the given note data.
    pub fn new_melody(melody: Melody<K, D>, func: F) -> Self {
        Self::new_note_reader(melody.times, NoteReader::new(melody.events, func))
    }

    /// Returns a reference to the underlying [`NoteReader`].
    pub const fn note_reader(&self) -> &NoteReader<K, D, F> {
        self.func()
    }

    /// Returns a mutable reference to the underlying [`NoteReader`].
    pub fn note_reader_mut(&mut self) -> &mut NoteReader<K, D, F> {
        self.func_mut()
    }
}

impl<K: Eq + Hash + Clone, D: Clone, F: Map<Input = D>> MelodyLoop<K, D, F>
where
    F::Output: Frequency + Stop + Done,
{
    /// Turns a [`NoteReader`] into a [`MelodyLoop`].
    pub fn new_note_reader(times: Vec<Time>, note_reader: NoteReader<K, D, F>) -> Self {
        Self::new(times, Polyphony::new(), note_reader)
    }

    /// Initializes a new [`MelodyLoop`] from a [`Melody`].
    ///
    /// The passed function builds signals from the given note data.
    pub fn new_melody(melody: Melody<K, D>, func: F) -> Self {
        Self::new_note_reader(melody.times, NoteReader::new(melody.events, func))
    }

    /// Returns a reference to the underlying [`NoteReader`].
    pub const fn note_reader(&self) -> &NoteReader<K, D, F> {
        self.func()
    }

    /// Returns a mutable reference to the underlying [`NoteReader`].
    pub fn note_reader_mut(&mut self) -> &mut NoteReader<K, D, F> {
        self.func_mut()
    }
}
