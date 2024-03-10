extern crate proc_macro;
use proc_macro::TokenStream;
use std::fmt::Write;

/// Minimum octave for notes.
const MIN_OCTAVE: i16 = -1;
/// Maximum octave for notes.
const MAX_OCTAVE: i16 = 10;
/// Midi note C0.
const C0: i16 = 12;
/// Least Midi note.
const MIN_NOTE: i16 = 12 * (MIN_OCTAVE + 1);

/// List of note names and their indices from 0 to 11.
const NOTE_NAMES: [(char, i16); 7] = [
    ('C', 0),
    ('D', 2),
    ('E', 4),
    ('F', 5),
    ('G', 7),
    ('A', 9),
    ('B', 11),
];

/// Frequencies of notes in the first MIDI octave.
const NOTE_FREQS: [f64; 12] = [
    8.175_798_915_643_707,
    8.661_957_218_027_252,
    9.177_023_997_418_987,
    9.722_718_241_315_029,
    10.300_861_153_527_185,
    10.913_382_232_281_371,
    11.562_325_709_738_575,
    12.249_857_374_429_665,
    12.978_271_799_373_285,
    13.75,
    14.567_617_547_440_31,
    15.433_853_164_253_879,
];

/// For every note in our range, writes
///
/// ```rs
/// /// The note {note_name}.
/// pub const {var_name}: Self = {value};
/// ```
///
/// The variables `note_name` and `var_name` are calculated within the function, while `value` is
/// calculated from the function `f`.
fn for_all_notes<F: FnMut(&mut String, i16)>(mut f: F) -> proc_macro::TokenStream {
    let mut code = String::new();

    for octave in MIN_OCTAVE..=MAX_OCTAVE {
        for (name, index) in NOTE_NAMES {
            for (nat_symbol, symbol, offset) in [("", "", 0), ("♭", "B", -1), ("♯", "S", 1)] {
                // BS adds an octave, CB removes one.
                let index = index + offset;
                let octave = match index {
                    -1 => octave + 1,
                    12 => octave - 1,
                    _ => octave,
                };
                let note = 12 * octave + C0 + index;

                write!(
                    code,
                    "/// The note {name}{nat_symbol}<sub>{octave}</sub>.
                    pub const {name}{symbol}"
                )
                .unwrap();

                // Write negative signs as N.
                if octave < 0 {
                    write!(code, "N{}", -octave)
                } else {
                    write!(code, "{octave}")
                }
                .unwrap();

                write!(code, ": Self = ").unwrap();
                f(&mut code, note);
                code.push_str(";\n");
            }
        }
    }

    code.parse().expect("code could not be parsed")
}

/// Defines the constants for `MidiNote`.
#[proc_macro]
pub fn midi(_: TokenStream) -> TokenStream {
    for_all_notes(|code, note| {
        write!(code, "Self::new({note})").unwrap();
    })
}

/// Defines the constants for `RawFreq`.
#[proc_macro]
pub fn freq(_: TokenStream) -> TokenStream {
    for_all_notes(|code, note| {
        // Octaves raised relative to the least note.
        let octave = (note - MIN_NOTE).div_euclid(12) as u8;
        let index = note.rem_euclid(12) as usize;

        // Build the floating point 2 ^ octave.
        let pow = f64::from_bits((octave as u64 + 1023) << (f64::MANTISSA_DIGITS - 1));

        let freq = NOTE_FREQS[index] * pow;
        write!(code, "Self::new({freq:.15})").unwrap();
    })
}
