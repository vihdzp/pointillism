use crate::prelude::*;
use std::marker::PhantomData;

/// The identity function.
#[derive(Clone, Copy, Debug)]
pub struct Id<X> {
    /// Dummy value.
    phantom: PhantomData<X>,
}

impl<X> Id<X> {
    /// Initializes the identity function.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<X> Default for Id<X> {
    fn default() -> Self {
        Self::new()
    }
}

impl<X> map::Map for Id<X> {
    type Input = X;
    type Output = X;

    fn eval(&self, x: X) -> X {
        x
    }
}

/// The zero function.
///
/// This ignores the input and returns the additive identity for samples.
///
/// ## Caveat
///
/// The [additive
/// identity](https://codeyarns.com/tech/2020-06-12-floating-point-identity-elements.html#gsc.tab=0)
/// for floating point numbers is `-0.0`, and not `0.0`! This function will return the former.
#[derive(Clone, Copy, Debug)]
pub struct Zero<X, S: smp::Sample> {
    /// Dummy value.
    phantom: PhantomData<(X, S)>,
}

impl<X, S: smp::Sample> Zero<X, S> {
    /// Initializes the zero function.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<X, S: smp::Sample> Default for Zero<X, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<X, S: smp::Sample> map::Map for Zero<X, S> {
    type Input = X;
    type Output = S;

    fn eval(&self, _: X) -> S {
        -S::ZERO
    }
}

/// A constant function `X → Y`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Const<X, Y: Clone> {
    /// The constant value attained by the function.
    pub val: Y,

    /// Dummy value.
    phantom: PhantomData<X>,
}

impl<X, Y: Clone> Const<X, Y> {
    /// Initializes a new constant function.
    pub const fn new(val: Y) -> Self {
        Self {
            val,
            phantom: PhantomData,
        }
    }
}

impl<X, Y: Clone>map:: Map for Const<X, Y> {
    type Input = X;
    type Output = Y;

    fn eval(&self, _: X) -> Y {
        self.val.clone()
    }
}

/// Function composition g ⚬ f.
#[derive(Clone, Copy, Debug, Default)]
pub struct Comp<F: map::Map, G: map::Map<Input = F::Output>> {
    /// The inner function.
    pub inner: F,
    /// The outer function.
    pub outer: G,
}

impl<F: map::Map, G:map:: Map<Input = F::Output>> Comp<F, G> {
    /// Composes two functions.
    pub const fn new(inner: F, outer: G) -> Self {
        Self { inner, outer }
    }
}

impl<F: map::Map, G: map::Map<Input = F::Output>> map::Map for Comp<F, G> {
    type Input = F::Input;
    type Output = G::Output;

    fn eval(&self, x: F::Input) -> G::Output {
        self.outer.eval(self.inner.eval(x))
    }
}

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Pos;

impl map::Map for Pos {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        map::pos(x)
    }
}

impl<F: map::Map<Output = f64>> map::Comp<F, Pos> {
    /// Composes a function with [`Pos`].
    pub const fn pos(f: F) -> Self {
        Self::new(f, Pos)
    }
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Sgn;

impl map::Map for Sgn {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        map::sgn(x)
    }
}

impl<F: map::Map<Output = f64>> map::Comp<F, Sgn> {
    /// Composes a function with [`Sgn`].
    pub const fn sgn(f: F) -> Self {
        Self::new(f, Sgn)
    }
}

/// Negates a floating point value.
#[derive(Clone, Copy, Debug, Default)]
pub struct Neg;

impl Neg {
    /// The negation function.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl map::Map for Neg {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        -x
    }
}

impl<F: map::Map<Output = f64>> map::Comp<F, Neg> {
    /// Composes a function with [`Neg`].
    pub const fn neg(f: F) -> Self {
        Self::new(f, Neg)
    }
}

/// A linear map `y = mx + b`.
#[derive(Clone, Copy, Debug)]
pub struct Linear {
    /// Slope of the map.
    pub slope: f64,

    /// y-intercept of the map.
    pub intercept: f64,
}

impl Linear {
    /// Initializes a new linear map.
    #[must_use]
    pub const fn new(slope: f64, intercept: f64) -> Self {
        Self { slope, intercept }
    }

    /// Initializes the linear map that rescales an interval `[init_lo, init_hi]` to another
    /// `[end_lo, end_hi]`.
    #[must_use]
    pub fn rescale(init_lo: f64, init_hi: f64, end_lo: f64, end_hi: f64) -> Self {
        let slope = (end_hi - end_lo) / (init_hi - init_lo);
        Self::new(slope, end_lo - slope * init_lo)
    }

    /// Initializes the linear map that rescales the unit interval `[0.0, 1.0]` to another `[lo,
    /// hi]`.
    #[must_use]
    pub fn rescale_unit(lo: f64, hi: f64) -> Self {
        Self::rescale(0.0, 1.0, lo, hi)
    }

    /// Initializes the linear map that rescales the interval `[-1.0, 1.0]` to another `[lo, hi]`.
    #[must_use]
    pub fn rescale_sgn(lo: f64, hi: f64) -> Self {
        Self::rescale(-1.0, 1.0, lo, hi)
    }
}

impl map::Map for Linear {
    type Input = f64;
    type Output = f64;

    fn eval(&self, x: f64) -> f64 {
        x * self.slope + self.intercept
    }
}
