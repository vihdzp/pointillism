//! Structures for panning an audio signal.

use std::marker::PhantomData;

use crate::prelude::*;

/// Represents the way in which gain correlates with panning angle.
pub trait Law: Copy + Default {
    /// Initializes a new struct with the specified angle.
    fn new(angle: f64) -> Self;

    /// Panning angle, between `0.0` and `1.0`.
    ///
    /// Hard left is `0.0`, center is `0.5`, hard right is `1.0`.
    fn angle(&self) -> f64;

    /// Returns a mutable reference to the panning angle.
    fn angle_mut(&mut self) -> &mut f64;

    /// Returns the left and right gains for a given angle.
    fn gain(&self) -> (f64, f64);
}

/// Linear panning.
///
/// The left and right channel gains scale linearly with the angle.
#[derive(Clone, Copy, Debug)]
pub struct Linear {
    /// Panning angle. See [`Law::angle`] for more info.
    pub angle: f64,
}

impl Default for Linear {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

/// Initializes the fields `new`, `angle`, `angle_mut`.
macro_rules! pan_boilerplate {
    () => {
        fn new(angle: f64) -> Self {
            Self { angle }
        }

        fn angle(&self) -> f64 {
            self.angle
        }

        fn angle_mut(&mut self) -> &mut f64 {
            &mut self.angle
        }
    };
}

/// Gain formula for a linear panning law.
pub fn linear_gain(angle: f64) -> (f64, f64) {
    (1.0 - angle, angle)
}

impl Law for Linear {
    pan_boilerplate!();

    fn gain(&self) -> (f64, f64) {
        linear_gain(self.angle)
    }
}

/// Constant power panning.
///
/// The left and right channel gains are the cosine and sine of the angle.
#[derive(Clone, Copy, Debug)]
pub struct Power {
    /// Panning angle. See [`Law::angle`] for more info.
    pub angle: f64,
}

impl Default for Power {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

/// Gain formula for a power panning law.
pub fn power_gain(angle: f64) -> (f64, f64) {
    let (r, l) = (std::f64::consts::FRAC_PI_2 * angle).sin_cos();
    (l, r)
}

impl Law for Power {
    pan_boilerplate!();

    fn gain(&self) -> (f64, f64) {
        power_gain(self.angle)
    }
}

/// -4.5 dB panning.
///
/// This takes the geometric mean of the gains from the [`Linear`] and
/// [`Power`].
#[derive(Clone, Copy, Debug)]
pub struct Mixed {
    /// Panning angle. See [`Law::angle`] for more info.
    pub angle: f64,
}

impl Default for Mixed {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

/// Gain formula for a mixed panning law.
pub fn mixed_gain(angle: f64) -> (f64, f64) {
    let linear = linear_gain(angle);
    let power = power_gain(angle);
    ((linear.0 * power.0).sqrt(), (linear.1 * power.1).sqrt())
}

impl Law for Mixed {
    pan_boilerplate!();

    fn gain(&self) -> (f64, f64) {
        mixed_gain(self.angle)
    }
}

/// A wrapper for a pan [`Law`] which converts it into a [`Map`].
#[derive(Clone, Copy, Debug)]
pub struct Wrapper<A: Audio, P: Law> {
    /// Dummy variable.
    phantom: PhantomData<A>,

    /// Inner pan law.
    pub pan_law: P,
}

impl<A: Audio, P: Law> Wrapper<A, P> {
    /// Initializes a new [`Wrapper`].
    pub const fn new(pan_law: P) -> Self {
        Self {
            phantom: PhantomData,
            pan_law,
        }
    }
}

impl<A: Audio, P: Law> Map for Wrapper<A, P> {
    type Input = A;
    type Output = Stereo;

    fn eval(&self, sample: A) -> Stereo {
        let Stereo(sl, sr) = sample.duplicate();
        let (gl, gr) = self.pan_law.gain();
        Stereo(sl * gl, sr * gr)
    }
}

/// Applies a pan effect, using the specified pan [`Law`].
pub struct Panner<S: Signal, P: Law>
where
    S::Sample: Audio,
{
    /// Inner data.
    inner: MapSgn<S, Wrapper<S::Sample, P>>,
}

impl<S: Signal, P: Law> Panner<S, P>
where
    S::Sample: Audio,
{
    /// Initializes a new [`Panner`] for a given signal and pan law.
    pub const fn new_pan(sgn: S, pan_law: P) -> Self {
        Self {
            inner: MapSgn::new(sgn, Wrapper::new(pan_law)),
        }
    }

    /// Initializes a new [`Panner`] for a given signal and angle.
    pub fn new(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, P::new(angle))
    }

    /// Returns a reference to the panned signal.
    pub const fn sgn(&self) -> &S {
        self.inner.sgn()
    }

    /// Returns a mutable reference to the panned signal.
    pub fn sgn_mut(&mut self) -> &mut S {
        self.inner.sgn_mut()
    }

    /// Returns the current panning angle.
    pub fn angle(&self) -> f64 {
        self.inner.map().pan_law.angle()
    }

    /// Returns a mutable reference to the current panning angle.
    pub fn angle_mut(&mut self) -> &mut f64 {
        self.inner.map_mut().pan_law.angle_mut()
    }
}

impl<S: Signal, P: Law> Signal for Panner<S, P>
where
    S::Sample: Audio,
{
    type Sample = Stereo;

    fn get(&self) -> Stereo {
        self.inner.get()
    }

    fn advance(&mut self) {
        self.inner.advance();
    }

    fn retrigger(&mut self) {
        self.inner.retrigger();
    }
}

impl<S: Frequency, P: Law> Frequency for Panner<S, P>
where
    S::Sample: Audio,
{
    fn freq(&self) -> Freq {
        self.inner.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.inner.freq_mut()
    }
}

impl<S: Base, P: Law> Base for Panner<S, P>
where
    S::Sample: Audio,
{
    type Base = S::Base;

    fn base(&self) -> &Self::Base {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut Self::Base {
        self.inner.base_mut()
    }
}

impl<S: Done, P: Law> Done for Panner<S, P>
where
    S::Sample: Audio,
{
    fn is_done(&self) -> bool {
        self.inner.is_done()
    }
}

impl<S: Stop, P: Law> Stop for Panner<S, P>
where
    S::Sample: Audio,
{
    fn stop(&mut self) {
        self.inner.stop();
    }
}

impl<S: Signal> Panner<S, Linear>
where
    S::Sample: Audio,
{
    /// Initializes a [`Linear`] panner with the specified angle.
    pub const fn linear(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, Linear { angle })
    }
}

impl<S: Signal> Panner<S, Power>
where
    S::Sample: Audio,
{
    /// Initializes a [`Power`] panner with the specified angle.
    pub const fn power(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, Power { angle })
    }
}

impl<S: Signal> Panner<S, Mixed>
where
    S::Sample: Audio,
{
    /// Initializes a [`Mixed`] panner with the specified angle.
    pub const fn mixed(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, Mixed { angle })
    }
}
