//! Declares control structures, which can be used to execute events at specified time intervals.

mod melody;
mod seq_loop;
mod timer;

pub use melody::{Melody, MelodyLoop, MelodySeq};
pub use seq_loop::{Loop, Sequence};
pub use timer::{Metronome, Timer};
