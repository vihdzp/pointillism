//! Defines the [`Buffer`] type for sample buffers.
//!
//! ## Supported WAV formats
//!
//! The [`hound`] library, and pointillism by extension, support only WAV files
//! in the following formats:
//!    
//! - 8-bit integer
//! - 16-bit integer
//! - 24-bit integer
//! - 32-bit integer
//! - 32-bit float

use std::{alloc, fmt::Display, io, path::Path};

use hound::{SampleFormat, WavReader};

use crate::{prelude::*, sample::WavSample};

/// A reader for a WAV file.
pub type WavFileReader = WavReader<io::BufReader<std::fs::File>>;

/// Returns the integer and fractional part of a float.
fn index_frac(val: f64) -> (isize, f64) {
    // Hopefully we never deal with files so large that truncation can occur.
    #[allow(clippy::cast_possible_truncation)]
    (val.floor() as isize, val.fract())
}

/// Linearly interpolates two samples `x0` and `x1`.
///
/// The variable `t` should range between `0` and `1`.
pub fn linear_inter<S: SampleLike>(x0: S, x1: S, t: f64) -> S {
    x0 * (1.0 - t) + x1 * t
}

/// Interpolates cubically between `x1` and `x2`. Makes use of the previous
/// sample `x0` and the next sample `x3`.
///
/// The variable `t` should range between `0` and `1`.
///
/// Adapted from <https://stackoverflow.com/a/1126113/12419072>.
pub fn cubic_inter<S: SampleLike>(x0: S, x1: S, x2: S, x3: S, t: f64) -> S {
    let a0 = x3 - x2 - x0 + x1;
    let a1 = x0 - x1 - a0;
    let a2 = x2 - x0;
    let a3 = x1;

    a0 * (t * t * t) + a1 * (t * t) + a2 * t + a3
}

/// Applies Hermite interpolation between `x1` and `x2`. Makes use of the
/// previous sample `x0` and the next sample `x3`.
///
/// The variable `t` should range between `0` and `1`.
///
/// Adapted from <https://stackoverflow.com/a/72122178/12419072>.
pub fn hermite_inter<S: SampleLike>(x0: S, x1: S, x2: S, x3: S, t: f64) -> S {
    let diff = x1 - x2;
    let c1 = x2 - x0;
    let c3 = x3 - x0 + diff * 3.0;
    let c2 = -(diff * 2.0 + c1 + c3);

    ((c3 * t + c2) * t + c1) * t * 0.5 + x1
}

/// A sample buffer.
#[derive(Clone, Debug, Default)]
pub struct Buffer<S: Sample> {
    /// The data stored by the buffer.
    data: Vec<S>,
}

impl<S: Sample> Buffer<S> {
    /// Initializes a new buffer from data.
    #[must_use]
    pub const fn from_data(data: Vec<S>) -> Self {
        Self { data }
    }

    /// Initializes a new empty buffer.
    #[must_use]
    pub const fn new() -> Self {
        Self::from_data(Vec::new())
    }

    /// Returns the inner slice.    
    #[must_use]
    pub fn as_slice(&self) -> &[S] {
        self.data.as_slice()
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

    /// Returns the time that takes to play this buffer at a standard sample
    /// rate of 44.1 kHz.
    #[must_use]
    pub fn time(&self) -> Time {
        // Any precision loss should be insignificant.
        #[allow(clippy::cast_precision_loss)]
        Time::new_frames(self.data.len() as f64)
    }

    /// Gets an exact sample at a given index.
    ///
    /// We return `0` for any sample outside of the buffer.
    #[must_use]
    pub fn get(&self, index: isize) -> S {
        if index < 0 {
            S::ZERO
        } else {
            // Hopefully this is never an issue.
            #[allow(clippy::cast_sign_loss)]
            self.data.get(index as usize).copied().unwrap_or_default()
        }
    }

    /// Gets an exact sample at a given index, wrapping around to fit the
    /// buffer.
    ///
    /// We return `0` for any sample outside of the buffer.
    #[must_use]
    pub fn get_loop(&self, index: isize) -> S {
        if self.is_empty() {
            S::ZERO
        } else {
            // Hopefully this is never an issue.
            #[allow(clippy::cast_possible_wrap)]
            self.get(index.rem_euclid(self.len() as isize))
        }
    }
}

impl<S: Sample> FromIterator<S> for Buffer<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self::from_data(FromIterator::from_iter(iter))
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Initializes a [`WavFileReader`] with the given path, which can expect a
/// [`Mono`] or [`Stereo`] file.
///
/// ## Errors
///
/// Will error in case of an IO error, or if there is a mismatch in the number
/// of channels.
fn init_reader(path: &Path, expected_mono: bool) -> Result<WavFileReader, Error> {
    let reader = WavReader::open(path)?;
    if (reader.spec().channels == 1) != expected_mono {
        return Err(Error::ChannelMismatch { expected_mono });
    }
    Ok(reader)
}

impl Buffer<Mono> {
    /// Allocates memory in which to store a buffer of a given size. If
    /// `length = 0` is passed, the null pointer will be returned.
    ///
    /// This is useful for two reasons:
    ///
    /// - It serves as an optimization. An audio buffer from a WAV file should
    ///   have no reason to change in size.
    /// - It allows us to transmute an interleaved array of `Mono` samples into
    ///   an array of `Stereo` samples.
    fn get_ptr(length: usize) -> *mut Mono {
        // This must be handled separately, as `alloc::alloc` doesn't allow for
        // an empty layout.
        if length == 0 {
            return std::ptr::null_mut();
        }

        let layout = alloc::Layout::array::<Mono>(length).unwrap();

        // Safety: the layout is nonempty, the alignment is set explicitly.
        #[allow(clippy::cast_ptr_alignment)]
        let ptr = unsafe { alloc::alloc(layout) }.cast::<Mono>();

        if ptr.is_null() {
            alloc::handle_alloc_error(layout);
        }
        ptr
    }

    /// Reads from a `WavReader` into a pointer (returned from
    /// [`Self::get_ptr`]).
    ///
    /// This function is generic in `S`. For a non-generic version, see
    /// [`Self::write_ptr`].
    ///
    /// ## Safety
    ///
    /// All of the samples returned from the iterator must fit exactly in the
    /// allocated memory area.
    ///
    /// ## Errors
    ///
    /// Will return an error if a sample can't be turned into the specified
    /// type `S`.
    unsafe fn write_ptr_gen<S: WavSample>(
        reader: WavFileReader,
        ptr: *mut Mono,
    ) -> hound::Result<()> {
        let length = reader.len() as usize;

        for (idx, sample) in reader.into_samples::<S>().enumerate() {
            // This is safe in debug.
            debug_assert!(idx < length);
            *ptr.add(idx) = sample?.into_mono();
        }

        Ok(())
    }

    /// Reads from a `WavReader` into a pointer (returned from
    /// [`Self::get_ptr`]).
    ///
    /// See also [`Self::write_ptr_gen`].
    ///
    /// ## Safety
    ///
    /// All of the samples returned from the iterator must fit exactly in the
    /// allocated memory area.
    ///
    /// ## Errors
    ///
    /// This should not error as long as the WAV file is in a supported format.
    /// See the [module docs](buffer) for a list.
    unsafe fn write_ptr(reader: WavFileReader, ptr: *mut Mono) -> hound::Result<()> {
        match reader.spec().sample_format {
            SampleFormat::Float => Self::write_ptr_gen::<f32>(reader, ptr),
            SampleFormat::Int => match reader.spec().bits_per_sample {
                8 => Self::write_ptr_gen::<i8>(reader, ptr),
                16 => Self::write_ptr_gen::<i16>(reader, ptr),
                24 | 32 => Self::write_ptr_gen::<i32>(reader, ptr),
                _ => Err(hound::Error::Unsupported),
            },
        }
    }

    /// Creates a buffer from an initialized pointer, returned from either
    /// [`Self::write_ptr_gen`] or [`Self::write_ptr`].
    ///
    /// ## Safety
    ///
    /// If `ptr` is not null, the memory area must be initialized, and have the
    /// exact length (in [`Mono`] samples) passed as an argument.
    unsafe fn from_ptr(length: usize, ptr: *mut Mono) -> Self {
        if ptr.is_null() {
            Buffer::new()
        } else {
            Buffer::from_data(unsafe { Vec::from_raw_parts(ptr, length, length) })
        }
    }

    /// Creates a [`Mono`] buffer from a wav file, with a given [`WavSample`]
    /// format.
    ///
    /// See [`Self::from_wav`] for a non-generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The read samples can't be converted into the specified type `S`.
    /// - The WAV format is unsupported (see the [module docs](buffer)).
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
    /// - The WAV format is unsupported (see the [module docs](buffer)).
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
    /// Creates a buffer from an initialized pointer, returned from either
    /// [`Buffer::write_ptr_gen`] or [`Buffer::write_ptr`].
    ///
    /// This pointer should point to memory consisting of an even number of
    /// interleaved `Mono` samples.
    ///
    /// ## Safety
    ///
    /// If `ptr` is not null, the memory area must be initialized, and have the
    /// exact length (in [`Mono`] samples) passed as an argument.
    ///
    /// In particular, `length` must be even.
    fn from_ptr(length: usize, ptr: *mut Mono) -> Self {
        if ptr.is_null() {
            Buffer::new()
        } else {
            Buffer::from_data(unsafe {
                Vec::from_raw_parts(ptr.cast::<Stereo>(), length / 2, length / 2)
            })
        }
    }

    /// Creates a [`Stereo`] buffer from a wav file, with a given [`WavSample`]
    /// format.
    ///
    /// See [`Self::from_wav`] for a non-generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The read samples can't be converted into the specified type `S`.
    /// - The WAV format is unsupported (see the [module docs](buffer)).
    /// - Some IO error related to opening the file.
    /// - The WAV file doesn't have 2 channels.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(path: P) -> Result<Self, Error> {
        let reader = init_reader(path.as_ref(), false)?;
        let length = reader.len() as usize;
        let ptr = Buffer::get_ptr(length);

        // This should be guaranteed by the `WavReader::open` function itself.
        // We don't make this a debug assertion, as safety of what follows
        // depends on this.
        debug_assert_eq!(length % 2, 0);

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
    /// - The WAV format is unsupported (see the [module docs](buffer)).
    /// - Some IO error related to opening the file.
    /// - The WAV file doesn't have 2 channels.
    pub fn from_wav<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let reader = init_reader(path.as_ref(), false)?;
        let length = reader.len() as usize;
        let ptr = Buffer::get_ptr(length);

        // This should be guaranteed by the `WavReader::open` function itself.
        debug_assert_eq!(length % 2, 0);

        // Safety: the memory area has the correct length.
        unsafe {
            Buffer::write_ptr(reader, ptr)?;
            Ok(Self::from_ptr(length, ptr))
        }
    }
}

/// Represents a mode of interpolating between samples.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub enum Interpolate {
    /// Drop-sample interpolation, alias "take the previous sample".
    ///
    /// This is not recommended if fidelity is the intended result.
    Drop,

    /// Linear interpolation between a sample and the next.
    #[default]
    Linear,
    /// Linear interpolation between a sample and the next. Loops back to the
    /// beginning.
    LoopLinear,

    /// Cubic interpolation between a sample, the previous, and the next two.
    Cubic,
    /// Cubic interpolation between a sample, the previous, and the next two.
    /// Loops back to the beginning.
    LoopCubic,

    /// Hermite interpolation between a sample, the previous, and the next two.
    Hermite,
    /// Hermite interpolation between a sample, the previous, and the next two.
    /// Loops back to the beginning.
    LoopHermite,
}

/// A sample curve read from a buffer.
pub struct BufCurve<S: Sample> {
    /// The inner buffer.
    buffer: Buffer<S>,

    /// The interpolation mode.
    inter: Interpolate,
}

impl<S: Sample> BufCurve<S> {
    /// Initializes a new [`BufCurve`].
    #[must_use]
    pub const fn new(buffer: Buffer<S>, inter: Interpolate) -> Self {
        Self { buffer, inter }
    }

    /// Returns a reference to the underlying buffer.
    #[must_use]
    pub const fn buffer(&self) -> &Buffer<S> {
        &self.buffer
    }

    /// Convenience method for calling [`Buffer::get`] on the inner buffer.
    #[must_use]
    pub fn get(&self, index: isize) -> S {
        self.buffer().get(index)
    }

    /// Convenience method for calling [`Buffer::get_loop`] on the inner buffer.
    #[must_use]
    pub fn get_loop(&self, index: isize) -> S {
        self.buffer().get_loop(index)
    }

    /// Gets a sample from the buffer via the given interpolation mode.
    #[must_use]
    pub fn get_inter(&self, val: f64) -> S {
        let (index, t) = index_frac(val);

        match self.inter {
            Interpolate::Drop => self.buffer().get(index),

            Interpolate::Linear => linear_inter(self.get(index), self.get(index + 1), t),
            Interpolate::LoopLinear => {
                linear_inter(self.get_loop(index), self.get_loop(index + 1), t)
            }

            Interpolate::Cubic => cubic_inter(
                self.get(index - 1),
                self.get(index),
                self.get(index + 1),
                self.get(index + 2),
                t,
            ),
            Interpolate::LoopCubic => cubic_inter(
                self.get_loop(index - 1),
                self.get_loop(index),
                self.get_loop(index + 1),
                self.get_loop(index + 2),
                t,
            ),

            Interpolate::Hermite => hermite_inter(
                self.get(index - 1),
                self.get(index),
                self.get(index + 1),
                self.get(index + 2),
                t,
            ),
            Interpolate::LoopHermite => hermite_inter(
                self.get_loop(index - 1),
                self.get_loop(index),
                self.get_loop(index + 1),
                self.get_loop(index + 2),
                t,
            ),
        }
    }
}

impl BufCurve<Mono> {
    /// Creates a [`Mono`] buffer from a wav file, with a given [`WavSample`]
    /// format.
    ///
    /// There are two different implementations of this method for `Mono` and
    /// `Stereo` buffers, so you'll often have to specify this type explicitly.
    ///
    /// See [`Self::from_wav`] for a non-generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The read samples can't be converted into the specified type `S`.
    /// - The WAV format is unsupported (see the [module docs](buffer)).
    /// - Some IO error related to opening the file.
    /// - The WAV file has more than one channel.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(
        path: P,
        inter: Interpolate,
    ) -> Result<Self, Error> {
        Buffer::<Mono>::from_wav_gen::<P, S>(path).map(|buf| Self::new(buf, inter))
    }

    /// Creates a [`Mono`] buffer from a wav file.
    ///
    /// There are two different implementations of this method for `Mono` and
    /// `Stereo` buffers, so you'll often have to specify this type explicitly.
    ///
    /// See [`Self::from_wav_gen`] for a generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The WAV format is unsupported (see the [module docs](buffer)).
    /// - Some IO error related to opening the file.
    /// - The WAV file has more than one channel.
    pub fn from_wav<P: AsRef<Path>>(path: P, inter: Interpolate) -> Result<Self, Error> {
        Buffer::<Mono>::from_wav(path).map(|buf| Self::new(buf, inter))
    }
}

impl BufCurve<Stereo> {
    /// Creates a [`Stereo`] buffer from a wav file, with a given [`WavSample`]
    /// format.
    ///
    /// There are two different implementations of this method for `Mono` and
    /// `Stereo` buffers, so you'll often have to specify this type explicitly.
    ///
    /// See [`Self::from_wav`] for a non-generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The read samples can't be converted into the specified type `S`.
    /// - The WAV format is unsupported (see the [module docs](buffer)).
    /// - Some IO error related to opening the file.
    /// - The WAV file doesn't have 2 channels.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(
        path: P,
        inter: Interpolate,
    ) -> Result<Self, Error> {
        Buffer::<Stereo>::from_wav_gen::<P, S>(path).map(|buf| Self::new(buf, inter))
    }

    /// Creates a [`Stereo`] buffer from a wav file.
    ///
    /// There are two different implementations of this method for `Mono` and
    /// `Stereo` buffers, so you'll often have to specify this type explicitly.
    ///
    /// See [`Self::from_wav_gen`] for a generic version.
    ///
    /// ## Errors
    ///
    /// This can error for various possible reasons:
    ///
    /// - The WAV format is unsupported (see the [module docs](buffer)).
    /// - Some IO error related to opening the file.
    /// - The WAV file doesn't have 2 channels.
    pub fn from_wav<P: AsRef<Path>>(path: P, inter: Interpolate) -> Result<Self, Error> {
        Buffer::<Stereo>::from_wav(path).map(|buf| Self::new(buf, inter))
    }
}

impl<S: Sample> Map for BufCurve<S> {
    type Input = f64;
    type Output = S;

    fn eval(&self, val: f64) -> S {
        // Any precision loss is hopefully insignificant.
        #[allow(clippy::cast_precision_loss)]
        self.get_inter(val * self.buffer().len() as f64)
    }
}
