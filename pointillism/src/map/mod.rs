//! Defines the traits [`Map`] and [`Mut`].
//!
//! These traits serve two main purposes:
//!
//! - Implementing custom curves, that can be played back as samples via [`gen::Once`] or
//!   [`gen::Loop`]. See the [`gen`] module for more info.
//!
//! - Create signals that modify others, either sample-wise via [`MapSgn`](crate::eff::MapSgn), or
//!   by directly tweaking parameters via [`MutSgn`](crate::eff::MutSgn) or
//!   [`ModSgn`](crate::eff::ModSgn).
//!
//! In many cases, one can use a Rust function, wrapped in a [`Func`] struct. However, in cases
//! where one wants control over this function, or to create multiple instances of it, one is
//! encouraged to implement these traits for their own custom structs.

mod basic;
mod sample;
pub use basic::*;
pub use sample::*;

use std::marker::PhantomData;

use crate::prelude::*;

/// Rescales a value from `-1.0` to `1.0`, into a value from `0.0` to `1.0`.
#[must_use]
pub fn pos(x: f64) -> f64 {
    (x + 1.0) / 2.0
}

/// Rescales a value from `0.0` to `1.0`, into a value from `-1.0` to `1.0`.
#[must_use]
pub fn sgn(x: f64) -> f64 {
    2.0 * x - 1.0
}

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
