//! Implements the [`Timer`] and [`Metronome`] types.
//!
//! These can be used as part of some control flow to perform an action at a specified point in time.

use crate::prelude::*;

/// A control structure that can be used to execute an event once after a certain time duration.
///
/// The intended use is to first declare a new [`Timer`] through [`Timer::new`], and then call
/// [`Timer::tick`] for every sample of music. When the elapsed time has just passed, this function
/// will return `true`, which can be used to execute some branching code.
///
/// ## Example
///
/// ```
/// # use pointillism::prelude::*;
/// // The global song time.
/// let mut time = unt::Time::new();
/// // Our timer.
/// let mut timer = ctr::Timer::new(unt::Time::from_samples(10));
///
/// for i in 0..20 {
///     if timer.tick(time) {
///         println!("The timer ticked on frame {i}!");
///         assert_eq!(i, 10);
///     }
///
///     time.advance();
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Timer {
    /// Length of the timer.
    length: unt::Time,
    /// Whether the timer is active.
    active: bool,
}

impl Timer {
    /// Initializes a new timer with the given length.
    #[must_use]
    pub const fn new(length: unt::Time) -> Self {
        Self {
            length,
            active: true,
        }
    }

    /// Whether the timer should activate, given the current time elapsed. A timer can only activate
    /// once.
    pub fn tick(&mut self, time: unt::Time) -> bool {
        if !self.active {
            return false;
        }

        let done = time >= self.length;
        if done {
            self.active = false;
        }
        done
    }

    /// Resets the timer, allowing it to activate again.
    pub fn reset(&mut self) {
        self.active = true;
    }
}

impl From<unt::Time> for Timer {
    fn from(length: unt::Time) -> Self {
        Timer::new(length)
    }
}

/// A control structure that can be used to execute an event every certain time duration.
///
/// The intended use is to first declare a new [`Metronome`] through [`Metronome::new`], and then
/// call [`Metronome::tick`] for every sample of music. When the elapsed time has just passed, this
/// function will return `true`, which can be used to execute some branching code.
///
/// Note that by design, the metronome can only tick once per frame. If you need something more
/// sophisticated that can run simultaneous events, consider using a [`ctr::Seq`] or [`ctr::Loop`]
/// instead.
///
/// By default, the metronome ticks on the first frame. You can add some condition like
/// `!time.is_zero()` to ignore this tick.
///
/// ## Example
///
/// ```
/// # use pointillism::prelude::*;
/// // The global song time.
/// let mut time = unt::Time::new();
/// // Our metronome.
/// let mut metr = ctr::Metronome::new(unt::Time::from_samples(10));
///
/// for i in 0..20 {
///     if metr.tick(time) {
///         println!("The metronome ticked on frame {i}!");
///         assert_eq!(i % 10, 0);
///     }
///
///     time.advance();
/// }
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Metronome {
    /// Period of the metronome.
    period: unt::Time,
}

impl Metronome {
    /// Initializes a new metronome with the given period.
    #[must_use]
    pub const fn new(period: unt::Time) -> Self {
        Self { period }
    }

    /// Whether the metronome should activate, given the current time elapsed. A timer can only
    /// activate once per frame.
    pub fn tick(&mut self, time: unt::Time) -> bool {
        time % self.period < unt::Time::SAMPLE
    }
}

impl From<unt::Time> for Metronome {
    fn from(period: unt::Time) -> Self {
        Metronome::new(period)
    }
}
