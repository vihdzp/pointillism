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
a `MutSgn`. Moreover, you can implement the trait for your own structs, giving 
you vast control over the samples you're producing.

## Naming conventions

The library has been coded in very large generality, and many types - even
"basic" ones, are actually type aliases. As such, many constructors `new`
are suffixed, to avoid ambiguity.

## Versions

The following versions of `pointillism` exist:

- 0.1.0 - 0.1.7: very early versions, have been yanked from `crates`.
- 0.2.0 - 0.2.3: more stable versions, but still subject to drastic change.

Once the basic structure of `pointillism` stabilizes, the version will advance
to 0.3.0, and a changelog will be made.