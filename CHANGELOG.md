## Version 0.3.6

Added MIDI support via the `midly` crate.

## Version 0.3.5

Added `MelodySeq` and `MelodyLoop` type aliases, which allow building and playing a melody as if in
a piano roll.

Also, improved pretty-printing for time and frequency units.

## Version 0.3.4

Added `cpal` support, enabled through the `cpal` feature. Also turned `hound` support into a
feature.

## Version 0.3.3

The types `Time` and `Freq` have been refactored.

A "raw" version of both types has been added, which records these quantities in natural units `s`
and `s⁻¹` respectively.

These raw types can be converted into `RawTime` and `Freq`, which are now in units `samples` and
`samples⁻¹` instead. This change achieves two things:

- Advancing time by one sample no longer loses precision.
- Different sample rates are now easily supported.

## Version 0.3.2

Create an `Arpeggio` type, and other small improvements throughout.

## Version 0.3.1

The `Mut` trait has been split in two, according to its actual uses of modifying a signal according
to an envelope (`MutEnv`), or simply modifying a signal (`Mut`).

`Sequences` and `Loops` no longer track time, as this is something better done manually if needed.

## Version 0.3.0

The basic structure of `pointillism` has been stabilized, and most basic features have been
implemented. Successive changes will be tracked in this file from now on.