//! We play a single sine wave, and hook it onto [`cpal`].
//!
//! This requires the `cpal` feature.

#[cfg(feature = "cpal")]
fn main() {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use pointillism::prelude::*;

    // Set up the host and device.
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    // Query for the max sample rate.
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let sample_rate: SampleRate = supported_config.sample_rate().into();

    // Length of the sine wave.
    let duration = std::time::Duration::from_secs(1);
    // A reasonable buffer size.
    let buffer_size = cpal::BufferSize::Fixed(1024);
    // Sine wave frequency.
    let freq = Freq::from_hz(440.0, sample_rate);

    // We create a mono signal that loops through a sine curve at the specified frequency.
    let sgn = LoopGen::<Mono, _>::new(Sin, freq);

    // Creates the stream and plays it.
    let stream = pointillism::cpal::build_output_stream_from_sgn(
        &device,
        Some(duration),
        sample_rate,
        buffer_size,
        |err| eprintln!("{err}"),
        sgn,
    )
    .unwrap();
    stream.play().unwrap();

    // Make sure we don't exit before the file ends playing.
    std::thread::sleep(duration);
}

#[cfg(not(feature = "cpal"))]
fn main() {
    println!("This example must be run with the cpal feature.")
}
