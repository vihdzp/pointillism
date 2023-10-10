use crate::prelude::*;

pub mod design;

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
/// Define the complex function
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

    /// Initializes new [`Coefficients`], which are there normalized.
    ///
    /// ## Panics
    ///
    /// You must guarantee `V = U + 1`!
    #[must_use]
    pub fn new<const V: usize>(mut input: [f64; T], feedback: [f64; V]) -> Self {
        assert_eq!(T, U + 1, "input element mismatch");
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

/// [`Coefficients`] for a biquadratic (order 2) filter.
pub type Biquad = Coefficients<3, 2>;

/// Shifts the elements of an array by one position, adds a new one.
///
/// This is quite inefficient for larger arrays, but should be fine for small filters and gives a
/// simple implementation.
fn shift<T: Copy>(array: &mut [T], val: T) {
    if !array.is_empty() {
        for i in (0..(array.len() - 1)).rev() {
            array[i + 1] = array[i];
        }
        array[0] = val;
    }
}

/// A filter defined by a given difference equation, determined by some [`Coefficients`].
///
/// At least for the moment being, this uses a Direct Form 1 architecture.
pub struct Filter<S: smp::Sample, const T: usize, const U: usize> {
    /// The coefficients which determine the difference equation.
    pub coefficients: Coefficients<T, U>,

    /// Previous inputs to the filter.
    prev_inputs: [S; T],
    /// Previous outputs to the filter.
    prev_outputs: [S; U],
}

impl<S: smp::Sample, const T: usize, const U: usize> Filter<S, T, U> {
    /// Initializes a filter with given preconditions.
    pub const fn new_prev(
        coefficients: Coefficients<T, U>,
        prev_inputs: [S; T],
        prev_outputs: [S; U],
    ) -> Self {
        Self {
            coefficients,
            prev_inputs,
            prev_outputs,
        }
    }

    /// Initializes a new filter.
    #[must_use]
    pub const fn new(coefficients: Coefficients<T, U>) -> Self {
        Self::new_prev(coefficients, [S::ZERO; T], [S::ZERO; U])
    }

    /// A reference to the input coefficients.
    pub const fn input(&self) -> &[f64; T] {
        &self.coefficients.input
    }

    /// A reference to the feedback coefficients.
    pub const fn feedback(&self) -> &[f64; U] {
        &self.coefficients.feedback
    }

    /// Takes in a new input, returns a new output.
    ///
    /// The previous inputs and outputs are shifted every time this function is called, so this is
    /// only efficient for low-order filters.
    pub fn eval(&mut self, input: S) -> S {
        shift(&mut self.prev_inputs, input);

        // Direct Form 1.
        let output = (0..T)
            .map(|i| self.prev_inputs[i] * self.input()[i])
            .sum::<S>()
            - (0..U)
                .map(|i| self.prev_outputs[i] * self.feedback()[i])
                .sum::<S>();

        shift(&mut self.prev_outputs, output);
        output
    }

    /// Gets the last output value.
    pub const fn get(&self) -> S {
        self.prev_outputs[0]
    }

    /// Resets the previous values to zero.
    pub fn retrigger(&mut self) {
        self.prev_inputs = [S::ZERO; T];
        self.prev_outputs = [S::ZERO; U];
    }
}

/// Filters a [`Signal`] through a [`Filter`].
pub struct Filtered<S: Signal, const T: usize, const U: usize> {
    /// The filtered signal.
    pub sgn: S,

    /// The filter employed.
    pub filter: Filter<S::Sample, T, U>,
}

impl<S: Signal, const T: usize, const U: usize> Filtered<S, T, U> {
    /// Initializes a [`Filtered`] signal from given preconditions.
    pub const fn new_prev(sgn: S, filter: Filter<S::Sample, T, U>) -> Self {
        Self { sgn, filter }
    }

    /// Initializes a [`Filtered`] signal given coefficients for the filter.
    pub const fn new(sgn: S, coefficients: Coefficients<T, U>) -> Self {
        Self::new_prev(sgn, Filter::new(coefficients))
    }

    /// Returns the current filter coefficients.
    pub const fn coefficients(&self) -> Coefficients<T, U> {
        self.filter.coefficients
    }

    /// Returns a reference to the filter coefficients.
    pub fn coefficients_mut(&mut self) -> &mut Coefficients<T, U> {
        &mut self.filter.coefficients
    }
}

impl<S: Signal, const T: usize, const U: usize> Signal for Filtered<S, T, U> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.filter.get()
    }
}

impl<S: SignalMut, const T: usize, const U: usize> SignalMut for Filtered<S, T, U> {
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
