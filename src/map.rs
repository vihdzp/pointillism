//! Defines the traits [`Map`] and [`Mut`].
//!
//! These traits serve two main purposes:
//!
//! - Implementing custom curves, either for envelopes via
//! [`CurveEnv`](crate::prelude::CurveEnv), or waveforms via
//! [`CurveGen`](crate::prelude::CurveGen). See also the
//! [`crate::generators::curves`] module for more info.
//! - Create signals that modify others, either sample-wise via
//! [`MapSgn`](crate::prelude::MapSgn), or by directly tweaking
//! parameters via [`MutSgn`](crate::prelude::MutSgn).
//!
//! In many cases, one can use a Rust function, wrapped in an [`FnWrapper`]
//! struct. However, in cases where one wants control over this function, or to
//! create multiple instances of it, one is encouraged to implement these traits
//! for their own custom structs.

use std::marker::PhantomData;

/// An abstract trait for a structure representing a function `X → Y`.
///
/// Due to orphan rules, this trait can't be implemented for Rust functions. In
/// order to use it in this case, wrap your function in [`FnWrapper`].
pub trait Map {
    /// Input type for the map.
    type Input;

    /// Output type for the map.
    type Output;

    /// Evaluates the function.
    fn eval(&self, x: Self::Input) -> Self::Output;
}

/// An abstract trait for a structure representing a function taking `&mut X`
/// and `Y`.
///
/// Due to orphan rules, this trait can't be implemented for Rust functions. In
/// order to use it in this case, wrap your function in [`FnWrapper`].
pub trait Mut<X, Y> {
    /// Modifies `x` according to `y`.
    fn modify(&mut self, x: &mut X, y: Y);
}

/// A wrapper for a Rust function which converts it into a [`Map`] or
/// [`Mut`].
///
/// It may be necessary to explicitly write down the types of the arguments to
/// the function.
#[derive(Clone, Copy, Debug)]
pub struct FnWrapper<X, Y, F> {
    /// Dummy variable.
    phantom_x: PhantomData<X>,

    /// Dummy variable.
    phantom_y: PhantomData<Y>,

    /// Wrapped function.
    pub func: F,
}

impl<X, Y, F> FnWrapper<X, Y, F> {
    /// Wraps a function in an [`FnWrapper`].
    pub const fn new(func: F) -> Self {
        Self {
            phantom_x: PhantomData,
            phantom_y: PhantomData,
            func,
        }
    }
}

impl<X, Y, F: Fn(X) -> Y> Map for FnWrapper<X, Y, F> {
    type Input = X;
    type Output = Y;

    fn eval(&self, x: X) -> Y {
        (self.func)(x)
    }
}

impl<X, Y, F: FnMut(&mut X, Y)> Mut<X, Y> for FnWrapper<X, Y, F> {
    fn modify(&mut self, x: &mut X, y: Y) {
        (self.func)(x, y);
    }
}

/// The identity function.
#[derive(Clone, Copy, Debug)]
pub struct Id<X> {
    /// Dummy variable.
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
        Self {
            phantom: PhantomData,
        }
    }
}

impl<X> Map for Id<X> {
    type Input = X;
    type Output = X;

    fn eval(&self, x: X) -> X {
        x
    }
}

/// A constant function `X → Y`.
#[derive(Clone, Copy, Debug, Default)]
pub struct Const<X, Y: Clone> {
    /// The constant value attained by the function.
    pub val: Y,

    /// Dummy variable.
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

impl<X, Y: Clone> Map for Const<X, Y> {
    type Input = X;
    type Output = Y;

    fn eval(&self, _: X) -> Y {
        self.val.clone()
    }
}

/// Function composition g ⚬ f.
#[derive(Clone, Copy, Debug, Default)]
pub struct Comp<F: Map, G: Map<Input = F::Output>> {
    /// The inner function.
    pub inner: F,

    /// The outer function.
    pub outer: G,
}

impl<F: Map, G: Map<Input = F::Output>> Comp<F, G> {
    /// Composes two functions.
    pub const fn new_generic(inner: F, outer: G) -> Self {
        Self { inner, outer }
    }
}

impl<F: Map, G: Map<Input = F::Output>> Map for Comp<F, G> {
    type Input = F::Input;
    type Output = G::Output;

    fn eval(&self, x: F::Input) -> G::Output {
        self.outer.eval(self.inner.eval(x))
    }
}
