use crate::{sample::Sample, signal::Signal, Freq, SAMPLE_RATE};

/// Generates random data.
#[derive(Clone, Copy, Debug)]
pub struct NoiseGen<S: Sample> {
    /// The current random value.
    current: S,
}

impl<S: Sample> Default for NoiseGen<S> {
    fn default() -> Self {
        Self { current: S::rand() }
    }
}

impl<S: Sample> NoiseGen<S> {
    /// Initializes a new [`NoiseGen`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S: Sample> Signal for NoiseGen<S> {
    type Sample = S;

    fn get(&self) -> Self::Sample {
        self.current
    }

    fn advance(&mut self) {
        self.current = S::rand();
    }

    fn retrigger(&mut self) {
        self.advance();
    }
}

/// Generates random data at a given frequency.
#[derive(Clone, Copy, Debug)]
pub struct NoiseStepGen<S: Sample> {
    /// Frequency at which random data is generated.
    pub freq: Freq,

    /// The current random value.
    current: S,

    /// Keeps track of when a new value must be generated.
    val: f64,
}

impl<S: Sample> NoiseStepGen<S> {
    /// Initializes a new [`NoiseStepGen`].
    pub fn new(freq: Freq) -> Self {
        Self {
            freq,
            current: S::rand(),
            val: 0.0,
        }
    }
}

impl<S: Sample> Signal for NoiseStepGen<S> {
    type Sample = S;

    fn get(&self) -> S {
        self.current
    }

    fn advance(&mut self) {
        self.val += self.freq.hz() / SAMPLE_RATE as f64;

        if self.val >= 1.0 {
            self.val -= 1.0;
            self.current = S::rand();
        }
    }

    fn retrigger(&mut self) {
        self.val = 0.0;
        self.current = S::rand();
    }
}
