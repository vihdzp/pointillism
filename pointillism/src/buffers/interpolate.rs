//! Implements functions for interpolating between samples.
//!
//! The main trait defined in this file is [`Interpolate`]. See the docs there for more information.

use crate::prelude::*;

/// Linearly interpolates two samples `x0` and `x1`.
pub fn linear<S: smp::SampleBase>(x0: S, x1: S, t: unt::Val) -> S {
    let t = t.inner();
    x0 * (1.0 - t) + x1 * t
}

/// Interpolates cubically between `x1` and `x2`. Makes use of the previous sample `x0` and the next
/// sample `x3`.
///
/// Adapted from <https://stackoverflow.com/a/1126113/12419072>.
pub fn cubic<S: smp::SampleBase>(x0: S, x1: S, x2: S, x3: S, t: unt::Val) -> S {
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
pub fn hermite<S: smp::SampleBase>(x0: S, x1: S, x2: S, x3: S, t: unt::Val) -> S {
    let t = t.inner();
    let diff = x1 - x2;
    let c1 = x2 - x0;
    let c3 = x3 - x0 + diff * 3.0;
    let c2 = -(diff * 2.0 + c1 + c3);

    ((c3 * t + c2) * t + c1) * t * 0.5 + x1
}

/// A trait for a ring buffer, which is used in order to apply some interpolation algorithm.
///
/// By interpolation, we mean a process of creating new [`Samples`](Sample) between two others. We
/// refer to the first of these samples as the "current" sample, and the second as the "next"
/// sample.
///
/// Interpolation is particularly relevant for time [`Stretching`](Stretch).
pub trait Interpolate:
    map::Map<Input = unt::Val, Output = <Self::Buf as buf::Buffer>::Item> + buf::Ring + Sized
{
    /// How many samples ahead of the current one must be loaded?
    const LOOK_AHEAD: u8;
    /// The size of the buffer.
    const SIZE: usize;
    /// An empty buffer.
    const EMPTY: Self;

    /// Initializes a buffer from a signal.
    ///
    /// This will advance the signal once for the current frame, and once for every
    /// [`Self::LOOK_AHEAD`] frame.
    fn init<S: SignalMut<Sample = <Self::Buf as buf::Buffer>::Item>>(sgn: &mut S) -> Self {
        let mut inter = Self::EMPTY;
        inter.push_many(sgn, Self::LOOK_AHEAD as usize + 1);
        inter
    }
}

/// Boilerplate code for the [`Ring`] impls of the interpolation buffers.
macro_rules! ring_boilerplate {
    () => {
        fn buffer(&self) -> &Self::Buf {
            self.0.buffer()
        }

        fn buffer_mut(&mut self) -> &mut Self::Buf {
            self.0.buffer_mut()
        }

        fn get(&self, index: usize) -> <Self::Buf as Buffer>::Item {
            self.0.get(index)
        }

        fn get_mut(&mut self, index: usize) -> &mut <Self::Buf as Buffer>::Item {
            self.0.get_mut(index)
        }

        fn push(&mut self, sample: A) {
            self.0.push(sample)
        }

        fn push_many<T: SignalMut<Sample = A>>(&mut self, sgn: &mut T, count: usize) {
            self.0.push_many(sgn, count)
        }
    };
}

/// An auxiliary function for casting `[A; N]` into `buf::Shift<buf::Stc<A, N>>`.
const fn shift_from<A: smp::Audio, const N: usize>(array: [A; N]) -> buf::Shift<buf::Stc<A, N>> {
    buf::Shift::new(buf::Stc::from_data(array))
}

/// A buffer for drop-sample [interpolation](Interpolate).
///
/// Drop-sample interpolation simply consists on taking the previously read sample. This is terrible
/// for audio fidelity, but can create some interesting bit-crush effects.
#[derive(Clone, Copy, Debug, Default)]
pub struct Drop<A: smp::Audio>(pub buf::ring::Shift<buf::Stc<A, 1>>);

impl<A: smp::Audio> Drop<A> {
    /// Initializes a new buffer for [`Drop`] interpolation.
    pub const fn new(sample: A) -> Self {
        Self(shift_from([sample]))
    }

    /// The zero buffer.
    pub const fn zero() -> Self {
        Self::new(A::ZERO)
    }

    /// The backing array.
    pub const fn array(&self) -> [A; 1] {
        self.0.inner().data
    }

    /// The stored sample.
    pub const fn sample(&self) -> A {
        self.array()[0]
    }
}

impl<A: smp::Audio> map::Map for Drop<A> {
    type Input = unt::Val;
    type Output = A;

    fn eval(&self, _: unt::Val) -> A {
        self.0.fst()
    }
}

impl<A: smp::Audio> buf::Ring for Drop<A> {
    type Buf = buf::Stc<A, 1>;
    ring_boilerplate!();
}

impl<A: smp::Audio> Interpolate for Drop<A> {
    const LOOK_AHEAD: u8 = 0;
    const SIZE: usize = 1;
    const EMPTY: Self = Self::zero();
}

/// A buffer for linear [interpolation](Interpolate).
///
/// Linear interpolation consists of drawing a straight line between consecutive samples. Although
/// better than [`Drop`] interpolation, both [`Cubic`] and [`Hermite`] interpolation will generally
/// give "cleaner" results.
#[derive(Clone, Copy, Debug, Default)]
pub struct Linear<A: smp::Audio>(pub buf::ring::Shift<buf::Stc<A, 2>>);

impl<A: smp::Audio> Linear<A> {
    /// Initializes a new buffer for [`Linear`] interpolation.
    pub const fn new(cur: A, next: A) -> Self {
        Self(shift_from([cur, next]))
    }

    /// The zero buffer.
    pub const fn zero() -> Self {
        Self::new(A::ZERO, A::ZERO)
    }

    /// The backing array.
    pub const fn array(&self) -> [A; 2] {
        self.0.inner().data
    }
}

impl<A: smp::Audio> map::Map for Linear<A> {
    type Input = unt::Val;
    type Output = A;

    fn eval(&self, t: unt::Val) -> A {
        let arr = self.array();
        linear(arr[0], arr[1], t)
    }
}

impl<A: smp::Audio> buf::Ring for Linear<A> {
    type Buf = buf::Stc<A, 2>;
    ring_boilerplate!();
}

impl<A: smp::Audio> Interpolate for Linear<A> {
    const LOOK_AHEAD: u8 = 1;
    const SIZE: usize = 2;
    const EMPTY: Self = Self::zero();
}

/// A buffer for cubic [interpolation](Interpolate).
///
/// Cubic interpolation uses the cubic [Lagrange
/// polynomial](https://en.wikipedia.org/wiki/Lagrange_polynomial) for the previous, current, next,
/// and next next samples. This will often yield good results, along with [`Hermite`] interpolation.
pub struct Cubic<A: smp::Audio>(pub buf::ring::Shift<buf::Stc<A, 4>>);

impl<A: smp::Audio> Cubic<A> {
    /// Initializes a new buffer for [`Cubic`] interpolation.
    pub const fn new(x0: A, x1: A, x2: A, x3: A) -> Self {
        Self(shift_from([x0, x1, x2, x3]))
    }

    /// The zero buffer.
    pub const fn zero() -> Self {
        Self::new(A::ZERO, A::ZERO, A::ZERO, A::ZERO)
    }

    /// The backing array.
    pub const fn array(&self) -> [A; 4] {
        self.0.inner().data
    }
}

impl<A: smp::Audio> map::Map for Cubic<A> {
    type Input = unt::Val;
    type Output = A;

    fn eval(&self, t: unt::Val) -> A {
        let arr = self.array();
        cubic(arr[0], arr[1], arr[2], arr[3], t)
    }
}

impl<A: smp::Audio> buf::Ring for Cubic<A> {
    type Buf = buf::Stc<A, 4>;
    ring_boilerplate!();
}

impl<A: smp::Audio> Interpolate for Cubic<A> {
    const LOOK_AHEAD: u8 = 2;
    const SIZE: usize = 4;
    const EMPTY: Self = Self::zero();
}

/// A buffer for (cubic) Hermite [interpolation](Interpolate).
///
/// Hermite interpolation uses a [Catmull-Rom
/// spline](https://en.wikipedia.org/wiki/Catmull–Rom_spline) (a special case of the cubic Hermite
/// spline) for interpolation. This will often yield good results, along with [`Cubic`]
/// interpolation.
pub struct Hermite<A: smp::Audio>(pub buf::ring::Shift<buf::Stc<A, 4>>);

impl<A: smp::Audio> Hermite<A> {
    /// Initializes a new buffer for [`Hermite`] interpolation.
    pub const fn new(x0: A, x1: A, x2: A, x3: A) -> Self {
        Self(shift_from([x0, x1, x2, x3]))
    }

    /// The zero buffer.
    pub const fn zero() -> Self {
        Self::new(A::ZERO, A::ZERO, A::ZERO, A::ZERO)
    }

    /// The backing array.
    pub const fn array(&self) -> [A; 4] {
        self.0.inner().data
    }
}

impl<A: smp::Audio> map::Map for Hermite<A> {
    type Input = unt::Val;
    type Output = A;

    fn eval(&self, t: unt::Val) -> A {
        let arr = self.array();
        hermite(arr[0], arr[1], arr[2], arr[3], t)
    }
}

impl<A: smp::Audio> buf::Ring for Hermite<A> {
    type Buf = buf::Stc<A, 4>;
    ring_boilerplate!();
}

impl<A: smp::Audio> Interpolate for Hermite<A> {
    const LOOK_AHEAD: u8 = 2;
    const SIZE: usize = 4;
    const EMPTY: Self = Self::zero();
}

/// Samples a [`SignalMut`] and time-stretches it. Both pitch and speed will be modified.
pub struct Stretch<S: SignalMut, I: Interpolate>
where
    I::Buf: buf::BufferMut<Item = S::Sample>,
{
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

impl<S: SignalMut, I: Interpolate> Stretch<S, I>
where
    I::Buf: buf::BufferMut<Item = S::Sample>,
{
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

impl<S: SignalMut> DropStretch<S>
where
    S::Sample: smp::Audio,
{
    /// Initializes a new [`DropStretch`].
    pub fn new_drop(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

/// A [`Stretch`] using [`Linear`] interpolation.
pub type LinearStretch<S> = Stretch<S, Linear<<S as Signal>::Sample>>;

impl<S: SignalMut> LinearStretch<S>
where
    S::Sample: smp::Audio,
{
    /// Initializes a new [`LinearStretch`].
    pub fn new_linear(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

/// A [`Stretch`] using [`Cubic`] interpolation.
pub type CubicStretch<S> = Stretch<S, Cubic<<S as Signal>::Sample>>;

impl<S: SignalMut> CubicStretch<S>
where
    S::Sample: smp::Audio,
{
    /// Initializes a new [`CubicStretch`].
    pub fn new_cubic(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

/// A [`Stretch`] using [`Hermite`] interpolation.
pub type HermiteStretch<S> = Stretch<S, Hermite<<S as Signal>::Sample>>;

impl<S: SignalMut> HermiteStretch<S>
where
    S::Sample: smp::Audio,
{
    /// Initializes a new [`HermiteStretch`].
    pub fn new_hermite(sgn: S, factor: f64) -> Self {
        Self::new(sgn, factor)
    }
}

impl<S: SignalMut, I: Interpolate> Signal for Stretch<S, I>
where
    I::Buf: buf::BufferMut<Item = S::Sample>,
{
    type Sample = S::Sample;

    fn get(&self) -> S::Sample {
        self.inter.eval(self.val)
    }
}

impl<S: SignalMut, I: Interpolate> SignalMut for Stretch<S, I>
where
    I::Buf: buf::BufferMut<Item = S::Sample>,
{
    fn advance(&mut self) {
        // The next position to read.
        let pos = self.val.inner() + self.factor;

        // The integer and fractional part of this position.
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let count = pos.floor() as usize;
        let val = unt::Val::fract(pos);

        self.inter.push_many(&mut self.sgn, count);
        self.val = val;
    }

    fn retrigger(&mut self) {
        self.sgn.retrigger();
        self.inter = I::init(self.sgn_mut());
    }
}
