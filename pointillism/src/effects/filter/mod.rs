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
    fn eval<A: smp::Audio, I: buf::Ring, O: buf::Ring>(&self, inputs: &I, outputs: &O) -> A
    where
        I::Buf: buf::BufferMut<Item = A>,
        O::Buf: buf::BufferMut<Item = A>;
}

/// In its most general form, a filter is defined by its previous inputs, its previous outputs, and
/// a function that maps these to a new output. Traditionally, this function would take the form of
/// a difference equation [`flt::DifEq`],
///
/// The inputs and outputs can be stored in any kind of ring buffer. These include [`buf::Shift`]
/// and [`buf::Circ`], where the former is preferred for very small buffers, while the latter is
/// preferred otherwise. You may also use [`buf::EmptyRing`] if you want to ignore the
/// inputs/outputs, at no cost.
pub struct Filter<A: smp::Audio, I: buf::Ring, O: buf::Ring, F: FilterMap>
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

impl<A: smp::Audio, I: buf::Ring, O: buf::Ring, F: FilterMap> Filter<A, I, O, F>
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
pub struct Filtered<S: Signal, I: buf::Ring, O: buf::Ring, F: FilterMap>
where
    S::Sample: smp::Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    /// The filtered signal.
    pub sgn: S,
    /// The filter employed.
    pub filter: Filter<S::Sample, I, O, F>,
}

impl<S: Signal, I: buf::Ring, O: buf::Ring, F: FilterMap> Filtered<S, I, O, F>
where
    S::Sample: smp::Audio,
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

impl<S: Signal, I: buf::Ring, O: buf::Ring, F: FilterMap> Signal for Filtered<S, I, O, F>
where
    S::Sample: smp::Audio,
    I::Buf: buf::BufferMut<Item = S::Sample>,
    O::Buf: buf::BufferMut<Item = S::Sample>,
{
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.filter.get()
    }
}

impl<S: SignalMut, I: buf::Ring, O: buf::Ring, F: FilterMap> SignalMut for Filtered<S, I, O, F>
where
    S::Sample: smp::Audio,
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

/// A low order filter defined by its [`Coefficients`]. **This is not the same as a low-pass!**
///
/// This is recommended only for simple filters like biquads, as it makes use of a [`buf::Shift`]
/// buffer. Higher orders, assuming the coefficients are dense, are better served by [`HiFilter`].
pub type LoFilter<A, const T: usize, const U: usize> =
    Filter<A, buf::Shift<buf::Stc<A, T>>, buf::Shift<buf::Stc<A, U>>, LoDiffEq<T, U>>;

impl<A: smp::Audio, const T: usize, const U: usize> LoFilter<A, T, U> {
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
    S::Sample: smp::Audio,
{
    /// Initializes a [`LoFilter`] from its coefficients.
    pub const fn new_coefs(sgn: S, coefs: LoDiffEq<T, U>) -> Self {
        Self::new(sgn, LoFilter::new_coefs(coefs))
    }

    /// Returns the coefficients of the filter.
    pub fn coefs(&self) -> &LoDiffEq<T, U> {
        self.func()
    }

    /// Returns a mutable reference to the coefficients of the filter.
    pub fn coefs_mut(&mut self) -> &mut LoDiffEq<T, U> {
        self.func_mut()
    }
}
