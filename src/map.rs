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
//! In many cases, one can use a Rust function, wrapped in a [`Func`] struct. However, in cases
//! where one wants control over this function, or to create multiple instances of it, one is
//! encouraged to implement these traits for their own custom structs.

use std::marker::PhantomData;

use crate::prelude::*;

/// An abstract trait for a structure representing a function `X → Y`.
///
/// Due to orphan rules, this trait can't be implemented directly for Rust functions. Instead, you
/// must wrap your function in an [`Func`].
pub trait Map {
    /// Input type for the map.
    type Input;
    /// Output type for the map.
    type Output;

    /// Evaluates the function.
    fn eval(&self, x: Self::Input) -> Self::Output;
}

/// An abstract trait for a structure representing a function which modifies a [`Signal`].
///
/// Due to orphan rules, this trait can't be implemented directly for Rust functions. Instead, you
/// must wrap your function in a [`Func`].
pub trait Mut<S: Signal> {
    /// Modifies `sgn`.
    fn modify(&mut self, sgn: &mut S);
}

/// An abstract trait for a structure representing a function which modifies a [`Signal`] according
/// to an envelope.
///
/// Due to orphan rules, this trait can't be implemented directly for Rust functions. Instead, you
/// must wrap your function in a [`Func`].
pub trait Env<S: Signal> {
    /// Modifies `sgn` according to `val`.
    fn modify_env(&mut self, sgn: &mut S, val: smp::Env);
}

/// A wrapper for a Rust function which converts it into a [`Map`] or [`Mut`].
///
/// Unfortunately, it may be necessary to explicitly write down the types of the arguments to the
/// function.
#[derive(Clone, Copy, Debug)]
pub struct Func<X, Y, F> {
    /// Wrapped function.
    pub func: F,

    /// Dummy value.
    phantom: PhantomData<(X, Y)>,
}

impl<X, Y, F> Func<X, Y, F> {
    /// Wraps a function in an [`Func`].
    pub const fn new(func: F) -> Self {
        Self {
            func,
            phantom: PhantomData,
        }
    }
}

impl<X, Y, F: Fn(X) -> Y> Map for Func<X, Y, F> {
    type Input = X;
    type Output = Y;

    fn eval(&self, x: X) -> Y {
        (self.func)(x)
    }
}

impl<S: Signal, F: FnMut(&mut S, smp::Env)> Env<S> for Func<S, smp::Env, F> {
    fn modify_env(&mut self, x: &mut S, y: smp::Env) {
        (self.func)(x, y);
    }
}

impl<S: Signal, F: FnMut(&mut S)> Mut<S> for Func<S, (), F> {
    fn modify(&mut self, x: &mut S) {
        (self.func)(x);
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

impl<X, S: smp::Sample> Map for Zero<X, S> {
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

/// The function that [flips](Stereo::flip) a [`Stereo`] signal.
#[derive(Copy, Clone, Debug, Default)]
pub struct Flip;

impl Map for Flip {
    type Input = smp::Stereo;
    type Output = smp::Stereo;

    fn eval(&self, x: smp::Stereo) -> smp::Stereo {
        x.flip()
    }
}
