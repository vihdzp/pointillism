//! Implements functions for interpolating between samples.
//!
//! The main trait defined in this file is [`Interpolate`]. See the docs there for more information.
//!
//! ## Todo
//!
//! Replace the buffers by more general ring buffers.

use crate::prelude::*;

/// Linearly interpolates two samples `x0` and `x1`.
pub fn linear<S: smp::SampleLike>(x0: S, x1: S, t: unt::Val) -> S {
    let t = t.inner();
    x0 * (1.0 - t) + x1 * t
}

/// Interpolates cubically between `x1` and `x2`. Makes use of the previous sample `x0` and the next
/// sample `x3`.
///
/// Adapted from <https://stackoverflow.com/a/1126113/12419072>.
pub fn cubic<S: smp::SampleLike>(x0: S, x1: S, x2: S, x3: S, t: unt::Val) -> S {
    let t = t.inner();
    let a0 = x3 - x2 - x0 + x1;
    let a1 = x0 - x1 - a0;
    let a2 = x2 - x0;
    let a3 = x1;

    ((a0 * t + a1) * t + a2) * t + a3
}

/// Applies Hermite interpolation between `x1` and `x2`. Makes use of the previous sample `x0` and
/// the next sample `x3`.
///
/// Adapted from <https://stackoverflow.com/a/72122178/12419072>.
pub fn hermite<S: smp::SampleLike>(x0: S, x1: S, x2: S, x3: S, t: unt::Val) -> S {
    let t = t.inner();
    let diff = x1 - x2;
    let c1 = x2 - x0;
    let c3 = x3 - x0 + diff * 3.0;
    let c2 = -(diff * 2.0 + c1 + c3);

    ((c3 * t + c2) * t + c1) * t * 0.5 + x1
}

/// A trait for a buffer, which is used in order to apply some interpolation algorithm.
///
/// By interpolation, we mean a process of creating new [`Samples`](Sample) between two others. We
/// refer to the first of these samples as the "current" sample, and the second as the "next"
/// sample.
///
/// Interpolation is particularly relevant for time [`Stretching`](Stretch).
///
/// ## Implementing the trait
///
/// The current design doesn't really lend itself to generality, unless you want to implement more
/// "experimental" interpolation methods that work with small buffers. As such, we don't recommend
/// implementing this trait for custom types. See also the to-do section below.
///
/// ## Todo
///
/// Having a rolling window probably works well enough for the sample sizes we currently use. For
/// anything larger, it might be more computationally efficient to make larger buffers. That way, we
/// can write from the signal "in one go", instead of having to constantly read values and shift
/// them.
pub trait Interpolate: map::Map<Input = unt::Val, Output = Self::Sample> + Sized {
    /// The type of sample stored in the buffer.
    type Sample:  smp::Sample;

    /// How many samples ahead of the current one must be loaded?
    const LOOK_AHEAD: u8;
    /// The size of the buffer.
    const SIZE: usize;
    /// An empty buffer.
    const EMPTY: Self;

    /// Loads a new sample into the buffer, phasing out the first one.
    fn load(&mut self, sample: Self::Sample);

    /// Loads `count` samples from a signal, phasing out the old ones.
    fn load_many<S: SignalMut<Sample = Self::Sample>>(&mut self, sgn: &mut S, count: usize);

    /// Initializes a buffer from a signal.
    ///
    /// This will advance the signal once for the current frame, and once for every
    /// [`Self::LOOK_AHEAD`] frame.
    fn init<S: SignalMut<Sample = Self::Sample>>(sgn: &mut S) -> Self {
        let mut inter = Self::EMPTY;
        inter.load_many(sgn, Self::LOOK_AHEAD as usize + 1);
        inter
    }
}

/// A buffer for drop-sample [interpolation](Interpolate).
///
/// Drop-sample interpolation simply consists on taking the previously read sample. This is terrible
/// for audio fidelity, but can create some interesting bit-crush effects.
#[derive(Clone, Copy, Debug, Default)]
pub struct Drop<S: smp::Sample>(pub S);

impl<S: smp::Sample> Drop<S> {
    /// Initializes a new buffer for [`Drop`] interpolation.
    pub const fn new(sample: S) -> Self {
        Self(sample)
    }
}

impl<S: smp::Sample> map::Map for Drop<S> {
    type Input = unt::Val;
    type Output = S;

    fn eval(&self, _: unt::Val) -> S {
        self.0
    }
}

impl<S: smp::Sample> Interpolate for Drop<S> {
    type Sample = S;

    const LOOK_AHEAD: u8 = 0;
    const SIZE: usize = 1;
    const EMPTY: Self = Self::new(S::ZERO);

    fn load(&mut self, sample: S) {
        self.0 = sample;
    }

    fn load_many<T: SignalMut<Sample = Self::Sample>>(&mut self, sgn: &mut T, count: usize) {
        if count > 0 {
            for _ in 0..(count - 1) {
                sgn.advance();
            }
            self.0 = sgn.next();
        }
    }
}

/// A buffer for linear [interpolation](Interpolate).
///
/// Linear interpolation consists of drawing a straight line between consecutive samples. Although
/// better than [`Drop`] interpolation, both [`Cubic`] and [`Hermite`] interpolation will generally
/// give "cleaner" results.
pub struct Linear<S: smp::Sample> {
    /// The current sample.
    pub cur: S,
    /// The next sample.
    pub next: S,
}

impl<S: smp::Sample> Linear<S> {
    /// Initializes a new buffer for [`Linear`] interpolation.
    pub const fn new(cur: S, next: S) -> Self {
        Self { cur, next }
    }
}

impl<S: smp::Sample> map::Map for Linear<S> {
    type Input = unt::Val;
    type Output = S;

    fn eval(&self, t: unt::Val) -> S {
        linear(self.cur, self.next, t)
    }
}

impl<S: smp::Sample> Interpolate for Linear<S> {
    type Sample = S;

    const LOOK_AHEAD: u8 = 1;
    const SIZE: usize = 2;
    const EMPTY: Self = Self::new(S::ZERO, S::ZERO);

    fn load(&mut self, sample: S) {
        self.cur = self.next;
        self.next = sample;
    }

    fn load_many<T: SignalMut<Sample = Self::Sample>>(&mut self, sgn: &mut T, count: usize) {
        match count {
            0 => {}
            1 => self.load(sgn.next()),
            count => {
                for _ in 0..(count - 2) {
                    sgn.advance();
                }
                self.cur = sgn.next();
                self.next = sgn.next();
            }
        }
    }
}

/// An implementation of the `load` method for buffers of arbitrary size.
///
/// We currently only use this for `N = 4`.
fn load_gen<S: Copy, const N: usize>(buf: &mut [S; N], sample: S) {
    for i in 0..(N - 1) {
        buf[i] = buf[i + 1];
    }
    buf[N - 1] = sample;
}

/// An implementation of the `load_many` method for buffers of size 4.
fn load_many_gen<S: SignalMut>(buf: &mut [S::Sample; 4], sgn: &mut S, count: usize) {
    match count {
        0 => {}
        1 => load_gen(buf, sgn.next()),

        // The hope here is that the compiler can optimize away redundant writes.
        2 => {
            for _ in 0..2 {
                load_gen(buf, sgn.next());
            }
        }
        3 => {
            for _ in 0..3 {
                load_gen(buf, sgn.next());
            }
        }

        count => {
            for _ in 0..(count - 4) {
                sgn.advance();
            }

            // `as_mut` is not needed, but stops `rust-analyzer` from (erroneously) complaining.
            for i in 0..4 {
                buf.as_mut()[i] = sgn.next();
            }
        }
    }
}

/// A buffer for cubic [interpolation](Interpolate).
///
/// Cubic interpolation uses the cubic [Lagrange
/// polynomial](https://en.wikipedia.org/wiki/Lagrange_polynomial) for the previous, current, next,
/// and next next samples. This will often yield good results, along with [`Hermite`] interpolation.
pub struct Cubic<S: smp::Sample>(pub [S; 4]);

impl<S: smp::Sample> Cubic<S> {
    /// Initializes a new buffer for [`Cubic`] interpolation.
    pub const fn new(x0: S, x1: S, x2: S, x3: S) -> Self {
        Self([x0, x1, x2, x3])
    }
}

impl<S: smp::Sample> map::Map for Cubic<S> {
    type Input = unt::Val;
    type Output = S;

    fn eval(&self, t: unt::Val) -> S {
        cubic(self.0[0], self.0[1], self.0[2], self.0[3], t)
    }
}

impl<S: smp::Sample> Interpolate for Cubic<S> {
    type Sample = S;

    const LOOK_AHEAD: u8 = 2;
    const SIZE: usize = 4;
    const EMPTY: Self = Self([S::ZERO; 4]);

    fn load(&mut self, sample: S) {
        load_gen(&mut self.0, sample);
    }

    fn load_many<T: SignalMut<Sample = Self::Sample>>(&mut self, sgn: &mut T, count: usize) {
        load_many_gen(&mut self.0, sgn, count);
    }
}

/// A buffer for (cubic) Hermite [interpolation](Interpolate).
///
/// Hermite interpolation uses a [Catmull-Rom
/// spline](https://en.wikipedia.org/wiki/Catmull–Rom_spline) (a special case of the cubic Hermite
/// spline) for interpolation. This will often yield good results, along with [`Cubic`]
/// interpolation.
pub struct Hermite<S: smp::Sample>(pub [S; 4]);

impl<S: smp::Sample> Hermite<S> {
    /// Initializes a new buffer for [`Hermite`] interpolation.
    pub const fn new(x0: S, x1: S, x2: S, x3: S) -> Self {
        Self([x0, x1, x2, x3])
    }
}

impl<S: smp::Sample> map::Map for Hermite<S> {
    type Input = unt::Val;
    type Output = S;

    fn eval(&self, t: unt::Val) -> S {
        hermite(self.0[0], self.0[1], self.0[2], self.0[3], t)
    }
}

impl<S: smp::Sample> Interpolate for Hermite<S> {
    type Sample = S;

    const LOOK_AHEAD: u8 = 2;
    const SIZE: usize = 4;
    const EMPTY: Self = Self([S::ZERO; 4]);

    fn load(&mut self, sample: S) {
        load_gen(&mut self.0, sample);
    }

    fn load_many<T: SignalMut<Sample = Self::Sample>>(&mut self, sgn: &mut T, count: usize) {
        load_many_gen(&mut self.0, sgn, count);
    }
}

/// Samples a [`SignalMut`] and time-stretches it. Both pitch and speed will be modified.
pub struct Stretch<S: SignalMut, I: Interpolate<Sample = S::Sample>> {
    /// The signal being sampled.
    sgn: S,

    /// The time stretching factor.
    ///
    /// For instance, `2.0` means that the signal will be played twice as fast.
    factor: f64,

    /// The interpolation buffer.
    inter: I,

    /// The fractional position between this sample and the next.
    val: unt::Val,
}

impl<S: SignalMut, I: Interpolate<Sample = S::Sample>> Stretch<S, I> {
    /// Initializes a new [`Stretch`].
    ///
    /// If you call this function, you'll have to write down the interpolation mode explicitly.
    /// Consider instead calling one of [`Self::new_drop`], [`Self::new_linear`],
    /// [`Self::new_cubic`] or [`Self::new_hermite`].
    pub fn new(mut sgn: S, factor: f64) -> Self {
        Self {
            inter: I::init(&mut sgn),
            sgn,
            factor,
            val: unt::Val::ZERO,
        }
    }

    /// Returns a reference to the original signal.
    ///
    /// Note that the signal might have been read a few samples ahead of what's currently playing,
    /// depending on the interpolation method.
    pub const fn sgn(&self) -> &S {
        &self.sgn
    }

    /// Returns a mutable reference to the original signal.
    ///
    /// Note that the signal might have been read a few samples ahead of what's currently playing,
    /// depending on the interpolation method.
    pub fn sgn_mut(&mut self) -> &mut S {
        &mut self.sgn
    }

    /// Returns the time-stretching factor.
    pub fn factor(&self) -> f64 {
        self.factor
    }

    /// The fractional position between the current and next samples.
    pub const fn val(&self) -> unt::Val {
        self.val
    }
}

/// A [`Stretch`] using [`Drop`] interpolation.
pub type DropStretch<S> = Stretch<S, Drop<<S as Signal>::Sample>>;

impl<S: SignalMut> DropStretch<S> {
    /// Initializes a new [`DropStretch`].
    pub fn new_drop(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

/// A [`Stretch`] using [`Linear`] interpolation.
pub type LinearStretch<S> = Stretch<S, Linear<<S as Signal>::Sample>>;

impl<S: SignalMut> LinearStretch<S> {
    /// Initializes a new [`LinearStretch`].
    pub fn new_linear(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

/// A [`Stretch`] using [`Cubic`] interpolation.
pub type CubicStretch<S> = Stretch<S, Cubic<<S as Signal>::Sample>>;

impl<S: SignalMut> CubicStretch<S> {
    /// Initializes a new [`CubicStretch`].
    pub fn new_cubic(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

/// A [`Stretch`] using [`Hermite`] interpolation.
pub type HermiteStretch<S> = Stretch<S, Hermite<<S as Signal>::Sample>>;

impl<S: SignalMut> HermiteStretch<S> {
    /// Initializes a new [`HermiteStretch`].
    pub fn new_hermite(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

impl<S: SignalMut, I: Interpolate<Sample = S::Sample>> Signal for Stretch<S, I> {
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inter.eval(self.val)
    }
}

impl<S: SignalMut, I: Interpolate<Sample = S::Sample>> SignalMut for Stretch<S, I> {
    fn advance(&mut self) {
        // The next position to read.
        let pos = self.val.inner() + self.factor;

        // The integer and fractional part of this position.
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let count = pos.floor() as usize;
        let val = unt::Val::fract(pos);

        self.inter.load_many(&mut self.sgn, count);
        self.val = val;
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.inter = I::init(self.sgn_mut());
    }
}
