pub mod basic;
pub mod composed;
pub mod utils;
pub mod read_only;
pub mod varint;

use arc_swap::ArcSwap;
use std::sync::Arc;
use parking_lot::Mutex;
use std::mem::{MaybeUninit, transmute};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt::{Display, Formatter};
use std::io::{Read, ErrorKind};
use std::error::Error;
use std::convert::TryInto;
use crate::buffer::utils::{IntoRaw, RawBuffer};
use std::ops::{DerefMut, Deref};

// These buffers are inspired by Netty

// TODO: MAYBE use this BufferAllocator ZST in the future
pub trait BufferAllocator<const SIZE: usize> {

    // FIXME: Use this instead of alloc once https://github.com/rust-lang/rust/issues/29661 is closed!
    // type Buffer = [u8; SIZE];

    fn alloc() -> [u8; SIZE];

}

pub type RWBuffer = Box<dyn RWBufferType>;
pub type RBuffer = Box<dyn ReadableBuffer>;

pub trait RWBufferType: ReadableBuffer + WritableBuffer {
}

pub trait GeneralBuffer {

    fn alloc_new(size: usize) -> Self where Self: Sized;

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized;

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized;

}

impl<T> RWBufferType for T where T: WritableBuffer + ReadableBuffer {}

pub trait NoWriteBuffer: GeneralBuffer {}

pub trait ReadableBuffer: GeneralBuffer {

    // TODO: Implement char such that it somewhat works for any language!

    fn read_bool(&self) -> Result<bool, OOBSError> { // TODO: Check if it's worth it to replace Result with Option for performance reasons
        return self.read_u8().map(|x| x == 1);
    }

    fn read_i8(&self) -> Result<i8, OOBSError> {
        self.read_u8().map(|x| unsafe { transmute::<u8, i8>(x) })
    }

    fn read_u8(&self) -> Result<u8, OOBSError>;

    fn read_i16(&self) -> Result<i16, OOBSError> {
        self.read_u16().map(|x| unsafe { transmute::<u16, i16>(x) })
    }

    fn read_u16(&self) -> Result<u16, OOBSError> {
        self.read_bytes(2).map(|x| u16::from_be_bytes((*x).try_into().unwrap()))
    }

    fn read_i32(&self) -> Result<i32, OOBSError> {
        self.read_u32().map(|x| unsafe { transmute::<u32, i32>(x) })
    }

    fn read_u32(&self) -> Result<u32, OOBSError> {
        self.read_bytes(4).map(|x| u32::from_be_bytes((*x).try_into().unwrap()))
    }

    fn read_i64(&self) -> Result<i64, OOBSError> {
        self.read_u64().map(|x| unsafe { transmute::<u64, i64>(x) })
    }

    fn read_u64(&self) -> Result<u64, OOBSError> {
        self.read_bytes(8).map(|x| u64::from_be_bytes((*x).try_into().unwrap()))
    }

    fn read_f32(&self) -> Result<f32, OOBSError> {
        self.read_u32().map(|x| unsafe { transmute::<u32, f32>(x) })
    }

    fn read_f64(&self) -> Result<f64, OOBSError> {
        self.read_u64().map(|x| unsafe { transmute::<u64, f64>(x) })
    }

    fn read_bytes(&self, byte_count: usize) -> Result<Box<[u8]>, OOBSError>;

    fn read_bytes_into(&self, byte_count: usize, buffer: &mut [u8]) -> Option<OOBSError>;

    fn set_reader_index(&self, reader_index: usize);

    fn get_reader_index(&self) -> usize;

    /// Returns the previous reader index.
    #[inline]
    fn reset_reader_index(&self) {
        self.set_reader_index(0)
    }

    #[inline]
    fn readable_bytes(&self) -> usize {
        self.contained_bytes() - self.get_reader_index()
    }

    #[inline]
    fn has_readable_bytes(&self, bytes: usize) -> bool {
        self.readable_bytes() >= bytes
    }

    fn contained_bytes(&self) -> usize;

}

pub trait WritableBuffer: GeneralBuffer {

    // TODO: Implement char such that it somewhat works for any language!

    // TODO: Does the output assembly change if i change Option<ERROR> to Result<(), ERROR> ?

    #[inline]
    fn write_bool(&self, x: bool) -> Option<OOBSError> {
        self.write_u8(unsafe { transmute::<bool, u8>(x) })
    }

    #[inline]
    fn write_i8(&self, x: i8) -> Option<OOBSError> {
        self.write_u8(unsafe { transmute::<i8, u8>(x) })
    }

    fn write_u8(&self, _: u8) -> Option<OOBSError>;

    #[inline]
    fn write_i16(&self, x: i16) -> Option<OOBSError> {
        self.write_u16(unsafe { transmute::<i16, u16>(x) })
    }

    fn write_u16(&self, _: u16) -> Option<OOBSError>;

    #[inline]
    fn write_i32(&self, x: i32) -> Option<OOBSError> {
        self.write_u32(unsafe { transmute::<i32, u32>(x) })
    }

    fn write_u32(&self, _: u32) -> Option<OOBSError>;

    #[inline]
    fn write_i64(&self, x: i64) -> Option<OOBSError> {
        self.write_u64(unsafe { transmute::<i64, u64>(x) })
    }

    fn write_u64(&self, x: u64) -> Option<OOBSError> {
        self.write_bytes(u64::to_be_bytes(x).as_slice())
    }

    #[inline]
    fn write_f32(&self, x: f32) -> Option<OOBSError> {
        self.write_u32(unsafe { transmute::<f32, u32>(x) })
    }

    #[inline]
    fn write_f64(&self, x: f64) -> Option<OOBSError> {
        self.write_u64(unsafe { transmute::<f64, u64>(x) })
    }

    fn write_bytes(&self, _: &[u8]) -> Option<OOBSError>;

    fn set_writer_index(&self, writer_index: usize);

    fn get_writer_index(&self) -> usize;

    #[inline]
    fn reset_writer_index(&self) {
        self.set_writer_index(0)
    }

    fn writable_bytes(&self) -> usize;

    #[inline]
    fn has_writable_bytes(&self, bytes: usize) -> bool {
        self.writable_bytes() >= bytes
    }

}

pub trait ThreadSafeBuffer: GeneralBuffer + Send + Sync {}

#[derive(Debug)]
pub struct OOBSError(String); // OutOfBufferSpaceError

impl OOBSError {

    pub fn new(msg: String) -> Self {
        Self(msg)
    }

}

impl Display for OOBSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug, Default)]
pub struct NoReadableBytesError();

impl Display for NoReadableBytesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("There are no bytes available in the buffer which could be read.")
    }
}

impl Error for NoReadableBytesError {}

// pub type Buffer = ArcSwap<Vec<Arc<Box<Mutex<[u8]>>>>>; // TODO: Can this size be inlined into
pub type CSBuffer<const SIZE: usize> = ArcSwap<Vec<Arc<Mutex<[u8; SIZE]>>>>;

impl Read for dyn ReadableBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let readable = self.readable_bytes();
        if readable < 1 {
            return std::io::Result::Err(std::io::Error::new(ErrorKind::UnexpectedEof, NoReadableBytesError::default()));
        }
        let _result = self.read_bytes_into(readable, buf);
        return Ok(readable);
    }
}