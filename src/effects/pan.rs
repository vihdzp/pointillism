use crate::{
    sample::{AudioSample, Sample, Stereo},
    signal::{MapSgn, Signal},
    Map,
};

/// Represents the gain of some signal.
#[derive(Clone, Copy, Debug)]
pub struct Vol {
    /// Gain factor.
    pub gain: f64,
}

impl Vol {
    /// Initializes a new volume variable.
    pub fn new(gain: f64) -> Self {
        Self { gain }
    }

    /// Gain measured in decibels.
    pub fn new_db(db: f64) -> Self {
        Self::new(10f64.powf(db / 20.0))
    }

    /// The gain in decibels.
    pub fn db(&self) -> f64 {
        20.0 * self.gain.log10()
    }

    /// Silence.
    pub fn silence() -> Self {
        Self::new(0.0)
    }
}

impl Default for Vol {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl<S: Sample> Map<S, S> for Vol {
    fn eval(&self, x: S) -> S {
        x * self.gain
    }
}

pub type Volume<S> = MapSgn<S, <S as Signal>::Sample, Vol>;

impl<S: Signal> Volume<S> {
    pub fn new(sgn: S, gain: f64) -> Self {
        Self::new_generic(sgn, Vol { gain })
    }

    pub fn gain(&self) -> f64 {
        self.map.gain
    }

    pub fn gain_mut(&mut self) -> &mut f64 {
        &mut self.map.gain
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
