//! Implements generators that read from a buffer.
//!
//! These are [`OnceBuf`] and [`LoopBuf`], which work analogously to [`Once`] and [`Loop`].

use crate::prelude::*;

/// Boilerplate common to [`OnceBuf`] and [`LoopBuf`].
macro_rules! buf_gen_boilerplate {
    () => {
        /// Returns the number of samples in the buffer.
        #[must_use]
        pub fn len(&self) -> usize {
            self.buffer.len()
        }

        /// Returns whether the buffer is empty.
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.buffer.is_empty()
        }

        /// Returns the inner slice.
        #[must_use]
        pub fn as_slice(&self) -> &[B::Item] {
            self.buffer.as_slice()
        }

        /// Returns a mutable reference to the inner slice.
        pub fn as_mut_slice(&mut self) -> &mut [B::Item]
        where
            B: BufferMut,
        {
            self.buffer.as_mut_slice()
        }

        /// Returns the current index.
        pub const fn index(&self) -> usize {
            self.index
        }
    };
}

/// A generator that reads through an audio buffer, once.
#[derive(Clone, Debug)]
pub struct OnceBuf<B: Buffer> {
    /// The inner buffer.
    pub buffer: B,

    /// The sample being read.
    index: usize,
}

impl<B: Buffer> OnceBuf<B> {
    /// Initializes a new [`OnceBuf`].
    #[must_use]
    pub const fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    buf_gen_boilerplate!();
}

impl<B: Buffer> Signal for OnceBuf<B> {
    type Sample = B::Item;

    fn get(&self) -> B::Item {
        self.buffer.get(self.index).unwrap_or_default()
    }
}

impl<B: Buffer> SignalMut for OnceBuf<B> {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<B: Buffer> Base for OnceBuf<B> {
    impl_base!();
}

impl<B: Buffer> Stop for OnceBuf<B> {
    fn stop(&mut self) {
        self.index = self.buffer.len();
    }
}

impl<B: Buffer> Done for OnceBuf<B> {
    fn is_done(&self) -> bool {
        self.index >= self.buffer.len()
    }
}

impl<B: Buffer> Panic for OnceBuf<B> {
    fn panic(&mut self) {
        self.stop();
    }
}

/// A generator that loops an audio buffer.
#[derive(Clone, Debug)]
pub struct LoopBuf<B: Buffer> {
    /// The inner buffer.
    pub buffer: B,

    /// The sample being read.
    index: usize,
}

impl<B: Buffer> LoopBuf<B> {
    /// Initializes a new [`LoopBuf`].
    #[must_use]
    pub const fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    buf_gen_boilerplate!();
}

impl<B: Buffer> Signal for LoopBuf<B> {
    type Sample = B::Item;

    fn get(&self) -> B::Item {
        self.buffer[self.index]
    }
}

impl<B: Buffer> SignalMut for LoopBuf<B> {
    fn advance(&mut self) {
        crate::mod_inc(self.len(), &mut self.index);
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<B: Buffer> Base for LoopBuf<B> {
    impl_base!();
}

/// Reads through a signal in chunks.
#[derive(Clone, Debug)]
pub struct Chunks<S: SignalMut, B: BufferMut<Item = S::Sample>> {
    /// The signal being read.
    pub sgn: S,

    /// The inner [`OnceBuf`] controlling playback.
    pub once: OnceBuf<B>,
}

impl<S: SignalMut, B: BufferMut<Item = S::Sample>> Chunks<S, B> {
    /// Initializes a new [`Chunks`] with a specified buffer. The buffer will be immediately
    /// overwritten.
    pub fn new_gen(mut sgn: S, mut buffer: B) -> Self {
        sgn.fill(&mut buffer);
        Self {
            sgn,
            once: OnceBuf::new(buffer),
        }
    }

    /// Returns a reference to the inner buffer.
    pub const fn buffer(&self) -> &B {
        &self.once.buffer
    }

    /// Computes the next chunk.
    fn next_chunk(&mut self) {
        self.once.index = 0;
        self.sgn.fill(&mut self.once.buffer);
    }
}

impl<S: SignalMut, B: BufferMut<Item = S::Sample> + Default> Chunks<S, B> {
    /// Initializes a new [`Chunks`] with a default buffer of the inferred type. The buffer will be
    /// immediately overwritten.
    pub fn new_default(sgn: S) -> Self {
        Self::new_gen(sgn, B::default())
    }
}

impl<S: SignalMut, const N: usize> Chunks<S, buf::Stc<S::Sample, N>>
where
    S::Sample: Audio,
{
    /// Initializes a new [`Chunks`] with a [`buf::Stc`] buffer.
    pub fn new(sgn: S) -> Self {
        Self::new_default(sgn)
    }
}

impl<S: SignalMut> Chunks<S, buf::Dyn<S::Sample>>
where
    S::Sample: Audio,
{
    /// Initializes a new [`Chunks`] with a [`buf::Dyn`] buffer of the specified size.
    pub fn new(sgn: S, samples: usize) -> Self {
        Self::new_gen(sgn, buf::Dyn::new(samples))
    }
}

impl<S: SignalMut, B: BufferMut<Item = S::Sample>> Signal for Chunks<S, B>
where
    S::Sample: Audio,
{
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.once._get()
    }
}

impl<S: SignalMut, B: BufferMut<Item = S::Sample>> SignalMut for Chunks<S, B>
where
    S::Sample: Audio,
{
    fn advance(&mut self) {
        if self.once.is_done() {
            self.next_chunk();
        }

        self.once.advance();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.next_chunk();
    }
}
