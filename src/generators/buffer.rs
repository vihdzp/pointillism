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
            B: buf::Mut,
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
pub struct OnceBuf<B: buf::Ref> {
    /// The inner buffer.
    pub buffer: B,

    /// The sample being read.
    index: usize,
}

impl<B: buf::Ref> OnceBuf<B> {
    /// Initializes a new [`OnceBuf`].
    #[must_use]
    pub const fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    buf_gen_boilerplate!();
}

impl<B: buf::Ref> Signal for OnceBuf<B> {
    type Sample = B::Item;

    fn get(&self) -> B::Item {
        self.buffer.get(self.index).unwrap_or_default()
    }
}

impl<B: buf::Ref> SignalMut for OnceBuf<B> {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<B: buf::Ref> Base for OnceBuf<B> {
    impl_base!();
}

impl<B: buf::Ref> Stop for OnceBuf<B> {
    fn stop(&mut self) {
        self.index = self.buffer.len();
    }
}

impl<B: buf::Ref> Done for OnceBuf<B> {
    fn is_done(&self) -> bool {
        self.index >= self.buffer.len()
    }
}

impl<B: buf::Ref> Panic for OnceBuf<B> {
    fn panic(&mut self) {
        self.stop();
    }
}

/// A generator that loops an audio buffer.
#[derive(Clone, Debug)]
pub struct LoopBuf<B: buf::Ref> {
    /// The inner buffer.
    pub buffer: B,

    /// The sample being read.
    index: usize,
}

impl<B: buf::Ref> LoopBuf<B> {
    /// Initializes a new [`LoopBuf`].
    #[must_use]
    pub const fn new(buffer: B) -> Self {
        Self { buffer, index: 0 }
    }

    buf_gen_boilerplate!();
}

impl<B: buf::Ref> Signal for LoopBuf<B> {
    type Sample = B::Item;

    fn get(&self) -> B::Item {
        self.buffer[self.index]
    }
}

impl<B: buf::Ref> SignalMut for LoopBuf<B> {
    fn advance(&mut self) {
        crate::mod_inc(self.len(), &mut self.index);
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<B: buf::Ref> Base for LoopBuf<B> {
    impl_base!();
}
