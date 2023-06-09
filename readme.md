# Pointillism

A compositional library for musical composition.

## Examples

If you want to see `pointillism` in action and what it's capable of, run the examples in the
`examples` folder.

**Note:** Some examples may be loud, dissonant, and/or jarring. Hearing discretion is advised.

## Design

The way in which `pointillism` outputs audio is by writing sample by sample into a 32-bit floating
point `.wav` file. Internal calculations use 64-bit floating points.

For convenience, the `Signal` trait is provided. Types implementing this trait generate sample data
frame by frame. If the type also implements `SignalMut`, it can be advanced or retriggered.

Signals may be composed to create more complex signals, using for instance the `MapSgn` and `MutSgn`
structs. Moreover, you can implement the `Signal` and `SignalMut` traits for your own structs,
giving you vast control over the samples you're producing.

Signals that generate audio on their own are called *generators*. Their names are suffixed by `Gen`.
Signals that modify the output from another signal are called *effects*.

## Compile-time

You can think of pointillism as a compile-time modular synthesizer, where every new struct is its
own module.

Advantages of this design are extensibility and generality. It's relatively easy to create a highly
customizable and complex signal with many layers by composing some functions together.

The downside is that these synths end up having unwieldy type signatures. Moreso, it's really hard
to build synths in real time.

## Versions

The following versions of `pointillism` exist:

- 0.1.0 - 0.1.7: very early versions, have been yanked from `crates`.
- 0.2.0 - 0.2.10: more stable versions, but still subject to drastic change.
- 0.3.0 - 0.3.1: stable versions, tracked by a changelog.