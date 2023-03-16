# Pointillism

A compositional library for musical composition.

## Examples

If you want to see `pointillism` in action and what it's capable of, run the
examples in the `examples` folder.

**Note:** Some examples may be loud, dissonant, and/or jarring. Hearing 
discretion is advised.

## Design

The way in which `pointillism` outputs audio is by writing sample by sample
into a `.wav` file. The output is hardcoded to use 32-bit floating point, 
although calculations are internally done using 64-bit, for extra precision.

For convenience, the `Signal` trait is provided. Structs implementing this trait 
generate sample data frame by frame, which can be advanced, or retriggered.

Signals may be composed to create more complex signals, using for instance
an `Envelope`. Moreover, you can implement the trait for your own structs,
giving you vast control over the samples you're producing.