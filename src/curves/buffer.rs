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

use std::{alloc, io, path::Path};

use hound::{SampleFormat, WavReader};

use crate::{prelude::*, sample::WavSample};

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
}

impl<S: Sample> FromIterator<S> for Buffer<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self::from_data(FromIterator::from_iter(iter))
    }
}

impl Buffer<Mono> {
    /// Allocates memory in which to store a buffer of a given size.
    ///
    /// If `length = 0` is passed, the null pointer will be returned.
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

    /// Reads from a `WavReader` into a pointer (returned from [`get_ptr`]).
    ///
    /// This function is generic in `S`. For a non-generic version, see
    /// [`write_ptr`].
    ///
    /// ## Errors
    ///
    /// Will return an error if a sample can't be turned into the specified
    /// type `S`.
    fn write_ptr_gen<S: WavSample>(
        reader: WavReader<io::BufReader<std::fs::File>>,
        ptr: *mut Mono,
    ) -> hound::Result<()> {
        let length = reader.len() as usize;

        // Safety: the number of samples in the iterator equals the size of the
        // allocated memory area.
        for (idx, sample) in reader.into_samples::<S>().enumerate() {
            unsafe {
                debug_assert!(idx < length);
                *ptr.add(idx) = sample?.into_mono();
            }
        }

        Ok(())
    }

    /// Reads from a `WavReader` into a pointer (returned from [`get_ptr`]).
    ///
    /// See also [`write_ptr_gen`].
    ///
    /// ## Errors
    ///
    /// This should not error as long as the WAV file is in a supported format.
    /// See the [module docs](buffer) for a list.
    fn write_ptr(
        reader: WavReader<io::BufReader<std::fs::File>>,
        ptr: *mut Mono,
    ) -> hound::Result<()> {
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
    /// [`write_ptr_gen`] or [`write_ptr`].
    fn from_ptr(length: usize, ptr: *mut Mono) -> Self {
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
    ///
    /// ## Panics
    ///
    /// Panics if the WAV file has more than 1 channel.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(path: P) -> hound::Result<Self> {
        let reader = WavReader::open(path)?;
        assert_eq!(reader.spec().channels, 1, "mono file expected");
        let length = reader.len() as usize;

        let ptr = Self::get_ptr(length);
        Self::write_ptr_gen::<S>(reader, ptr)?;
        Ok(Self::from_ptr(length, ptr))
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
    ///
    /// ## Panics
    ///
    /// Panics if the WAV file has more than 1 channel.
    pub fn from_wav<P: AsRef<Path>>(path: P) -> hound::Result<Self> {
        let reader = WavReader::open(path)?;
        assert_eq!(reader.spec().channels, 1, "mono file expected");
        let length = reader.len() as usize;

        let ptr = Self::get_ptr(length);
        Self::write_ptr(reader, ptr)?;
        Ok(Self::from_ptr(length, ptr))
    }
}

impl Buffer<Stereo> {
    /// Creates a buffer from an initialized pointer, returned from either
    /// [`Buffer::write_ptr_gen`] or [`Buffer::write_ptr`].
    ///
    /// Note that the pointer is made out of (interleaved) `Mono` samples.
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
    ///
    /// ## Panics
    ///
    /// Panics if the WAV file doesn't have 2 channels.
    pub fn from_wav_gen<P: AsRef<Path>, S: WavSample>(path: P) -> hound::Result<Self> {
        let reader = WavReader::open(path)?;
        assert_eq!(reader.spec().channels, 2, "stereo file expected");
        let length = reader.len() as usize;

        // This should be guaranteed by the `WavReader::open` function itself.
        // We don't make this a debug assertion, as safety of what follows
        // depends on this.
        assert_eq!(length % 2, 0);

        let ptr = Buffer::get_ptr(length);
        Buffer::write_ptr_gen::<S>(reader, ptr)?;
        Ok(Self::from_ptr(length, ptr))
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
    ///
    /// ## Panics
    ///
    /// Panics if the WAV file doesn't have 2 channels.
    pub fn from_wav<P: AsRef<Path>>(path: P) -> hound::Result<Self> {
        let reader = WavReader::open(path)?;
        assert_eq!(reader.spec().channels, 2, "stereo file expected");
        let length = reader.len() as usize;

        // This should be guaranteed by the `WavReader::open` function itself.
        // We don't make this a debug assertion, as safety of what follows
        // depends on this.
        assert_eq!(length % 2, 0);

        let ptr = Buffer::get_ptr(length);
        Buffer::write_ptr(reader, ptr)?;
        Ok(Self::from_ptr(length, ptr))
    }
}
