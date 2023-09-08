//! Declares the [`Unison`] struct.
//!
//! This can be used in order to more efficiently play multiple copies of a base signal.

use crate::prelude::*;

/// An iterator that returns the detuning intervals for a given detune amount.
pub struct DetuneIter {
    /// The interval between two successive outputs.
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
    ///
    /// The `detune` interval passed is the distance between successive outputs.
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
///
/// It is strongly recommended you don't play 256 or more curves at once. Besides practical
/// concerns, many methods don't support this.
pub struct UnisonCurve<C: Map<Input = Val>>
where
    C::Output: Sample,
{
    /// The curve being played.
    map: C,

    /// The base frequency.
    base: Freq,

    /// The values and intervals from the base frequency for each curve.
    ///
    /// These two are needed in order to play curves in unison. By bundling data like this, we save
    /// on allocations. More importantly, we guarantee that there aren't mismatches between the
    /// number of these values.
    val_inters: Vec<(Val, Interval)>,
}

impl<C: Map<Input = Val>> UnisonCurve<C>
where
    C::Output: Sample,
{
    /// Initializes a new [`UnisonCurve`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers, with the
    /// given initial phases.
    pub const fn new_curve_phases(map: C, base: Freq, val_inters: Vec<(Val, Interval)>) -> Self {
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
    pub fn new_curve<I: IntoIterator<Item = Interval>>(map: C, base: Freq, intervals: I) -> Self {
        Self {
            map,
            base,
            val_inters: intervals.into_iter().map(|x| (Val::ZERO, x)).collect(),
        }
    }

    /// Plays copies of a curve, centered at a certain base frequency, spaced out by a given
    /// interval.
    ///
    /// Assuming an interval larger than `1.0`, the curves are indexed from highest to lowest
    /// pitched.
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
    pub fn intervals(&self) -> impl Iterator<Item = Interval> + '_ {
        self.val_inters.iter().map(|&(_, interval)| interval)
    }

    /// Returns an iterator over the mutable references to the intervals for the different curves.
    pub fn intervals_mut(&mut self) -> impl Iterator<Item = &mut Interval> {
        self.val_inters.iter_mut().map(|(_, interval)| interval)
    }

    /// Returns an iterator over the values for the different curves.
    pub fn val(&self) -> impl Iterator<Item = Val> + '_ {
        self.val_inters.iter().map(|&(val, _)| val)
    }

    /// Returns an iterator over the mutable references to the values for the different curves.
    pub fn val_mut(&mut self) -> impl Iterator<Item = &mut Val> {
        self.val_inters.iter_mut().map(|(val, _)| val)
    }

    /// Returns the current output from a given curve.
    pub fn get_at(&self, index: u8) -> C::Output {
        self.map().eval(self.val_inters[index as usize].0)
    }

    /// Randomizes the phases.
    ///
    /// This can help if you're getting a lot of interference between the different curves.
    pub fn randomize_phases(&mut self) {
        for val in self.val_mut() {
            use rand::Rng;
            *val = rand::thread_rng().gen();
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
            .map(|&(val, _)| self.map().eval(val))
            .sum()
    }
}

impl<C: Map<Input = Val>> SignalMut for UnisonCurve<C>
where
    C::Output: Sample,
{
    fn advance(&mut self) {
        for (val, interval) in &mut self.val_inters {
            val.advance_freq(*interval * self.base);
        }
    }

    fn retrigger(&mut self) {
        for vf in &mut self.val_inters {
            vf.0 = Val::ZERO;
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
///
/// It is strongly recommended you don't play 256 or more curves at once. Besides practical
/// concerns, many methods don't support this.
pub type Unison<S, C> = UnisonCurve<CurvePlayer<S, C>>;

impl<S: Sample, C: Map<Input = Val, Output = f64>> Unison<S, C> {
    /// Initializes a new [`Unison`].
    ///
    /// This will play multiple copies of a curve at the specified frequency multipliers, with the
    /// given initial phases.
    pub fn new_phases<I: IntoIterator<Item = f64>>(
        map: C,
        base: Freq,
        val_inters: Vec<(Val, Interval)>,
    ) -> Self {
        Self::new_curve_phases(CurvePlayer::new(map), base, val_inters)
    }

    /// Initializes a new [`Unison`].
    ///
    /// This will play multiple copies of a curve at the specified intervals from the base
    /// frequency.
    pub fn new<I: IntoIterator<Item = Interval>>(map: C, base: Freq, intervals: I) -> Self {
        Self::new_curve(CurvePlayer::new(map), base, intervals)
    }

    /// Plays copies of a curve, centered at a certain base frequency, spaced out by a given
    /// interval.
    ///
    /// Assuming an interval larger than `1.0`, the curves are indexed from highest to lowest
    /// pitched.
    pub fn detune(map: C, base: Freq, detune: Interval, num: u8) -> Self {
        Self::detune_curve(CurvePlayer::new(map), base, detune, num)
    }
}

/// The function that applies a detune effect to a [`UnisonCurve`].
///
/// A value of `0.0` means no detune, while `1.0` means an octave detuning.
#[derive(Clone, Copy, Debug, Default)]
pub struct Detune;

impl<C: Map<Input = Val>> MutEnv<UnisonCurve<C>> for Detune
where
    C::Output: Sample,
{
    fn modify_env(&mut self, sgn: &mut UnisonCurve<C>, val: Env) {
        // Assuming you've used `[DetuneCurveSgn::new_detune_curve`], this should not result in
        // truncation.
        #[allow(clippy::cast_possible_truncation)]
        let num = sgn.len() as u8;

        for (int, detune) in sgn
            .intervals_mut()
            .zip(DetuneIter::new(Interval::note(12.0 * val.0), num))
        {
            *int = detune;
        }
    }
}

/// Detunes various copies of a curve according to an envelope.
///
/// The detune amount is between successive intervals. A value of `0.0` means no detune, while `1.0`
/// means an octave detuning.
///
/// The curves within the [`UnisonCurve`] struct are indexed from highest to lowest pitched.
pub type DetuneCurveSgn<C, E> = MutSgn<UnisonCurve<C>, E, Detune>;

/// Detunes various copies of a curve according to an envelope.
///
/// The detune amount is between successive intervals. A value of `0.0` means no detune, while `1.0`
/// means an octave detuning.
///
/// The curves within the [`Unison`] struct are indexed from highest to lowest pitched.
pub type DetuneSgn<S, C, E> = MutSgn<Unison<S, C>, E, Detune>;

impl<C: Map<Input = Val>, E: SignalMut<Sample = Env>> DetuneCurveSgn<C, E>
where
    C::Output: Sample,
{
    /// Initializes a [`DetuneCurveSgn`].
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

impl<S: Sample, C: Map<Input = Val, Output = f64>, E: SignalMut<Sample = Env>> DetuneSgn<S, C, E> {
    /// Initializes a [`DetuneSgn`].
    pub fn new_detune(map: C, base: Freq, num: u8, env: E) -> Self {
        Self::new_detune_curve(CurvePlayer::new(map), base, num, env)
    }
}

/// A reference to one of the curves played in a [`UnisonCurve`] or [`Unison`] object.
///
/// This allows you to freely route these signals into other effects.
///
/// See also [`Ref`].
///
/// ## Example
///
/// In this example, we progressively detune some saw waves, and then pan them left to right.
///
/// ```
/// use pointillism::prelude::*;
///
/// /// The number of waves playing.
/// const NUM: u8 = 7;
/// const SCALE: f64 = 1.0 / NUM as f64;
///
/// /// Length of the detuning envelope.
/// const LEN: RawTime = RawTime::new(5.0);
///
/// fn main() {
///     let len = Time::from_raw_default(LEN);
///
///     // Plays a number of notes, and detunes them up to an octave.
///     let mut unison = DetuneSgn::<Mono, _, _>::new_detune(
///         Saw,
///         Freq::from_raw_default(RawFreq::A3),
///         NUM,
///         OnceGen::new(Comp::new(Saw, Linear::rescale_sgn(0.0, SCALE)), len),
///     );
///
///     // If you play a large amount of curves and remove this, you'll get some wacky interference.
///     unison.sgn_mut().randomize_phases();
///
///     pointillism::create("examples/detune.wav", 2u8 * len, SampleRate::default(), |_| {
///         // We pan every curve according to how much its detuned.
///         let sgn: Stereo = (0..NUM)
///             .into_iter()
///             .map(|i| {
///                 MixedPanner::new_pan(
///                     UnisonRef::new(unison.base(), i),
///                     i as f64 / (NUM - 1) as f64,
///                 )
///                 .get()
///             })
///             .sum();
///
///         // We must advance the signal manually.
///         unison.advance();
///         sgn * SCALE
///     })
///     .expect("IO error!");
/// }
/// ```
pub struct UnisonRef<'a, C: Map<Input = Val>>
where
    C::Output: Sample,
{
    /// The underlying [`UnisonCurve`].
    pub unison: &'a UnisonCurve<C>,

    /// The index of the curve we're reading from.
    pub index: u8,
}

impl<'a, C: Map<Input = Val>> UnisonRef<'a, C>
where
    C::Output: Sample,
{
    /// Initializes a new [`UnisonRef`].
    pub const fn new(unison: &'a UnisonCurve<C>, index: u8) -> Self {
        Self { unison, index }
    }
}

impl<'a, C: Map<Input = Val>> Signal for UnisonRef<'a, C>
where
    C::Output: Sample,
{
    type Sample = C::Output;

    fn get(&self) -> C::Output {
        self.unison.get_at(self.index)
    }
}
