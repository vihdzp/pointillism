//! Declares control structures, which can be used to execute events at specified time intervals.

pub mod melody;
mod seq_loop;
mod timer;

pub use melody as mel;
pub use seq_loop::{Arpeggio, Loop, Seq};
pub use timer::{Metronome, Timer};
