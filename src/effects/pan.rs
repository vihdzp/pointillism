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
#[must_use]
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
#[must_use]
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
#[must_use]
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
pub struct Wrapper<A: smp::Audio, P: Law> {
    /// Dummy value.
    phantom: PhantomData<A>,

    /// Inner pan law.
    pub pan_law: P,
}

impl<A: smp::Audio, P: Law> Wrapper<A, P> {
    /// Initializes a new [`Wrapper`].
    pub const fn new(pan_law: P) -> Self {
        Self {
            phantom: PhantomData,
            pan_law,
        }
    }
}

impl<A: smp::Audio, P: Law> map::Map for Wrapper<A, P> {
    type Input = A;
    type Output = smp::Stereo;

    fn eval(&self, sample: A) -> smp::Stereo {
        let smp::Stereo(sl, sr) = sample.duplicate();
        let (gl, gr) = self.pan_law.gain();
        smp::Stereo(sl * gl, sr * gr)
    }
}

/// Applies a pan effect, using the specified pan [`Law`].
pub type Panner<S, P> = eff::MapSgn<S, Wrapper<<S as Signal>::Sample, P>>;

impl<S: Signal, P: Law> Panner<S, P>
where
    S::Sample: smp::Audio,
{
    /// Initializes a new [`Panner`] for a given signal and pan law.
    pub const fn new_pan_law(sgn: S, pan_law: P) -> Self {
        eff::MapSgn::new(sgn, Wrapper::new(pan_law))
    }

    /// Initializes a new [`Panner`] for a given signal and angle.
    pub fn new_pan(sgn: S, angle: f64) -> Self {
        Self::new_pan_law(sgn, P::new(angle))
    }

    /// Returns the current panning angle.
    pub fn angle(&self) -> f64 {
        self.map().pan_law.angle()
    }

    /// Returns a mutable reference to the current panning angle.
    pub fn angle_mut(&mut self) -> &mut f64 {
        self.map_mut().pan_law.angle_mut()
    }
}

/// A [`Linear`] panner.
pub type LinearPanner<S> = Panner<S, Linear>;

impl<S: Signal> LinearPanner<S>
where
    S::Sample: smp::Audio,
{
    /// Initializes a [`LinearPanner`] with the specified angle.
    pub const fn linear(sgn: S, angle: f64) -> Self {
        Self::new_pan_law(sgn, Linear { angle })
    }
}

/// A [`Power`] panner.
pub type PowerPanner<S> = Panner<S, Power>;

impl<S: Signal> Panner<S, Power>
where
    S::Sample: smp::Audio,
{
    /// Initializes a [`PowerPanner`] with the specified angle.
    pub const fn power(sgn: S, angle: f64) -> Self {
        Self::new_pan_law(sgn, Power { angle })
    }
}

/// A [`Mixed`] panner.
pub type MixedPanner<S> = Panner<S, Mixed>;

impl<S: Signal> Panner<S, Mixed>
where
    S::Sample: smp::Audio,
{
    /// Initializes a [`MixedPanner`] with the specified angle.
    pub const fn mixed(sgn: S, angle: f64) -> Self {
        Self::new_pan_law(sgn, Mixed { angle })
    }
}
