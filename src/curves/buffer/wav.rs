//! Loads a [`Buffer`] from a WAV file.
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
//!
//! ## Example
//!
//! ```
//! # use crate::prelude::*;
//! const FILENAME: &str = "examples/buffer.wav";
//! 
//! // Creates some dummy wave file. In this case, a 440 Hz sine wave for 1s.
//! pointillism::create_from_sgn(
//!     FILENAME,
//!     Time::from_raw_default(RawTime::SEC),
//!     SampleRate::default(),
//!     LoopGen::<Mono, Sin>::default(),
//! )
//! .expect("IO error!");
//!
//! // Read back the file, stretch it to 3 seconds.
//! //
//! // This lowers the pitch, and may introduce some artifacts depending on the interpolation method.
//! const FACTOR: f64 = 3.0;
//! let buf_sgn = OnceBufGen::new(Buffer::<Mono>::from_wav(FILENAME).unwrap());
//! let time = buf_sgn.buffer().time();
//!
//! // We can change the interpolation method here.
//! let sgn = Stretch::new_drop(buf_sgn, 1.0 / FACTOR);
//! pointillism::create_from_sgn(FILENAME, time * FACTOR, SAMPLE_RATE, sgn).unwrap();
//! ```

use crate::{prelude::*, sample::WavSample};
use std::path::Path;

/// A reader for a WAV file.
pub type WavFileReader = hound::WavReader<std::io::BufReader<std::fs::File>>;

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
