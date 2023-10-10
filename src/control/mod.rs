//! Declares control structures, which can be used to execute events at specified time intervals.

mod melody;
mod seq_loop;

pub use melody::{Melody, MelodyLoop, MelodySeq};
pub use seq_loop::{Loop, Sequence};

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
/// let mut timer = unt::Timer::new(unt::Time::from_samples(10));
///
/// for i in 0..20 {
///     if timer.tick() {
///         println!("The timer ticked on frame {i}!");
///         assert_eq!(i, 10);
///     }
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
