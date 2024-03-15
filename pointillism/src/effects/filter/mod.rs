//! Defines filters and their coefficients.

use crate::prelude::*;

mod coefficients;
mod design;
pub use coefficients::*;
pub use design::*;

/// A trait for a filter's function.
///
/// A `FilterMap` returns an audio sample, given ring buffers for the previous inputs and outputs.
/// These might need to have a minimum length in order to avoid panicking.
pub trait FilterMap {
    /// Evaluates the function, given the previous inputs and outputs.
    fn eval<A: Audio, I: Ring, O: Ring>(&self, inputs: &I, outputs: &O) -> A
    where
        I::Buf: buf::BufferMut<Item = A>,
        O::Buf: buf::BufferMut<Item = A>;
}

/// In its most general form, a filter is defined by its previous inputs, its previous outputs, and
/// a function that maps these to a new output. Traditionally, this function would take the form of
/// a difference equation [`DiffEq`],
///
/// The inputs and outputs can be stored in any kind of ring buffer. These include [`buf::Shift`]
/// and [`buf::Circ`], where the former is preferred for very small buffers, while the latter is
/// preferred otherwise. You may also use [`buf::EmptyRing`] if you want to ignore the
/// inputs/outputs, at no cost.
pub struct Filter<A: Audio, I: Ring, O: Ring, F: FilterMap>
where
    I::Buf: buf::BufferMut<Item = A>,
    O::Buf: buf::BufferMut<Item = A>,
{
    /// The filter map.
    pub func: F,
    /// Previous inputs to the filter.
    inputs: I,
    /// Previous outputs to the filter.
    outputs: O,
}

impl<A: Audio, I: Ring, O: Ring, F: FilterMap> Filter<A, I, O, F>
where
    I::Buf: buf::BufferMut<Item = A>,
    O::Buf: buf::BufferMut<Item = A>,
{
    /// Initializes a filter with given preconditions.
    pub const fn new_prev(func: F, inputs: I, outputs: O) -> Self {
        Self {
            func,
            inputs,
            outputs,
        }
    }

    /// Takes in a new input, returns a new output.
    pub fn eval(&mut self, input: A) -> A {
        self.inputs.push(input);
        let output = self.func.eval(&self.inputs, &self.outputs);
        self.outputs.push(output);
        output
    }

    /// A reference to the previous input values.
    pub const fn inputs(&self) -> &I {
        &self.inputs
    }

    /// A reference to the previous output values.
    pub const fn outputs(&self) -> &O {
        &self.outputs
    }

    /// Gets the last output value.
    pub fn get(&self) -> A {
        self.outputs.fst()
    }

    /// Resets the previous values to zero.
    pub fn retrigger(&mut self) {
        self.inputs.clear();
        self.outputs.clear();
    }
}

/// Filters a [`Signal`] through a [`Filter`].
///
/// Note that the implementation of [`Done`] assumes that the filtered signal stops right after the
/// original. This isn't exactly accurate, even for the simplest filters, but it should be
/// approximately so in practice.
pub struct Filtered<S: Signal, I: Ring, O: Ring, F: FilterMap>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    /// The filtered signal.
    pub sgn: S,
    /// The filter employed.
    pub filter: Filter<S::Sample, I, O, F>,
}

impl<S: Signal, I: Ring, O: Ring, F: FilterMap> Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    /// Initializes a [`Filtered`] signal.
    pub const fn new(sgn: S, filter: Filter<S::Sample, I, O, F>) -> Self {
        Self { sgn, filter }
    }

    /// Returns the current filter function.
    pub const fn func(&self) -> &F {
        &self.filter.func
    }

    /// Returns a mutable reference to the current filter function.
    pub fn func_mut(&mut self) -> &mut F {
        &mut self.filter.func
    }
}

impl<S: Signal, I: Ring, O: Ring, F: FilterMap> Signal for Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.filter.get()
    }
}

impl<S: SignalMut, I: Ring, O: Ring, F: FilterMap> SignalMut for Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    fn advance(&mut self) {
        self.next();
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.filter.retrigger();
    }

    fn next(&mut self) -> S::Sample {
        self.filter.eval(self.sgn.next())
    }
}

impl<S: Base, I: Ring, O: Ring, F: FilterMap> Base for Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    type Base = S::Base;

    fn base(&self) -> &Self::Base {
        self.sgn.base()
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self.sgn.base_mut()
    }
}

impl<S: Stop, I: Ring, O: Ring, F: FilterMap> Stop for Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    fn stop(&mut self) {
        self.sgn.stop();
    }
}

impl<S: Panic, I: Ring, O: Ring, F: FilterMap> Panic for Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    fn panic(&mut self) {
        self.sgn.panic();
        self.filter.inputs.clear();
        self.filter.outputs.clear();
    }
}

impl<S: Done, I: Ring, O: Ring, F: FilterMap> Done for Filtered<S, I, O, F>
where
    S::Sample: Audio,
    I::Buf: BufferMut<Item = S::Sample>,
    O::Buf: BufferMut<Item = S::Sample>,
{
    fn is_done(&self) -> bool {
        self.sgn.is_done()
    }
}

/// Aliases for [`Filtered::func`] and [`Filtered::func_mut`].
impl<S: Signal, I: Ring, O: Ring, T: Coefficients, U: Coefficients> Filtered<S, I, O, DiffEq<T, U>>
where
    S::Sample: Audio,
    I::Buf: BufferMut<Item = S::Sample>,
    O::Buf: BufferMut<Item = S::Sample>,
{
    /// Returns the coefficients of the filter.
    pub fn coefs(&self) -> &DiffEq<T, U> {
        self.func()
    }

    /// Returns a mutable reference to the coefficients of the filter.
    pub fn coefs_mut(&mut self) -> &mut DiffEq<T, U> {
        self.func_mut()
    }
}

/// A low order filter defined by its [`Coefficients`]. **This is not the same as a low-pass!**
///
/// This is recommended only for simple filters like biquads, as it makes use of a [`buf::Shift`]
/// buffer. Higher order filters can be implemented in more than one way, and you'll probably want
/// to build your [`Filter`] manually.
pub type LoFilter<A, const T: usize, const U: usize> =
    Filter<A, buf::Shift<buf::Stc<A, T>>, buf::Shift<buf::Stc<A, U>>, LoDiffEq<T, U>>;

impl<A: Audio, const T: usize, const U: usize> LoFilter<A, T, U> {
    /// Initializes a [`LoFilter`] from its coefficients.
    #[must_use]
    pub const fn new_coefs(coefs: LoDiffEq<T, U>) -> Self {
        Self::new_prev(
            coefs,
            buf::Shift::new(buf::Stc::new()),
            buf::Shift::new(buf::Stc::new()),
        )
    }
}

/// Filters a signal through a [`LoFilter`]. **This is not the same as a low-pass!**
pub type LoFiltered<S, const T: usize, const U: usize> = Filtered<
    S,
    buf::Shift<buf::Stc<<S as Signal>::Sample, T>>,
    buf::Shift<buf::Stc<<S as Signal>::Sample, U>>,
    LoDiffEq<T, U>,
>;

impl<S: Signal, const T: usize, const U: usize> LoFiltered<S, T, U>
where
    S::Sample: Audio,
{
    /// Initializes a [`LoFilter`] from its coefficients.
    pub const fn new_coefs(sgn: S, coefs: LoDiffEq<T, U>) -> Self {
        Self::new(sgn, LoFilter::new_coefs(coefs))
    }
}
