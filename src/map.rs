//! Defines the traits [`Map`] and [`Mut`].
//!
//! These traits serve two main purposes:
//!
//! - Implementing custom curves, that can be played back as samples via
//!   [`OnceGen`](crate::prelude::OnceGen) or [`LoopGen`](crate::prelude::LoopGen). See the
//!   [`crate::generators`] module for more info.
//!
//! - Create signals that modify others, either sample-wise via [`MapSgn`](crate::prelude::MapSgn),
//!   or by directly tweaking parameters via [`MutSgn`](crate::prelude::MutSgn) or
//!   [`ModSgn`](crate::prelude::ModSgn).
//!
//! In many cases, one can use a Rust function, wrapped in an [`FnWrapper`] struct. However, in
//! cases where one wants control over this function, or to create multiple instances of it, one is
//! encouraged to implement these traits for their own custom structs.

use std::marker::PhantomData;

use crate::generators::Sample;

/// An abstract trait for a structure representing a function `X → Y`.
///
/// Due to orphan rules, this trait can't be implemented directly for Rust functions. Instead, you
/// must wrap your function in an [`FnWrapper`].
pub trait Map {
    /// Input type for the map.
    type Input;

    /// Output type for the map.
    type Output;

    /// Evaluates the function.
    fn eval(&self, x: Self::Input) -> Self::Output;
}

/// An abstract trait for a structure representing a function taking `&mut X` and `Y`.
///
/// Due to orphan rules, this trait can't be implemented directly for Rust functions. Instead, you
/// must wrap your function in an [`FnWrapper`].
pub trait Mut<X, Y> {
    /// Modifies `x` according to `y`.
    fn modify(&mut self, x: &mut X, y: Y);
}

/// A wrapper for a Rust function which converts it into a [`Map`] or [`Mut`].
///
/// Unfortunately, it may be necessary to explicitly write down the types of the arguments to the
/// function.
#[derive(Clone, Copy, Debug)]
pub struct FnWrapper<X, Y, F> {
    /// Dummy value.
    phantom: PhantomData<(X, Y)>,

    /// Wrapped function.
    pub func: F,
}

impl<X, Y, F> FnWrapper<X, Y, F> {
    /// Wraps a function in an [`FnWrapper`].
    pub const fn new(func: F) -> Self {
        Self {
            phantom: PhantomData,
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

impl<X> Map for Id<X> {
    type Input = X;
    type Output = X;

    fn eval(&self, x: X) -> X {
        x
    }
}

/// The zero function.
#[derive(Clone, Copy, Debug)]
pub struct Zero<X, S: Sample> {
    /// Dummy value.
    phantom: PhantomData<(X, S)>,
}

impl<X, S: Sample> Zero<X, S> {
    /// Initializes the zero function.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<X, S: Sample> Default for Zero<X, S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<X, S: Sample> Map for Zero<X, S> {
    type Input = X;
    type Output = S;

    fn eval(&self, _: X) -> S {
        S::ZERO
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
    pub const fn new(inner: F, outer: G) -> Self {
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
