//! Implements the types for frequency: [`RawTime`] and [`Time`].

mod raw;
mod time;

pub use raw::RawTime;
pub use time::Time;

/// If the `human_duration` feature is enabled, we use the [`human_duration`] crate for
/// pretty-printing.
const HUMAN_DURATION: bool = cfg!(feature = "human-duration");

/// A helper struct that can be used to do an event once after a certain time duration.
#[derive(Clone, Copy, Debug)]
pub struct Timer {
    /// Length of the timer.
    length: Time,
    /// Whether the timer is active.
    active: bool,
}

impl Timer {
    /// Initializes a new timer with the given length.
    #[must_use]
    pub const fn new(length: Time) -> Self {
        Self {
            length,
            active: true,
        }
    }

    /// Whether the timer should activate, given the current time elapsed. A timer can only activate
    /// once.
    pub fn tick(&mut self, time: Time) -> bool {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn yr_secs() {
        if HUMAN_DURATION {
            assert_eq!(format!("{:#?}", RawTime::YR), "1y 0mon 0d 0h 0m 0s 0ms");
        }
    }
}
