//! Structures that modify volume and panning.

use crate::{
    sample::{AudioSample, Stereo},
    signal::{MapSgn, PointwiseMapSgn, Signal},
    Map, Vol,
};

/// Controls the volume of a signal.
pub type Volume<S> = PointwiseMapSgn<S, Vol>;

impl<S: Signal> Volume<S> {
    /// Initializes a new signal with a given [`Vol`].
    pub const fn new(sgn: S, vol: Vol) -> Self {
        Self::new_pointwise(sgn, vol)
    }

    /// Gain of the signal.
    pub const fn gain(&self) -> f64 {
        self.func().gain
    }

    /// Returns a mutable reference to the gain of the signal.
    pub fn gain_mut(&mut self) -> &mut f64 {
        &mut self.func_mut().gain
    }
}

/// Represents the way in which gain correlates with panning angle.
pub trait PanLaw: Copy + Default {
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
    /// Panning angle. See [`PanLaw::angle`] for more info.
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

impl PanLaw for Linear {
    pan_boilerplate!();

    fn gain(&self) -> (f64, f64) {
        (1.0 - self.angle, self.angle)
    }
}

/// Constant power panning.
///
/// The left and right channel gains are the cosine and sine of the angle.
#[derive(Clone, Copy, Debug)]
pub struct Power {
    /// Panning angle. See [`PanLaw::angle`] for more info.
    pub angle: f64,
}

impl Default for Power {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

impl PanLaw for Power {
    pan_boilerplate!();

    fn gain(&self) -> (f64, f64) {
        let angle = std::f64::consts::FRAC_PI_2 * self.angle;
        (angle.cos(), angle.sin())
    }
}

/// -4.5 dB panning.
///
/// This takes the geometric mean of the gains from the [`Linear`] and
/// [`Power`].
#[derive(Clone, Copy, Debug)]
pub struct Mixed {
    /// Panning angle. See [`PanLaw::angle`] for more info.
    pub angle: f64,
}

impl Default for Mixed {
    fn default() -> Self {
        Self { angle: 0.5 }
    }
}

impl PanLaw for Mixed {
    pan_boilerplate!();

    fn gain(&self) -> (f64, f64) {
        let linear = Linear { angle: self.angle }.gain();
        let power = Power { angle: self.angle }.gain();
        ((linear.0 * power.0).sqrt(), (linear.1 * power.1).sqrt())
    }
}

/// A wrapper for a [`PanLaw`] which converts it into a [`Map`].
#[derive(Clone, Copy, Debug)]
pub struct PanWrapper<P>(pub P);

impl<A: AudioSample, P: PanLaw> Map<A, Stereo> for PanWrapper<P> {
    fn eval(&self, sample: A) -> Stereo {
        let Stereo(sl, sr) = sample.duplicate();
        let (gl, gr) = self.0.gain();
        Stereo(sl * gl, sr * gr)
    }
}

/// Applies a pan effect, using the specified [`PanLaw`].
pub type Panner<S, P> = MapSgn<S, Stereo, PanWrapper<P>>;

impl<S: Signal, P: PanLaw> Panner<S, P>
where
    S::Sample: AudioSample,
{
    /// Initializes a new [`Panner`] for a given signal and angle.
    pub fn new_pan(sgn: S, angle: f64) -> Self {
        Self::new_generic(sgn, PanWrapper(P::new(angle)))
    }

    /// Returns the current panning angle.
    pub fn angle(&self) -> f64 {
        self.map.0.angle()
    }

    /// Returns a mutable reference to the current panning angle.
    pub fn angle_mut(&mut self) -> &mut f64 {
        self.map.0.angle_mut()
    }
}

/// Applies a pan effect, with a [`Linear`] panning law.
pub type LinearPanner<S> = Panner<S, Linear>;

impl<S: Signal> LinearPanner<S>
where
    S::Sample: AudioSample,
{
    /// Initializes a new [`LinearPanner`] for a given signal and angle.
    pub fn new(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, angle)
    }
}

/// Applies a pan effect, with a constant [`Power`] panning law.
pub type PowerPanner<S> = Panner<S, Power>;

impl<S: Signal> PowerPanner<S>
where
    S::Sample: AudioSample,
{
    /// Initializes a new [`PowerPanner`] for a given signal and angle.
    pub fn new(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, angle)
    }
}

/// Applies a pan effect, with a [`Mixed`] panning law.
pub type MixedPanner<S> = Panner<S, Mixed>;

impl<S: Signal> MixedPanner<S>
where
    S::Sample: AudioSample,
{
    /// Initializes a new [`MixedPanner`] for a given signal and angle.
    pub fn new(sgn: S, angle: f64) -> Self {
        Self::new_pan(sgn, angle)
    }
}
