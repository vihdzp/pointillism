//! Defines filters and their coefficients.

use crate::prelude::*;

use self::smp::Audio;

pub mod design;
pub use design::Biquad;

/// A trait for a filter's function.
///
/// A `FilterMap` returns an audio sample, given ring buffers for the previous inputs and outputs.
/// Often, these need to have a minimum length in order for the filter to be effective, which can be
/// specified.
pub trait FilterMap<A: smp::Audio> {
    /// Minimum length for the inputs.
    fn min_inputs(&self) -> usize;
    /// Minimum length for the outputs.
    fn min_outputs(&self) -> usize;

    /// Evaluates the function, given the previous inputs and outputs.
    fn eval<I: buf::Ring, O: buf::Ring>(&self, inputs: &I, outputs: &O) -> A
    where
        I::Buf: buf::BufferMut<Item = A>,
        O::Buf: buf::BufferMut<Item = A>;
}

/// Coefficients of a difference equation
///
/// ```txt
/// y[n] = b[0]x[n] + b[1]x[n - 1] + ... + b[T - 1]x[n - p]
///                 - a[1]y[n - 1] - ... - a[U - 1]y[n - q]
/// ```
///
/// The values `x[n]` are the inputs, while the values `y[n]` are the outputs. These can be used to
/// build a [`Filter`]. `T` is the length of `b`, while `U` is the length of `a`.
///
/// For common filter designs, see [`Biquad`].
///
/// ## Transfer function
///
/// Define the complex [transfer function](https://en.wikipedia.org/wiki/Transfer_function):
///
/// ```txt
/// H(z) = (b[0] + b[1] * z⁻¹ + ...) / (a[0] + a[1] * z⁻¹ + ...)
/// ```
///
/// Suppose a [`Filter`] is built from these coefficients. Let `f` be the [`Freq`] of some signal
/// (measured in samples<sup>–1</sup>). Let `z = exp(τi * f)`, where `τ ≈ 6.28` and `i` is the
/// imaginary unit. Then:
///
/// - `|H(z)|` is the filter gain at this frequency,
/// - `arg H(z)` is the phase shift.
#[derive(Clone, Copy, Debug)]
pub struct Coefficients<const T: usize, const U: usize> {
    /// The input or feedforward coefficients `b`.
    pub input: [f64; T],
    /// The feedback coefficients `a`.
    pub feedback: [f64; U],
}

impl<const T: usize, const U: usize> Coefficients<T, U> {
    /// Initializes new normalized [`Coefficients`].
    ///
    /// The first feedback coefficient `a0` is omitted.
    #[must_use]
    pub const fn new_normalized(input: [f64; T], feedback: [f64; U]) -> Self {
        Self { input, feedback }
    }

    /// Initializes new [`Coefficients`], which are then normalized.
    ///
    /// ## Panics
    ///
    ///
    #[must_use]
    pub fn new<const V: usize>(mut input: [f64; T], feedback: [f64; V]) -> Self {
        assert_eq!(V, U + 1, "input element mismatch");
        let a0 = feedback[0];

        // Normalize input.
        for x in &mut input {
            *x /= a0;
        }

        // Normalize feedback.
        let mut new_feedback = [0.0; U];
        for i in 0..U {
            new_feedback[i] = feedback[i + 1] / a0;
        }

        Self::new_normalized(input, new_feedback)
    }
}

impl<const T: usize> Coefficients<T, 0> {
    /// Initializes the coefficients for a new Finite Impulse Response (FIR) filter.
    ///
    /// This just means that the feedback coefficients are all zero.
    #[must_use]
    pub const fn new_fir(input: [f64; T]) -> Self {
        Self::new_normalized(input, [])
    }
}

impl<A: smp::Audio, const T: usize, const U: usize> FilterMap<A> for Coefficients<T, U> {
    fn min_inputs(&self) -> usize {
        T
    }

    fn min_outputs(&self) -> usize {
        U
    }

    fn eval<I: buf::Ring, O: buf::Ring>(&self, inputs: &I, outputs: &O) -> A
    where
        I::Buf: buf::BufferMut<Item = A>,
        O::Buf: buf::BufferMut<Item = A>,
    {
        // Direct form 1.
        (0..T).map(|i| inputs.get(i) * self.input[i]).sum::<A>()
            - (0..U).map(|i| outputs.get(i) * self.feedback[i]).sum::<A>()
    }
}

/// In its most general form, a filter is defined by its previous inputs, its previous outputs, and
/// a function that maps these to a new output. Traditionally, this function would take the form of
/// a difference equation, like that of [`Coefficients`], but we allow for versatility so that other
/// structures like [`Delays`](eff::Delay) can be implemented as a special case.
///
/// The inputs and outputs can be stored in any kind of ring buffer. These include [`buf::Shift`]
/// and [`buf::Circ`], where the former is preferred for very small buffers, while the latter is
/// preferred otherwise. You may also use [`buf::EmptyRing`] if you want to ignore the
/// inputs/outputs, at no cost.
pub struct Filter<A: smp::Audio, I: buf::Ring, O: buf::Ring, F: FilterMap<A>>
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

impl<A: smp::Audio, I: buf::Ring, O: buf::Ring, F: FilterMap<A>> Filter<A, I, O, F>
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
pub struct Filtered<S: Signal, I: buf::Ring, O: buf::Ring, F: FilterMap<S::Sample>>
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

impl<S: Signal, I: buf::Ring, O: buf::Ring, F: FilterMap<S::Sample>> Filtered<S, I, O, F>
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

impl<S: Signal, I: buf::Ring, O: buf::Ring, F: FilterMap<S::Sample>> Signal for Filtered<S, I, O, F>
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

impl<S: SignalMut, I: buf::Ring, O: buf::Ring, F: FilterMap<S::Sample>> SignalMut
    for Filtered<S, I, O, F>
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

/// A low order filter defined by its [`Coefficients`]. **This is not the same as a low-pass!**
///
/// This is recommended only for simple filters like biquads, as it makes use of a [`buf::Shift`]
/// buffer. Higher orders, assuming the coefficients are dense, are better served by [`HiFilter`].
pub type LoFilter<A, const T: usize, const U: usize> =
    Filter<A, buf::Shift<buf::Stc<A, T>>, buf::Shift<buf::Stc<A, U>>, Coefficients<T, U>>;

impl<A: smp::Audio, const T: usize, const U: usize> LoFilter<A, T, U> {
    /// Initializes a [`LoFilter`] from its coefficients.
    #[must_use]
    pub const fn new_coefs(coefs: Coefficients<T, U>) -> Self {
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
    Coefficients<T, U>,
>;

impl<S: Signal, const T: usize, const U: usize> LoFiltered<S, T, U>
where
    S::Sample: smp::Audio,
{
    /// Initializes a [`LoFilter`] from its coefficients.
    pub const fn new_coefs(sgn: S, coefs: Coefficients<T, U>) -> Self {
        Self::new(sgn, LoFilter::new_coefs(coefs))
    }

    /// Returns the coefficients of the filter.
    pub fn coefs(&self) -> &Coefficients<T, U> {
        self.func()
    }

    /// Returns a mutable reference to the coefficients of the filter.
    pub fn coefs_mut(&mut self) -> &mut Coefficients<T, U> {
        self.func_mut()
    }
}
