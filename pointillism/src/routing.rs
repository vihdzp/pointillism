//! Structures for mixing or combining different signals together.

use crate::prelude::*;
use std::cell::UnsafeCell;

/// Combines two [`smp::Mono`] signals into a [`smp::Stereo`] signal. One signal plays on each
/// channel.
pub struct Stereo<X: Signal<Sample = smp::Mono>, Y: Signal<Sample = smp::Mono>>(pub X, pub Y);

impl<X: Signal<Sample = smp::Mono>, Y: Signal<Sample = smp::Mono>> Stereo<X, Y> {
    /// Initializes a new [`Stereo`].
    pub const fn new(sgn1: X, sgn2: Y) -> Self {
        Self(sgn1, sgn2)
    }
}

impl<Z: Signal<Sample = smp::Mono> + Clone> Stereo<Z, Z> {
    /// Duplicates a [`smp::Mono`] signal.
    pub fn dup(sgn: Z) -> Self {
        Self(sgn.clone(), sgn)
    }
}

impl<X: Signal<Sample = smp::Mono>, Y: Signal<Sample = smp::Mono>> Signal for Stereo<X, Y> {
    type Sample = smp::Stereo;

    fn get(&self) -> Self::Sample {
        smp::Stereo(self.0.get().0, self.1.get().0)
    }
}

impl<X: SignalMut<Sample = smp::Mono>, Y: SignalMut<Sample = smp::Mono>> SignalMut
    for Stereo<X, Y>
{
    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: Done<Sample = smp::Mono>, Y: Done<Sample = smp::Mono>> Done for Stereo<X, Y> {
    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

impl<X: Stop<Sample = smp::Mono>, Y: Stop<Sample = smp::Mono>> Stop for Stereo<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }
}

impl<X: Panic<Sample = smp::Mono>, Y: Panic<Sample = smp::Mono>> Panic for Stereo<X, Y> {
    fn panic(&mut self) {
        self.0.panic();
        self.1.panic();
    }
}

/// Adds two signals together.
///
/// If you want to mix more signals together (e.g. an entire song), it might be easier to manually
/// add the samples instead.
pub struct Mix<X: Signal, Y: Signal<Sample = X::Sample>>(pub X, pub Y);

impl<X: Signal, Y: Signal<Sample = X::Sample>> Mix<X, Y> {
    /// Initializes a new [`Mix`].
    pub const fn new(x: X, y: Y) -> Self {
        Self(x, y)
    }
}

impl<X: Signal, Y: Signal<Sample = X::Sample>> Signal for Mix<X, Y> {
    type Sample = X::Sample;

    fn get(&self) -> Self::Sample {
        self.0.get() + self.1.get()
    }
}

impl<X: SignalMut, Y: SignalMut<Sample = X::Sample>> SignalMut for Mix<X, Y> {
    fn advance(&mut self) {
        self.0.advance();
        self.1.advance();
    }

    fn retrigger(&mut self) {
        self.0.retrigger();
        self.1.retrigger();
    }
}

impl<X: Done, Y: Done<Sample = X::Sample>> Done for Mix<X, Y> {
    fn is_done(&self) -> bool {
        self.0.is_done() && self.1.is_done()
    }
}

impl<X: Stop, Y: Stop<Sample = X::Sample>> Stop for Mix<X, Y> {
    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }
}

impl<X: Panic, Y: Panic<Sample = X::Sample>> Panic for Mix<X, Y> {
    fn panic(&mut self) {
        self.0.panic();
        self.1.panic();
    }
}

/// The function that duplicates a [`smp::Mono`] sample in both channels.
#[derive(Clone, Copy, Debug, Default)]
pub struct Dup;

impl Map for Dup {
    type Input = smp::Mono;
    type Output = smp::Stereo;

    fn eval(&self, x: smp::Mono) -> smp::Stereo {
        x.duplicate()
    }
}

/// Duplicates a [`smp::Mono`] signal to create a [`Stereo`] signal.
pub type Duplicate<S> = eff::MapSgn<S, Dup>;

impl<S: Signal<Sample = smp::Mono>> Duplicate<S> {
    /// Duplicates a [`smp::Mono`] signal in both channels.
    pub const fn new_dup(sgn: S) -> Self {
        Self::new(sgn, Dup)
    }
}

/// A reference to another signal.
///
/// This can be used as a simple and efficient way to "clone" a signal, in order to use its output
/// across various other signals.
///
/// Note that due to Rust's aliasing rules, once a [`Ref<S>`] is created, the original signal can't
/// be modified until it's dropped. For this same reason, [`Ref<S>`] does not implement
/// [`SignalMut`], even if `S` does. The only way to "advance" a signal is to re-build it for each
/// sample. If this restriction is unacceptable, [`Cell`] can be used to provide interior
/// mutability.
///
/// ## Examples
///
/// In this example, we apply two different distortion effects to a single sine wave, and play them
/// in both ears. The left ear should be noticeably louder.
///
/// ```
/// # use pointillism::prelude::*;
/// // The original signal.
/// let mut signal = gen::Loop::new(crv::Sin, unt::Freq::from_raw_default(unt::RawFreq::A3));
///
/// pointillism::create(
///     "examples/routing.wav",
///     unt::Time::from_sec_default(3.0), unt::SampleRate::default(),
///     |_| {
///         // Thanks to `Ref`, we're able to re-use our signal.
///         // However, we need to re-define it for every sample.
///         let sgn1 = eff::PwMapSgn::inf_clip(rtn::Ref::new(&signal));
///         let sgn2 = eff::PwMapSgn::cubic(rtn::Ref::new(&signal));
///         let stereo = rtn::Stereo::new(sgn1, sgn2);
///
///         // However, we must manually advance them.
///         let res = stereo.get();
///         signal.advance();
///         res
///     }
/// )
/// .expect(pointillism::IO_ERROR);
/// ```
///
/// The next example rewrites our previous code in terms of [`Cell`]. If our wrappers had non-zero
/// cost, this would most likely give a noticeable improvement in performance.
///
/// ```
/// # use pointillism::prelude::*;
/// // The original signal.
/// let signal = gen::Loop::new(crv::Sin, unt::Freq::from_raw_default(unt::RawFreq::A3));
/// let cell = rtn::Cell::new(signal);
///
/// // Thanks to `Ref`, we're able to re-use our signal.
/// // And thanks to `Cell`, we only need to define our mix once.
/// let sgn1 = eff::PwMapSgn::inf_clip(rtn::Ref::new(&cell));
/// let sgn2 = eff::PwMapSgn::cubic(rtn::Ref::new(&cell));
/// let stereo = rtn::Stereo::new(sgn1, sgn2);
///
/// pointillism::create(
///     "examples/routing_cell.wav",
///     unt::Time::from_sec_default(3.0), unt::SampleRate::default(),
///     |_| {
///         // The `advance` method here uses interior mutability.
///         let res = stereo.get();
///         cell.advance();
///         res
///     }
/// )
/// .expect(pointillism::IO_ERROR);
/// ```
pub struct Ref<'a, S: Signal>(pub &'a S);

impl<'a, S: Signal> Ref<'a, S> {
    /// Initializes a new [`Ref`].
    pub const fn new(sgn: &'a S) -> Self {
        Self(sgn)
    }
}

impl<'a, S: Signal> Signal for Ref<'a, S> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.0.get()
    }
}

impl<'a, S: Done> Done for Ref<'a, S> {
    fn is_done(&self) -> bool {
        self.0.is_done()
    }
}

/// A wrapper around [`UnsafeCell`], which allows us to reference a signal that might be modified.
///
/// For safety reasons, we don't allow access to the pointer `&mut S`. Instead, the signal must be
/// read using the [`Signal`] methods, and modified using [`Cell::modify`].
pub struct Cell<S: Signal>(UnsafeCell<S>);

impl<S: Signal> Cell<S> {
    /// Initializes a new [`Cell`].
    pub const fn new(sgn: S) -> Self {
        Self(UnsafeCell::new(sgn))
    }

    /// Modify the signal through the function.
    ///
    /// Although this should be safe, note that it's subject to the same pitfalls as the nightly
    /// [`Cell::update`](https://github.com/rust-lang/rust/issues/50186#issuecomment-593233850).
    /// Particularly, you should avoid nested calls.
    pub fn modify<F: FnMut(&mut S)>(&self, mut func: F) {
        // Safety: within this scope, this is an exclusive reference.
        func(unsafe { &mut *self.0.get() });
    }
}

impl<S: SignalMut> Cell<S> {
    /// Advances the signal. Note that this only requires `&self`.
    pub fn advance(&self) {
        self.modify(SignalMut::advance);
    }

    /// Gets the next sample and advances the state of the signal. Note that this only requires
    /// `&self`.
    pub fn next(&self) -> S::Sample {
        self.advance();
        self._get()
    }
}

/// If we own `&mut Cell<S>`, it must be the only reference.
impl<S: Signal> AsMut<S> for Cell<S> {
    fn as_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }
}

impl<S: Signal> Signal for Cell<S> {
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        // Safety: within this scope, this is an exclusive reference.
        (unsafe { &*self.0.get() })._get()
    }
}

impl<S: Done> Done for Cell<S> {
    fn is_done(&self) -> bool {
        // Safety: within this scope, this is an exclusive reference.
        (unsafe { &*self.0.get() }).is_done()
    }
}
