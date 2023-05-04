//! Declares the [`Unison`] struct.
//!
//! This can be used in order to more efficiently play multiple copies of a base signal.

use crate::prelude::*;

/// A bundled [`Val`] for a curve, and an [`Interval`]. By bundling data like this, we save on
/// allocations.
#[derive(Clone, Copy, Debug, Default)]
pub struct ValInter {
    /// How far along some curve we are.
    pub val: Val,

    /// An interval.
    pub interval: Interval,
}

impl ValInter {
    /// Initializes a new [`ValFreq`].
    #[must_use]
    pub const fn new(val: Val, interval: Interval) -> Self {
        Self { val, interval }
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
    pub fn detune_curve(map: C, base: Freq, num: u8, detune: Interval) -> Self {
        // The lowest frequency detune.
        let low_freq_mul = if num % 2 == 0 {
            detune.sqrt() * detune.powi(i32::from(num) / 2 - 1)
        } else {
            detune.powi(i32::from(num) / 2)
        };
        let mut freq_mul = low_freq_mul;

        Self::new_curve(
            map,
            base,
            (0..num).map(|_| {
                let res = freq_mul;
                freq_mul *= detune;
                res
            }),
        )
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
    pub fn detune(map: C, base: Freq, num: u8, detune: Interval) -> Self {
        Self::detune_curve(CurvePlayer::new(map), base, num, detune)
    }
}
