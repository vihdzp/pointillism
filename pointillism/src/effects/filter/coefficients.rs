//! Defines the different types for the [`Coefficients`] of a [`DiffEq`].
//!
//! These include [`Dense`] for polynomials with mostly non-zero entries, and [`Sparse`] for
//! anything else.

use crate::prelude::*;

/// A trait for the coefficients in a [`DiffEq`].
pub trait Coefficients {
    /// If this structure represents the values `a`, given the samples `x` in backwards order and a
    /// shift `i`, evaluates
    ///
    /// ```txt
    /// a[0]x[i] + a[1]x[i + 1] + ... + a[k]x[i + k] + ...
    /// ```
    ///
    /// This is used for both inputs and outputs.
    fn eval<A: Audio, B: Ring>(&self, inputs: &B) -> A
    where
        B::Buf: buf::Buffer<Item = A>;

    /// Scales all coefficients by a given factor.
    fn scale(&mut self, factor: f64);
}

/// An array of [`Coefficients`], stored in sequence from least to greatest degree.
///
/// This is recommended when most of these coefficients are non-zero.
#[derive(Clone, Default, Debug)]
pub struct Dense<T: AsRef<[f64]> + AsMut<[f64]>>(pub T);

impl<T: AsRef<[f64]> + AsMut<[f64]>> Dense<T> {
    /// Initializes a new [`Dense`] from the coefficients.
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Gets the coefficient corresponding to some degree.
    pub fn get(&self, degree: usize) -> f64 {
        self.0.as_ref().get(degree).copied().unwrap_or_default()
    }
}

impl<T: AsRef<[f64]> + AsMut<[f64]>> Coefficients for Dense<T> {
    fn eval<A: Audio, B: Ring>(&self, samples: &B) -> A
    where
        B::Buf: buf::Buffer<Item = A>,
    {
        self.0
            .as_ref()
            .iter()
            .enumerate()
            .map(|(k, &c)| samples.get(k) * c)
            .sum()
    }

    fn scale(&mut self, factor: f64) {
        for v in self.0.as_mut() {
            *v *= factor;
        }
    }
}

/// A [`Dense`] set of [`Coefficients`] backed by a `[f64; N]`.
pub type DenseStc<const N: usize> = Dense<[f64; N]>;
/// A [`Dense`] set of [`Coefficients`] backed by a `Vec<f64>`.
pub type DenseDyn = Dense<Vec<f64>>;
/// Zero coefficients.
pub type Zero = DenseStc<0>;

impl<const N: usize> DenseStc<N> {
    /// An array of zero coefficients.
    #[must_use]
    pub const fn zero() -> Self {
        Self([0.0; N])
    }
}

/// An array of [`Coefficients`], stored alongside their degrees.
///
/// This is recommended when most of these coefficients are zero.
#[derive(Clone, Default, Debug)]
pub struct Sparse<T: AsRef<[(usize, f64)]> + AsMut<[(usize, f64)]>>(pub T);

impl<T: AsRef<[(usize, f64)]> + AsMut<[(usize, f64)]>> Sparse<T> {
    /// Initializes a new [`Sparse`] from the coefficients.
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Gets the coefficient corresponding to some degree.
    ///
    /// Note that the complexity is O(n) on the size of the backing array. If you know the terms are
    /// ordered by degree, you can use [`Self::get_ordered`] instead.
    pub fn get(&self, degree: usize) -> f64 {
        match self.0.as_ref().iter().find(|&&(k, _)| k == degree) {
            Some(&(_, c)) => c,
            None => 0.0,
        }
    }

    /// Gets the coefficient corresponding to some degree.
    ///
    /// Assumes that the degrees of terms are ordered in the backing array. Uses binary searching.
    pub fn get_ordered(&self, degree: usize) -> f64 {
        let array = self.0.as_ref();
        match array.binary_search_by_key(&degree, |&(k, _)| k) {
            Ok(idx) => array[idx].1,
            Err(_) => 0.0,
        }
    }
}

impl<T: AsRef<[(usize, f64)]> + AsMut<[(usize, f64)]>> Coefficients for Sparse<T> {
    fn eval<A: Audio, B: Ring>(&self, samples: &B) -> A
    where
        B::Buf: buf::Buffer<Item = A>,
    {
        self.0
            .as_ref()
            .iter()
            .map(|&(k, c)| samples.get(k) * c)
            .sum()
    }

    fn scale(&mut self, factor: f64) {
        for (_, v) in self.0.as_mut() {
            *v *= factor;
        }
    }
}

/// A [`Sparse`] set of [`Coefficients`] backed by a `[f64; N]`.
pub type SparseStc<const N: usize> = Sparse<[f64; N]>;
/// A [`Sparse`] set of [`Coefficients`] backed by a `Vec<f64>`.
pub type SparseDyn = Sparse<Vec<f64>>;

/// Coefficients of a difference equation
///
/// ```txt
/// y[n] = b[0]x[n] + b[1]x[n - 1] + ... + b[T - 1]x[n - p]
///                 - a[1]y[n - 1] - ... - a[U - 1]y[n - q]
/// ```
///
/// The values `x[n]` are the inputs, while the values `y[n]` are the outputs. These can be used to
/// build a [`eff::flt::Filter`]. `T` is the length of `b`, while `U` is the length of `a`.
///
/// Some sources separate out a value `a[0]`, which essentially serves as a normalization factor.
/// The "real" coefficients result by dividing everything else out by `a[0]`.
///
/// For common filter designs, see [`eff::flt::Biquad`].
///
/// ## Transfer function
///
/// Define the complex [transfer function](https://en.wikipedia.org/wiki/Transfer_function):
///
/// ```txt
/// H(z) = (b[0] + b[1] * z⁻¹ + ...) / (1 + a[0] * z⁻¹ + ...)
/// ```
///
/// Suppose a [`Filter`](super::Filter) is built from these coefficients. Let `f` be the
/// [`unt::Freq`] of some signal (measured in samples<sup>–1</sup>). Let `z = exp(τi * f)`, where `τ
/// ≈ 6.28` and `i` is the imaginary unit. Then:
///
/// - `|H(z)|` is the filter gain at this frequency,
/// - `arg H(z)` is the phase shift.
#[derive(Clone, Copy, Debug, Default)]
pub struct DiffEq<T: Coefficients, U: Coefficients> {
    /// The input or feedforward coefficients `b`.
    pub input: T,
    /// The feedback coefficients `a`.
    pub feedback: U,
}

impl<T: Coefficients, U: Coefficients> DiffEq<T, U> {
    /// Initializes a new [`DiffEq`].
    ///
    /// The constant feedback coefficient is omitted.
    #[must_use]
    pub const fn new_raw(input: T, feedback: U) -> Self {
        Self { input, feedback }
    }

    /// Mutably applies an overall gain factor.
    pub fn gain(&mut self, gain: unt::Vol) {
        self.input.scale(gain.gain);
    }

    /// Applies an overall gain factor. Returns `self`.
    pub fn with_gain(mut self, gain: unt::Vol) -> Self {
        self.gain(gain);
        self
    }
}

impl<T: Coefficients> DiffEq<T, Zero> {
    /// Initializes the coefficients for a new Finite Impulse Response (FIR) filter.
    ///
    /// This just means that the feedback coefficients are all zero.
    #[must_use]
    pub const fn new_fir(input: T) -> Self {
        Self::new_raw(input, Zero::zero())
    }
}

impl<T: Coefficients, U: Coefficients> FilterMap for DiffEq<T, U> {
    fn eval<A: Audio, I: Ring, O: Ring>(&self, inputs: &I, outputs: &O) -> A
    where
        I::Buf: buf::BufferMut<Item = A>,
        O::Buf: buf::BufferMut<Item = A>,
    {
        // Direct form 1.
        self.input.eval(inputs) - self.feedback.eval(outputs)
    }
}
