# Pointillism

A compositional library for musical composition.

## Examples

If you want to see `pointillism` in action and what it's capable of, run the
examples in the `examples` folder.

**Note:** Some examples may be loud, dissonant, and/or jarring. Hearing
discretion is advised.

## Design

The way in which `pointillism` outputs audio is by writing sample by sample into
a 32-bit floating point `.wav` file. Internal calculations use 64-bit floating
points.

For convenience, the `Signal` trait is provided. Structs implementing this trait
generate sample data frame by frame, which can be advanced or retriggered.

Signals may be composed to create more complex signals, using for instance the
`MapSgn` and `MutSgn` structs. Moreover, you can implement the `Signal` trait
for your own structs, giving you vast control over the samples you're producing.

Signals that generate audio on their own are called *generators*. Their names
are suffixed by `Gen`. Signals that modify the output from another signal are
called *effects*.

You can think of pointillism as a compile-time modular synthesizer, where every
new struct is its own module.

## Versions

The following versions of `pointillism` exist:

- 0.1.0 - 0.1.7: very early versions, have been yanked from `crates`.
- 0.2.0 - 0.2.8: more stable versions, but still subject to drastic change.

Once the basic structure of `pointillism` stabilizes, the version will advance
to 0.3.0, and a changelog will be made.