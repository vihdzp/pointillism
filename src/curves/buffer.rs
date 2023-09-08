//! Defines the [`Buffer`] type for sample buffers.
//!
//! ## Supported WAV formats
//!
//! The [`hound`] library, and `pointillism` by extension, support only WAV files in the following
//! formats:
//!    
//! - 8-bit integer
//! - 16-bit integer
//! - 24-bit integer
//! - 32-bit integer
//! - 32-bit float

use std::path::Path;

use crate::{prelude::*, sample::WavSample};

/// A reader for a WAV file.
#[cfg(feature = "hound")]
pub type WavFileReader = hound::WavReader<std::io::BufReader<std::fs::File>>;

/// A trait for some sort of buffer, mutable or not, that stores audio data.
///
/// This implements some auxiliary methods to calculate properties of the buffer.
pub trait BufTrait: AsRef<[Self::Item]> {
    /// The type of sample stored in the buffer.
    type Item: Audio;

    /// Returns the sample corresponding to peak amplitude on all channels.
    #[must_use]
    fn peak(&self) -> <Self::Item as ArrayLike>::Array<Vol> {
        /// Prevent code duplication.
        fn peak<A: Audio>(buf: &[A]) -> <A as ArrayLike>::Array<Vol> {
            let mut res = A::new_default();

            for sample in buf {
                A::for_each(|index| {
                    let peak = &mut res[index];
                    let new = sample[index].abs();

                    if *peak > new {
                        *peak = new;
                    }
                });
            }

            res.map_array(|&x| Vol::new(x))
        }

        peak(self.as_ref())
    }

    /// Calculates the RMS on all channels.
    #[must_use]
    fn rms(&self) -> <Self::Item as ArrayLike>::Array<Vol> {
        /// Prevent code duplication.
        fn rms<A: Audio>(buf: &[A]) -> <A as ArrayLike>::Array<Vol> {
            let mut res: <A as ArrayLike>::Array<f64> = ArrayLike::new_default();

            for sample in buf {
                A::for_each(|index| {
                    let new = sample[index];
                    res[index] += new * new;
                });
            }

            // Precision loss should not occur in practice.
            #[allow(clippy::cast_precision_loss)]
            A::for_each(|index| {
                res[index] = (res[index] / buf.len() as f64).sqrt();
            });

            res.map_array(|&x| Vol::new(x))
        }

        rms(self.as_ref())
    }
}

impl<A: Audio> BufTrait for [A] {
    type Item = A;
}

impl<A: Audio> BufTrait for &[A] {
    type Item = A;
}

impl<A: Audio> BufTrait for &mut [A] {
    type Item = A;
}

/// A sample buffer.
#[derive(Clone, Debug, Default)]
pub struct Buffer<A: Audio> {
    /// The data stored by the buffer.
    data: Vec<A>,
}

impl<A: Audio> Buffer<A> {
    /// Initializes a new buffer from data.
    #[must_use]
    pub const fn from_data(data: Vec<A>) -> Self {
        Self { data }
    }

    /// Initializes a new empty buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self::from_data(Vec::new())
    }

    /// Returns the inner slice.    
    #[must_use]
    pub fn as_slice(&self) -> &[A] {
        self.data.as_slice()
    }

    /// Returns a mutable reference to the inner slice.
    pub fn as_mut_slice(&mut self) -> &mut [A] {
        self.data.as_mut_slice()
    }

    /// Returns the number of samples in the buffer.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns the time that takes to play this buffer.
    #[must_use]
    pub fn time(&self) -> Time {
        Time::new(crate::units::FracInt::new(self.data.len() as u64))
    }

    /// Gets a sample at a given index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<A> {
        self.data.get(index).copied()
    }

    /// Gets a mutable reference to a sample at a given index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut A> {
        self.data.get_mut(index)
    }
}

impl<A: Audio> AsRef<[A]> for Buffer<A> {
    fn as_ref(&self) -> &[A] {
        self.as_slice()
    }
}

impl<A: Audio> AsMut<[A]> for Buffer<A> {
    fn as_mut(&mut self) -> &mut [A] {
        self.as_mut_slice()
    }
}

impl<A: Audio> BufTrait for Buffer<A> {
    type Item = A;
}

impl<A: Audio> std::ops::Index<usize> for Buffer<A> {
    type Output = A;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<A: Audio> std::ops::IndexMut<usize> for Buffer<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<A: Audio> FromIterator<A> for Buffer<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::from_data(FromIterator::from_iter(iter))
    }
}

impl<A: Audio> IntoIterator for Buffer<A> {
    type IntoIter = std::vec::IntoIter<A>;
    type Item = A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, A: Audio> IntoIterator for &'a Buffer<A> {
    type IntoIter = std::slice::Iter<'a, A>;
    type Item = &'a A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, A: Audio> IntoIterator for &'a mut Buffer<A> {
    type IntoIter = std::slice::IterMut<'a, A>;
    type Item = &'a mut A;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

/// An error in reading WAV files.
#[derive(Debug)]
pub enum Error {
    /// The number of channels got was different from the expected.
    ChannelMismatch {
        /// True if [`Mono`] was expected, false if [`Stereo`] was expected.
        expected_mono: bool,
    },

    /// Some other error managed by [`hound`].
    Hound(hound::Error),
}

impl From<hound::Error> for Error {
    fn from(value: hound::Error) -> Self {
        Error::Hound(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Self::ChannelMismatch { expected_mono } => {
                if expected_mono {
                    write!(f, "expected mono audio, got stereo audio")
                } else {
                    write!(f, "expected stereo audio, got mono audio")
                }
            }

            Self::Hound(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {}

/// Initializes a [`WavFileReader`] with the given path, which can expect a [`Mono`] or [`Stereo`]
/// file.
///
/// ## Errors
///
/// Will error in case of an IO error, or if there is a mismatch in the number of channels.
fn init_reader(path: &Path, expected_mono: bool) -> Result<WavFileReader, Error> {
    let reader = hound::WavReader::open(path)?;
    if (reader.spec().channels == 1) != expected_mono {
        return Err(Error::ChannelMismatch { expected_mono });
    }
    Ok(reader)
}

impl Buffer<Mono> {
    /// Allocates memory in which to store a buffer of a given size. If `length = 0` is passed, the
    /// null pointer will be returned.
    ///
    /// This is useful for two reasons:
    ///
    /// - It serves as an optimization. An audio buffer from a WAV file should have no reason to
    ///   change in size.
    /// - It allows us to transmute an interleaved array of `Mono` samples into an array of `Stereo`
    ///   samples.
    fn get_ptr(length: usize) -> *mut Mono {
        // This must be handled separately, as `alloc::alloc` doesn't allow for an empty layout.
        if length == 0 {
            return std::ptr::null_mut();
        }

        let layout = std::alloc::Layout::array::<Mono>(length).unwrap();

        // Safety: the layout is nonempty, the alignment is set explicitly.
        #[allow(clippy::cast_ptr_alignment)]
        let ptr = unsafe { std::alloc::alloc(layout) }.cast::<Mono>();

        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout)
        } else {
            ptr
        }
    }

    /// Reads from a `WavReader` into a pointer (returned from [`Self::get_ptr`]).
    ///
    /// This function is generic in `S`. For a non-generic version, see [`Self::write_ptr`].
    ///
    /// ## Safety
    ///
    /// All of the samples returned from the iterator must fit exactly in the allocated memory area.
    ///
    /// ## Errors
    ///
    /// Will return an error if a sample can't be turned into the specified type `S`.
    unsafe fn write_ptr_gen<S: WavSample>(
        reader: WavFileReader,
        ptr: *mut Mono,
    ) -> hound::Result<()> {
        let length = reader.len() as usize;

        for (index, sample) in reader.into_samples::<S>().enumerate() {
            // This is safe in debug.
            debug_assert!(index < length);
            *ptr.add(index) = sample?.into_mono();
        }

        Ok(())
    }

    /// Reads from a `WavReader` into a pointer (returned from [`Self::get_ptr`]).
    ///
    /// See also [`Self::write_ptr_gen`].
    ///
    /// ## Safety
    ///
    /// All of the samples returned from the iterator must fit exactly in the allocated memory area.
    ///
    /// ## Errors
    ///
    /// This should not error as long as the WAV file is in a supported format. See the [module
    /// docs](self) for a list.
    unsafe fn write_ptr(reader: WavFileReader, ptr: *mut Mono) -> hound::Result<()> {
        match reader.spec().sample_format {
            hound::SampleFormat::Float => Self::write_ptr_gen::<f32>(reader, ptr),
            hound::SampleFormat::Int => match reader.spec().bits_per_sample {
                8 => Self::write_ptr_gen::<i8>(reader, ptr),
                16 => Self::write_ptr_gen::<i16>(reader, ptr),
                24 | 32 => Self::write_ptr_gen::<i32>(reader, ptr),
                _ => Err(hound::Error::Unsupported),
            },
        }
    }

    /// Creates a buffer from an initialized pointer, returned from either [`Self::write_ptr_gen`]
    /// or [`Self::write_ptr`].
    ///
    /// ## Safety
    ///
    /// If `ptr` is not null, the memory area must be initialized, and have the exact length (in
    /// [`Mono`] samples) passed as an argument.
    unsafe fn from_ptr(length: usize, ptr: *mut Mono) -> Self {
        if ptr.is_null() {
            Buffer::new()
        } else {
            debug_assert_ne!(length, 0);
            Buffer::from_data(unsafe { Vec::from_raw_parts(ptr, length, length) })
        }
    }

    /// Creates a [`Mono`] buffer from a wav file, with a given [`WavSample`] format.
    ///
    /// See [`Self::from_wav`] for a non-generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The read samples can't be converted into the specified type `S`.
    /// - The WAV format is unsupported (see the [module docs](self)).
    /// - Some IO error related to opening the file.
    /// - The WAV file has more than one channel.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(path: P) -> Result<Self, Error> {
        let reader = init_reader(path.as_ref(), true)?;
        let length = reader.len() as usize;
        let ptr = Self::get_ptr(length);

        // Safety: the memory area has the correct length.
        unsafe {
            Self::write_ptr_gen::<S>(reader, ptr)?;
            Ok(Self::from_ptr(length, ptr))
        }
    }

    /// Creates a [`Mono`] buffer from a wav file.
    ///
    /// See [`Self::from_wav_gen`] for a generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The WAV format is unsupported (see the [module docs](self)).
    /// - Some IO error related to opening the file.
    /// - The WAV file has more than one channel.
    pub fn from_wav<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let reader = init_reader(path.as_ref(), true)?;
        let length = reader.len() as usize;
        let ptr = Self::get_ptr(length);

        // Safety: the memory area has the correct length.
        unsafe {
            Self::write_ptr(reader, ptr)?;
            Ok(Self::from_ptr(length, ptr))
        }
    }
}

impl Buffer<Stereo> {
    /// Creates a buffer from an initialized pointer, returned from either [`Buffer::write_ptr_gen`]
    /// or [`Buffer::write_ptr`].
    ///
    /// This pointer should point to memory consisting of an even number of interleaved `Mono`
    /// samples.
    ///
    /// ## Safety
    ///
    /// If `ptr` is not null, the memory area must be initialized, and have the exact length (in
    /// [`Mono`] samples) passed as an argument.
    ///
    /// In particular, `length` must be even.
    fn from_ptr(length: usize, ptr: *mut Mono) -> Self {
        if ptr.is_null() {
            Buffer::new()
        } else {
            debug_assert_eq!(length % 2, 0);
            debug_assert_ne!(length, 0);

            Buffer::from_data(unsafe {
                Vec::from_raw_parts(ptr.cast::<Stereo>(), length / 2, length / 2)
            })
        }
    }

    /// Creates a [`Stereo`] buffer from a wav file, with a given [`WavSample`] format.
    ///
    /// See [`Self::from_wav`] for a non-generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The read samples can't be converted into the specified type `S`.
    /// - The WAV format is unsupported (see the [module docs](self)).
    /// - Some IO error related to opening the file.
    /// - The WAV file doesn't have 2 channels.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(path: P) -> Result<Self, Error> {
        let reader = init_reader(path.as_ref(), false)?;
        let length = reader.len() as usize;
        let ptr = Buffer::get_ptr(length);

        // Safety: the memory area has the correct length.
        unsafe {
            Buffer::write_ptr_gen::<S>(reader, ptr)?;
            Ok(Self::from_ptr(length, ptr))
        }
    }

    /// Creates a [`Stereo`] buffer from a wav file.
    ///
    /// See [`Self::from_wav_gen`] for a generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The WAV format is unsupported (see the [module docs](self)).
    /// - Some IO error related to opening the file.
    /// - The WAV file doesn't have 2 channels.
    pub fn from_wav<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let reader = init_reader(path.as_ref(), false)?;
        let length = reader.len() as usize;
        let ptr = Buffer::get_ptr(length);

        // Safety: the memory area has the correct length.
        unsafe {
            Buffer::write_ptr(reader, ptr)?;
            Ok(Self::from_ptr(length, ptr))
        }
    }
}

/// A generator that reads through an audio buffer, once.
#[derive(Clone, Debug)]
pub struct OnceBufGen<A: Audio> {
    /// The inner buffer.
    buffer: Buffer<A>,

    /// The sample being read.
    index: usize,
}

impl<A: Audio> OnceBufGen<A> {
    /// Initializes a new [`OnceBufGen`].
    #[must_use]
    pub const fn new(buffer: Buffer<A>) -> Self {
        Self { buffer, index: 0 }
    }

    /// Returns a reference to the underlying buffer.
    #[must_use]
    pub const fn buffer(&self) -> &Buffer<A> {
        &self.buffer
    }

    /// Returns a mutable reference to the underlying buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer<A> {
        &mut self.buffer
    }
}

impl<A: Audio> Signal for OnceBufGen<A> {
    type Sample = A;

    fn get(&self) -> A {
        self.buffer().get(self.index).unwrap_or_default()
    }
}

impl<A: Audio> SignalMut for OnceBufGen<A> {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<A: Audio> Base for OnceBufGen<A> {
    impl_base!();
}

impl<A: Audio> Stop for OnceBufGen<A> {
    fn stop(&mut self) {
        self.index = self.buffer().len();
    }
}

impl<A: Audio> Done for OnceBufGen<A> {
    fn is_done(&self) -> bool {
        self.index >= self.buffer().len()
    }
}

impl<A: Audio> Panic for OnceBufGen<A> {
    fn panic(&mut self) {
        self.stop();
    }
}

/// A generator that loops an audio buffer.
#[derive(Clone, Debug)]
pub struct LoopBufGen<A: Audio> {
    /// The inner buffer.
    buffer: Buffer<A>,

    /// The sample being read.
    index: usize,
}

impl<A: Audio> LoopBufGen<A> {
    /// Initializes a new [`LoopBufGen`].
    #[must_use]
    pub const fn new(buffer: Buffer<A>) -> Self {
        Self { buffer, index: 0 }
    }

    /// Returns a reference to the underlying buffer.
    #[must_use]
    pub const fn buffer(&self) -> &Buffer<A> {
        &self.buffer
    }

    /// Returns a mutable reference to the underlying buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer<A> {
        &mut self.buffer
    }

    /// Returns the inner slice.    
    #[must_use]
    pub fn as_slice(&self) -> &[A] {
        self.buffer.as_slice()
    }

    /// Returns a mutable reference to the inner slice.
    pub fn as_mut_slice(&mut self) -> &mut [A] {
        self.buffer.as_mut_slice()
    }

    /// Returns the number of samples in the buffer.
    #[must_use]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns whether the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl<A: Audio> Signal for LoopBufGen<A> {
    type Sample = A;

    fn get(&self) -> A {
        self.buffer()[self.index]
    }
}

impl<A: Audio> SignalMut for LoopBufGen<A> {
    fn advance(&mut self) {
        crate::mod_inc(self.len(), &mut self.index);
    }

    fn retrigger(&mut self) {
        self.index = 0;
    }
}

impl<A: Audio> Base for LoopBufGen<A> {
    impl_base!();
}
