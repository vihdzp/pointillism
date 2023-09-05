//! Implements the types for frequency: [`RawFreq`], [`Freq`], [`Interval`].
//!
//! ## Equal division of the octave
//!
//! Most music is written using 12 notes. These form an [equal division of the
//! octave](https://en.wikipedia.org/wiki/Equal_temperament). However, much interesting music can be
//! made that uses other more complicated scales. For this reason, although we provide convenience
//! methods for 12-EDO, we also provide many other methods that allow you to specify your own equal
//! division of the octave. If you want something even more general, you can always input the raw
//! numbers yourself.

mod freq;
mod interval;
mod raw;

pub use {freq::Freq, interval::Interval, raw::RawFreq};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_a4() {
        assert_eq!(format!("{:#?}", RawFreq::A4), "A4 +0c");
    }

    #[test]
    fn parse_a4() {
        let a4: RawFreq = "A4".parse().unwrap();
        assert_approx_eq::assert_approx_eq!(a4.hz, RawFreq::A4.hz);
    }
}
