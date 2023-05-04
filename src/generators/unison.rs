//! Declares the [`Unison`] struct.
//!
//! This can be used in order to more efficiently play multiple copies of a base signal.

use rand::{thread_rng, Rng};

use crate::prelude::*;

/// A bundled [`Val`] for a curve, and an [`Interval`].
///
/// These two are needed in order to play curves in unison. By bundling data like this, we save on
/// allocations. More importantly, we guarantee that there aren't mismatches between the number of
/// these values.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValInter {
    /// How far along some curve we are.
    pub val: Val,

    /// An interval.
    pub interval: Interval,
}

impl ValInter {
    /// Initializes a new [`ValInter`].
    #[must_use]
    pub const fn new(val: Val, interval: Interval) -> Self {
        Self { val, interval }
    }
}

/// An iterator that returns the detuning intervals for a given detune amount.
pub struct DetuneIter {
    /// The amount to detune.
    detune: Interval,

    /// The number of intervals to output.
    num: u8,

    /// Stores the next interval to output.
    output: Interval,

    /// How many intervals have been output.
    index: u8,
}

impl DetuneIter {
    /// Initializes a new [`Detune`] object.
    #[must_use]
    pub fn new(detune: Interval, num: u8) -> Self {
        // We initialize the output to the highest frequency detune.
        let output = if num % 2 == 0 {
            detune.sqrt() * detune.powi(i32::from(num) / 2 - 1)
        } else {
            detune.powi(i32::from(num) / 2)
        };

        Self {
            detune,
            num,
            output,
            index: 0,
        }
    }

    /// The number of values yet to be output.
    #[must_use]
    pub fn size(&self) -> u8 {
        self.num - self.index
    }
}

impl Iterator for DetuneIter {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.num {
            None
        } else {
            let res = self.output;
            self.output /= self.detune;
            self.index += 1;
            Some(res)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.size() as usize;
        (size, Some(size))
    }
}

/// Plays multiple copies of a curve in unison.
pub struct UnisonCurve<C: Map<Input = Val>>
where
    C::Output: Sample,
{
    /// The curve being played.
    map: C,

    /// The base frequency.
    base: Freq,

    /// The values and intervals from the base frequency for each curve.
    val_inters: Vec<ValInter>,
}

impl<C: Map<Input = Val>> UnisonCurve<C>
where
    C::Output: Sample,
{
    /// Initializes a new [`UnisonCurve`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers, with the
    /// given initial phases.
    pub const fn new_curve_phases(map: C, base: Freq, val_inters: Vec<ValInter>) -> Self {
        Self {
            map,
            base,
            val_inters,
        }
    }

    /// Initializes a new [`UnisonCurve`].
    ///
    /// This will play multiple copies of a curve at the specified intervals from the base
    /// frequency.
    pub fn new_curve<I: Iterator<Item = Interval>>(map: C, base: Freq, intervals: I) -> Self {
        Self {
            map,
            base,
            val_inters: intervals.map(|x| ValInter::new(Val::ZERO, x)).collect(),
        }
    }

    /// Plays copies of a curve, centered at a certain base frequency, spaced out by a given
    /// interval.
    pub fn detune_curve(map: C, base: Freq, detune: Interval, num: u8) -> Self {
        Self::new_curve(map, base, DetuneIter::new(detune, num))
    }

    /// The number of copies of the signal that play.
    pub fn len(&self) -> usize {
        self.val_inters.len()
    }

    /// Whether there are no signals to be played.
    pub fn is_empty(&self) -> bool {
        self.val_inters.is_empty()
    }

    /// A reference to the curve being played.
    pub const fn map(&self) -> &C {
        &self.map
    }

    /// A mutable reference to the curve being played.
    pub fn map_mut(&mut self) -> &mut C {
        &mut self.map
    }

    /// Returns an iterator over the intervals for the different curves.
    pub fn intervals<'a>(&'a self) -> impl Iterator<Item = Interval> + 'a {
        self.val_inters.iter().map(|vf| vf.interval)
    }

    /// Returns an iterator over the mutable references to the intervals for the different curves.
    pub fn intervals_mut(&mut self) -> impl Iterator<Item = &mut Interval> {
        self.val_inters.iter_mut().map(|vf| &mut vf.interval)
    }

    /// Returns an iterator over the values for the different curves.
    pub fn val<'a>(&'a self) -> impl Iterator<Item = Val> + 'a {
        self.val_inters.iter().map(|vf| vf.val)
    }

    /// Returns an iterator over the mutable references to the values for the different curves.
    pub fn val_mut(&mut self) -> impl Iterator<Item = &mut Val> {
        self.val_inters.iter_mut().map(|vf| &mut vf.val)
    }

    /// Randomizes the phases.
    ///
    /// This can help if you're getting a lot of interference between the different curves.
    pub fn randomize_phases(&mut self) {
        for val in self.val_mut() {
            *val = thread_rng().gen();
        }
    }
}

impl<C: Map<Input = Val>> Signal for UnisonCurve<C>
where
    C::Output: Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.val_inters
            .iter()
            .map(|vf| self.map().eval(vf.val))
            .sum()
    }

    fn advance(&mut self) {
        for vf in &mut self.val_inters {
            vf.val.advance_freq(self.base * vf.interval);
        }
    }

    fn retrigger(&mut self) {
        for vf in &mut self.val_inters {
            vf.val.retrigger();
        }
    }
}

impl<C: Map<Input = Val>> Base for UnisonCurve<C>
where
    C::Output: Sample,
{
    impl_base!();
}

impl<C: Map<Input = Val>> Frequency for UnisonCurve<C>
where
    C::Output: Sample,
{
    fn freq(&self) -> Freq {
        self.base
    }

    fn freq_mut(&mut self) -> &mut Freq {
        &mut self.base
    }
}

/// Plays a given curve by reading its output as values of a given sample type.
pub type Unison<S, C> = UnisonCurve<CurvePlayer<S, C>>;

impl<S: Sample, C: Map<Input = Val, Output = f64>> Unison<S, C> {
    /// Initializes a new [`Unison`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers, with the
    /// given initial phases.
    pub fn new_phases<I: Iterator<Item = f64>>(
        map: C,
        base: Freq,
        val_freqs: Vec<ValInter>,
    ) -> Self {
        Self::new_curve_phases(CurvePlayer::new(map), base, val_freqs)
    }

    /// Initializes a new [`Unison`].
    ///
    /// This will play multiple copies of a curve at the specified intervals from the base
    /// frequency.
    pub fn new<I: Iterator<Item = Interval>>(map: C, base: Freq, intervals: I) -> Self {
        Self::new_curve(CurvePlayer::new(map), base, intervals)
    }

    /// Plays copies of a curve, centered at a certain base frequency, spaced out by a given
    /// interval.
    pub fn detune(map: C, base: Freq, detune: Interval, num: u8) -> Self {
        Self::detune_curve(CurvePlayer::new(map), base, detune, num)
    }
}

/// The function that applies a detune effect to a [`UnisonCurve`].
///
/// A value of `0.0` means no detune, while `1.0` means an octave detuning.
#[derive(Clone, Copy, Debug, Default)]
pub struct Detune;

impl<C: Map<Input = Val>> Mut<UnisonCurve<C>, Env> for Detune
where
    C::Output: Sample,
{
    fn modify(&mut self, sgn: &mut UnisonCurve<C>, val: Env) {
        let num = sgn.len() as u8;

        for (int, detune) in sgn
            .intervals_mut()
            .zip(DetuneIter::new(Interval::note(12.0 * val.0), num))
        {
            *int = detune;
        }
    }
}

pub type DetuneCurveSgn<C, E> = MutSgn<UnisonCurve<C>, E, Detune>;

pub type DetuneSgn<S, C, E> = MutSgn<Unison<S, C>, E, Detune>;

impl<C: Map<Input = Val>, E: Signal<Sample = Env>> DetuneCurveSgn<C, E>
where
    C::Output: Sample,
{
    pub fn new_detune_curve(map: C, base: Freq, num: u8, env: E) -> Self {
        Self::new(
            UnisonCurve::new_curve(
                map,
                base,
                std::iter::repeat(Interval::UNISON).take(num as usize),
            ),
            env,
            Detune,
        )
    }
}

impl<S: Sample, C: Map<Input = Val, Output = f64>, E: Signal<Sample = Env>> DetuneSgn<S, C, E> {
    pub fn new_detune(map: C, base: Freq, num: u8, env: E) -> Self {
        Self::new_detune_curve(CurvePlayer::new(map), base, num, env)
    }
}
