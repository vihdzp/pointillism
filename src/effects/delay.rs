use crate::prelude::*;

/// Plays a signal back with a delay. Can have some optional feedback.
///
/// To emulate a dry/wet effect, you can add this to the original signal in different proportions.
///
/// ## Instanciation
///
/// The [`Delay`] type is quite generic. It allows for a buffer that might or might not be owned,
/// and it lets the feedback function be essentially anything. However in most cases, you'll want an
/// owned buffer, and you'll want one of the following three feedback behaviors, for which we make
/// type aliases.
///
/// - No feedback: [`SingleDelay`].
/// - Exponential feedback: [`ExpDelay`].
/// - Exponential feedback + ping-pong: [`FlipDelay`].
#[derive(Clone, Debug)]
pub struct Delay<
    S: Signal<Sample = B::Item>,
    B: BufMutTrait,
    F: Map<Input = B::Item, Output = B::Item>,
> {
    /// The played signal.
    pub sgn: S,

    /// The backing [`LoopBufGen`]. This contains the buffer, and an index controlling the current
    /// position.
    ///
    /// The length of the buffer deterimes the length of the delay. Since buffers can only hold an
    /// integral amount of samples, a delay might not be exact to the frame. This probably isn't a
    /// problem unless you have a really long delay though.
    pub loop_gen: LoopBufGen<B>,

    /// A function that determines how a sample from the buffer is fed back into itself. This is
    /// most often used to lower its volume, and flip the channels for a ping-pong effect.
    ///
    /// The zero signal should be mapped to zero, or you'll get all sorts of unexpected behavior.
    pub feedback: F,
}

impl<S: Signal<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>>
    Delay<S, B, F>
{
    /// Initializes a new delay.
    ///    
    /// To use an empty, owned buffer, see [`Self::new_owned`].
    pub const fn new(sgn: S, buffer: B, feedback: F) -> Self {
        Self {
            sgn,
            loop_gen: LoopBufGen::new(buffer),
            feedback,
        }
    }

    /// Returns a reference to the inner buffer.
    pub const fn buffer(&self) -> &B {
        &self.loop_gen.buffer
    }

    /// Returns a mutable reference to the inner buffer.
    pub fn buffer_mut(&mut self) -> &mut B {
        &mut self.loop_gen.buffer
    }

    /// Clears the inner buffer.
    pub fn clear(&mut self) {
        self.buffer_mut().clear();
    }

    /// Reads the current sample from the signal and advances the buffer position. The signal is not
    /// advanced.
    ///
    /// You can use this if you're advancing the signal manually.
    pub fn read_sgn(&mut self) {
        // Computes the new signal.
        let sgn = self.sgn.get();
        let buf = self.loop_gen._get();
        let new_sgn = sgn + self.feedback.eval(buf);

        // Updates the buffer, advances its position.
        let idx = self.loop_gen.index();
        self.buffer_mut()[idx] = new_sgn;
        self.loop_gen.advance();
    }
}

impl<S: Signal, F: Map<Input = S::Sample, Output = S::Sample>> Delay<S, Buffer<S::Sample>, F>
where
    S::Sample: Audio,
{
    /// Initializes a new delay that owns its buffer. The size of the buffer is determined by the
    /// delay time.
    pub fn new_owned(sgn: S, delay: Time, feedback: F) -> Self {
        Self::new(sgn, Buffer::empty(delay.samples.int() as usize), feedback)
    }
}

impl<S: Signal<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>> Signal
    for Delay<S, B, F>
{
    type Sample = S::Sample;

    fn get(&self) -> Self::Sample {
        self.loop_gen._get()
    }
}

impl<S: SignalMut<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>>
    SignalMut for Delay<S, B, F>
{
    fn advance(&mut self) {
        self.read_sgn();
        self.sgn.advance();
    }

    fn retrigger(&mut self) {
        // Retriggers the signals.
        self.sgn.retrigger();
        self.loop_gen.retrigger();

        // Clears the buffer.
        self.clear();
    }
}

impl<S: Frequency<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>>
    Frequency for Delay<S, B, F>
{
    fn freq(&self) -> Freq {
        self.sgn.freq()
    }

    fn freq_mut(&mut self) -> &mut Freq {
        self.sgn.freq_mut()
    }
}

impl<S: Base<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>> Base
    for Delay<S, B, F>
{
    type Base = S::Base;

    fn base(&self) -> &S::Base {
        self.sgn.base()
    }

    fn base_mut(&mut self) -> &mut S::Base {
        self.sgn.base_mut()
    }
}

impl<S: Stop<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>> Stop
    for Delay<S, B, F>
{
    fn stop(&mut self) {
        self.sgn.stop();
    }
}

impl<S: Panic<Sample = B::Item>, B: BufMutTrait, F: Map<Input = B::Item, Output = B::Item>> Panic
    for Delay<S, B, F>
{
    fn panic(&mut self) {
        self.sgn.panic();
        self.clear();
    }
}

/// A delay that only plays once.
pub type SingleDelay<S, B> = Delay<S, B, Zero<<S as Signal>::Sample, <S as Signal>::Sample>>;

impl<S: Signal<Sample = B::Item>, B: BufMutTrait> SingleDelay<S, B> {
    /// Initializes a delay that only plays once.
    ///
    /// To use an empty, owned buffer, see [`Self::new_single_owned`].
    pub const fn new_single(sgn: S, buffer: B) -> Self {
        Self::new(sgn, buffer, Zero::new())
    }
}

impl<S: Signal> SingleDelay<S, Buffer<S::Sample>>
where
    S::Sample: Audio,
{
    /// Initializes a delay that only plays once and owns its buffer. The size of the buffer is
    /// determined by the delay time.
    pub fn new_single_owned(sgn: S, delay: Time) -> Self {
        Self::new_owned(sgn, delay, Zero::new())
    }
}

/// A delay that feeds back into itself with some gain factor.
///
/// This causes the signal to decay exponentially. You can set the volume to `1.0` for an infinite
/// delay, but other than that, you'll probably want a value between `0.0` and `1.0`, exclusively.
pub type ExpDelay<S, B> = Delay<S, B, Pw<<S as Signal>::Sample, Vol>>;

impl<S: Signal<Sample = B::Item>, B: BufMutTrait> ExpDelay<S, B> {
    /// Initializes a new [`ExpDelay`].
    ///
    /// To use an empty, owned buffer, see [`Self::new_exp_owned`].
    pub const fn new_exp(sgn: S, buffer: B, vol: Vol) -> Self {
        Self::new(sgn, buffer, Pw::new(vol))
    }

    /// Returns the feedback volume.
    pub const fn vol(&self) -> Vol {
        self.feedback.func
    }

    /// Returns a mutable reference to the feedback volume.
    pub fn vol_mut(&mut self) -> &mut Vol {
        &mut self.feedback.func
    }
}

impl<S: Signal> ExpDelay<S, Buffer<S::Sample>>
where
    S::Sample: Audio,
{
    /// Initializes a delay with exponential decay that owns its buffer. The size of the buffer is
    /// determined by the delay time.
    pub fn new_exp_owned(sgn: S, delay: Time, vol: Vol) -> Self {
        Self::new_owned(sgn, delay, Pw::new(vol))
    }
}

/// An exponential delay with a ping-pong effect.
pub type FlipDelay<S, B> = Delay<S, B, Comp<Pw<Stereo, Vol>, Flip>>;

/// Simple auxiliary function.
const fn comp_flip(vol: Vol) -> Comp<Pw<Stereo, Vol>, Flip> {
    Comp::new(Pw::new(vol), Flip)
}

impl<S: Signal<Sample = Stereo>, B: BufMutTrait<Item = Stereo>> FlipDelay<S, B> {
    /// Initializes a new [`FlipDelay`].
    ///
    /// You can set the volume to `1.0` for an infinite delay, but other than that, you'll probably
    /// want a value between `0.0` and `1.0`, exclusively.
    pub const fn new_flip(sgn: S, buffer: B, vol: Vol) -> Self {
        Self::new(sgn, buffer, comp_flip(vol))
    }

    /// Returns the feedback volume.
    pub const fn vol(&self) -> Vol {
        self.feedback.inner.func
    }

    /// Returns a mutable reference to the feedback volume.
    pub fn vol_mut(&mut self) -> &mut Vol {
        &mut self.feedback.inner.func
    }
}

impl<S: Signal<Sample = Stereo>> FlipDelay<S, Buffer<S::Sample>> {
    /// Initializes a ping-pong delay that owns its buffer. The size of the buffer is determined by
    /// the delay time.
    pub fn new_flip_owned(sgn: S, delay: Time, vol: Vol) -> Self {
        Self::new_owned(sgn, delay, comp_flip(vol))
    }
}
