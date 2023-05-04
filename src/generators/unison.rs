use crate::prelude::*;

/// A bundled value for a curve, and a frequency multiplier. By bundling data like this, we save on
/// allocations.
#[derive(Clone, Copy, Debug)]
pub struct ValFreq {
    /// How far along some curve we are.
    pub val: Val,

    /// Frequency multiplier for a curve.
    pub freq_mul: f64,
}

impl ValFreq {
    pub const fn new(val: Val, freq_mul: f64) -> Self {
        Self { val, freq_mul }
    }
}

impl Default for ValFreq {
    fn default() -> Self {
        Self::new(Val::ZERO, 1.0)
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
    base_freq: Freq,

    /// The values and frequency multipliers for each curve.
    val_freqs: Vec<ValFreq>,
}

impl<C: Map<Input = Val>> UnisonCurve<C>
where
    C::Output: Sample,
{
    /// Initializes a new [`UnisonCurve`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers, with the
    /// given initial phases.
    pub const fn new_curve_phases(map: C, base: Freq, val_freqs: Vec<ValFreq>) -> Self {
        Self {
            map,
            base_freq: base,
            val_freqs,
        }
    }

    /// Initializes a new [`UnisonCurve`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers.
    pub fn new_curve<I: Iterator<Item = f64>>(map: C, base: Freq, freq_muls: I) -> Self {
        Self {
            map,
            base_freq: base,
            val_freqs: freq_muls.map(|x| ValFreq::new(Val::ZERO, x)).collect(),
        }
    }

    pub fn detune_curve(map: C, base: Freq, num: usize, detune: f64) -> Self {
        // The lowest frequency detune.
        let low_freq_mul = if num % 2 == 0 {
            detune.sqrt() * detune.powi(num as i32 / 2 - 1)
        } else {
            detune.powi(num as i32 / 2)
        };
        let mut freq_mul = low_freq_mul;

        Self::new_curve(
            map,
            base,
            (0..num).into_iter().map(|_| {
                let res = freq_mul;
                freq_mul *= detune;
                res
            }),
        )
    }

    /// The number of copies of the signal that play.
    pub fn len(&self) -> usize {
        self.val_freqs.len()
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
        self.val_freqs
            .iter()
            .map(|vf| self.map().eval(vf.val))
            .sum()
    }

    fn advance(&mut self) {
        for vf in &mut self.val_freqs {
            vf.val.advance_freq(self.base_freq * vf.freq_mul)
        }
    }

    fn retrigger(&mut self) {
        for vf in &mut self.val_freqs {
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
        self.base_freq
    }

    fn freq_mut(&mut self) -> &mut Freq {
        &mut self.base_freq
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
        val_freqs: Vec<ValFreq>,
    ) -> Self {
        Self::new_curve_phases(CurvePlayer::new(map), base, val_freqs)
    }

    /// Initializes a new [`UnisonCurve`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers.
    pub fn new<I: Iterator<Item = f64>>(map: C, base: Freq, freq_muls: I) -> Self {
        Self::new_curve(CurvePlayer::new(map), base, freq_muls)
    }

    pub fn detune(map: C, base: Freq, num: usize, detune: f64) -> Self {
        Self::detune_curve(CurvePlayer::new(map), base, num, detune)
    }
}
