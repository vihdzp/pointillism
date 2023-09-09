//! Declares the most basic traits concerning [`Signals`](Signal).
//!
//! ## Example
//!
//! The following example is a simplified implementation of [`NoiseGen`](crate::prelude::NoiseGen).
//!
//! ```
//! # use pointillism::prelude::{Env, Sample, Signal, SignalMut};
//! /// A signal that produces random envelope data.
//! struct NoiseGen {
//!     /// The current random value.
//!     current: Env,
//! }
//!
//! impl Signal for NoiseGen {
//!     // This signal produces envelope data.
//!     type Sample = Env;
//!
//!     // Returns the current value.
//!     fn get(&self) -> Env {
//!         self.current
//!     }
//! }
//!
//! impl SignalMut for NoiseGen {
//!     // Updates the current value.
//!     fn advance(&mut self) {
//!         self.current = Env::rand();
//!     }
//!     
//!     // Retriggering a random signal amounts to choosing a new random value.
//!     fn retrigger(&mut self) {         
//!         self.current = Env::rand();
//!     }
//! }
//! ```

use crate::{sample::Sample, units::Freq};

/// A trait for a stream of data [`Samples`](Sample), generated every frame.
///
/// This data can either be audio data, meaning [`Mono`](crate::sample::Mono) or
/// [`Stereo`](crate::sample::Stereo), or envelope data [`Env`](crate::sample::Env).
///
/// Most signals will implement the stronger trait [`SignalMut`], meaning that the state of the
/// signal can be advanced. The main use case for this weaker trait is signal routing. For instance,
/// you can create two references to a [`SignalMut`] via [`Ref`](crate::prelude::Ref), and apply
/// separate effects to them.
///
/// ## Implementing the trait
///
/// In order to implement this trait on your custom type, you only need to define one method:
///
/// - [`get`](Signal::get): Gets the next sample from the signal.
///
/// See the [module docs](self) for an example implementation.
pub trait Signal {
    /// The type of sample generated by the signal.
    type Sample: Sample;

    /// Gets the next sample from the signal.
    ///
    /// This should return the same result when called repeatedly, as long as the signal isn't
    /// modified, and `advance` or `retrigger` is not called.
    fn get(&self) -> Self::Sample;

    /// Currently, `rust-analyzer` trips up sometimes that `get` is called, confusing it with
    /// [`ArrayLike::get`](crate::prelude::ArrayLike::get). This hack bypasses this.
    fn _get(&self) -> Self::Sample {
        self.get()
    }
}

/// A trait for a stream of data [`Samples`](Sample), generated every frame.
///
/// This data can either be audio data, meaning [`Mono`](crate::sample::Mono) or
/// [`Stereo`](crate::sample::Stereo), or envelope data [`Env`](crate::sample::Env).
///
/// This trait is stronger than the [`Signal`] trait, as it also allows the signal to be advanced by
/// a frame.
///
/// ## Implementing the trait
///
/// In order to implement this trait on your custom type, you need to implement [`Sample`] together
/// with the following two methods:
///
/// - [`advance`](SignalMut::advance): Advances the state of the signal by a frame.
/// - [`retrigger`](SignalMut::retrigger): Resets the signal to its initial state.
///
/// See the [module docs](self) for an example implementation.
pub trait SignalMut: Signal {
    /// Advances the state of the signal by a frame.
    fn advance(&mut self);

    /// Resets the signal to its initial state.
    fn retrigger(&mut self);

    /// Gets the next sample and advances the state of the signal.
    fn next(&mut self) -> Self::Sample {
        let res = self.get();
        self.advance();
        res
    }
}

/// A trait for a signal with a "main" frequency that can be modified.
///
/// This is implemented both for signals that have a frequency parameter such as
/// [`LoopGen`](crate::generators::LoopGen), as well as straightforward wrappers for these signals.
///
/// Not to be confused with [`Freq`].
pub trait Frequency: SignalMut {
    /// The "main" frequency of the signal.
    fn freq(&self) -> Freq;

    /// Returns a mutable reference to the "main" frequency of the signal.
    fn freq_mut(&mut self) -> &mut Freq;
}

/// A trait for a signal with a "base" signal that can be modified. This is often a generator in a
/// chain of effects. This is implemented both for basic signals that don't depend on others, as
/// well as straightforward wrappers of these.
///
/// This convenience trait doesn't really add functionality, but is instead meant to help make code
/// a bit more manageable.
pub trait Base: SignalMut {
    /// The "base" signal type.
    type Base: SignalMut;

    /// A reference to the "base" signal.
    fn base(&self) -> &Self::Base;

    /// A mutable reference to the "base" signal.
    fn base_mut(&mut self) -> &mut Self::Base;
}

/// Implements the [`Base`] trait on a base signal.
macro_rules! impl_base {
    () => {
        type Base = Self;

        fn base(&self) -> &Self {
            self
        }

        fn base_mut(&mut self) -> &mut Self {
            self
        }
    };
}

pub(crate) use impl_base;

/// Represents a signal that ends.
///
/// Is used in [`Polyphony`](crate::prelude::Polyphony) so that a synth can be cleared from memory
/// when it stops.
///
/// If a signal never ends, it should not implement this trait. If you really want to use such a
/// signal within a `Polyphony` object, wrap it in the [`Trailing`](crate::prelude::Trailing)
/// structure.
pub trait Done: Signal {
    /// Returns whether the signal has stopped producing any sound altogether.
    ///
    /// If this returns `true` once, it must return `true` in all successive times, unless
    /// retriggered. Further, if this returns `true`, then getting a sample from the signal must
    /// always return zero.
    fn is_done(&self) -> bool;
}

/// Represents a signal that can be stopped. You can think of the `stop` method as an analog to a
/// MIDI note off event.
///
/// Note that stopping a signal doesn't necessarily mean it will immediately stop producing sound.
/// Use [`Panic`] for this purpose.
pub trait Stop: SignalMut {
    /// Releases a note.
    fn stop(&mut self);
}

/// Represents a signal that can be stopped abruptly.
///
/// All sound or envelope data should stop being produced once the `panic` method is called.
///
/// Depending on how your code is structured, it might be easier to simply stop calling `next` on
/// your signal.
pub trait Panic: SignalMut {
    /// Stops all subsequent sound.
    fn panic(&mut self);
}
