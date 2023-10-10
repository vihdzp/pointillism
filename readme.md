# Pointillism

A compositional library for musical composition.

## Examples

If you want to see pointillism in action and what it's capable of, run the examples in the
`examples` folder. There's also many simple examples scattered throughout the source code, showing
off different features.

For a starting example, see the [`create`] docs.

**Note:** Some examples may be loud, dissonant, and/or jarring. Hearing discretion is advised.

## Design

The default way in which pointillism outputs audio is by writing sample by sample into a 32-bit
floating point `.wav` file. Internal calculations use 64-bit floating points.

For convenience, the [`Signal`] trait is provided. Types implementing this trait generate sample
data frame by frame. If the type also implements [`SignalMut`], it can be advanced or retriggered.

Signals may be composed to create more complex signals, using for instance the [`MapSgn`] and
[`MutSgn`] structs. Moreover, you can implement the [`Signal`] and [`SignalMut`] traits for your own
structs, giving you vast control over the samples you're producing.

### Naming scheme

The `pointillism` code has a lot of moving parts, and a bunch of similarly named types. Because of
this, we rely on the `prelude` to categorize things neatly.

Every type has a three-letter namespace which helps categorizes it. The main namespaces are as
follows:

| Namespace | Full Name | Contents |
|-|-|-|
| [`unt`] | `units` | Different units for musical measurement, and associated arithmetic boilerplate.
| [`crv`] | `curves` | Basic oscillator shapes, and builder methods for more complex ones (in the future).
| [`gen`] | `generators` | Types that generate a signal "on their own". This includes the basic oscillators like [`gen::Loop`] and [`gen::Once`].
| [`eff`] | `effects` | For effects, meaning types that alter other signals.
| [`ctr`] | `control` | Control structures, which allow for events to happen at specified time intervals.
| [`sgn`] | `signal` | Traits on signals, including the basic [`sgn::Ref`] and [`sgn::Mut`].
| [`map`] | `map` | Basic functions.

### Compile-time

You can think of pointillism as a compile-time modular synthesizer, where every new struct is its
own module.

Advantages of this design are extensibility and generality. It's relatively easy to create a highly
customizable and complex signal with many layers by composing some functions together.

The downside is that these synths end up having unwieldy type signatures. Moreso, it's really hard
to build synths in real time.

### Features

The project uses the following features:

| Feature | Enables |
|-|-|
| [`hound`](https://docs.rs/hound/latest/hound/)* | Saving songs as WAV files. |
| [`cpal`](https://docs.rs/cpal/latest/cpal/) | Playing songs in a dedicated thread. |
| [`midly`](https://docs.rs/midly/latest/midly/) | Reading and playing back MIDI files. |
| [`human-duration`](https://docs.rs/human-duration/latest/human_duration/)* | Pretty-printing for the [`RawTime`] type. |

\* Features marked with an asterisk are enabled by default.

## Goals

Future goals of pointillism are:

- Algorithmic reverb
- Limiters, compressors, sidechaining
- [Me](https://viiii.bandcamp.com) making a whole album with it :D

## Disclaimer

This is a passion project made by one college student learning about DSP. I make no guarantees on it
being well-designed, well-maintained, or usable for your own goals.

If you just want to make music with code, and especially if you enjoy live feedback,
[SuperCollider](https://supercollider.github.io/) and [Pure Data](https://puredata.info/) will most
likely be better alternatives for you.

That said, if you happen to stumble across this and make something cool, please let me know!