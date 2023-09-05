mod raw;
mod time;

pub use raw::{RawTime, RawTimer};
pub use time::Time;

/// If the `human_duration` feature is enabled, we use the [`human_duration`] crate for
/// pretty-printing.
const HUMAN_DURATION: bool = cfg!(feature = "human-duration");

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
