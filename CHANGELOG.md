## Version 0.3.2

Create an `Arpeggio` type, and other small improvements throughout.

## Version 0.3.1

The `Mut` trait has been split in two, according to its actual uses of modifying a signal according
to an envelope (`MutEnv`), or simply modifying a signal (`Mut`).

`Sequences` and `Loops` no longer track time, as this is something better done manually if needed.

## Version 0.3.0

The basic structure of `pointillism` has been stabilized, and most basic features have been
implemented. Successive changes will be tracked in this file from now on.