pub mod basic;
pub mod composed;
pub mod utils;
pub mod read_only;

use arc_swap::ArcSwap;
use std::sync::Arc;
use parking_lot::Mutex;
use std::mem::{MaybeUninit, transmute};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt::{Display, Formatter};

// These buffers are inspired by Netty

// TODO: MAYBE use this BufferAllocator ZST in the future
pub trait BufferAllocator<const SIZE: usize> {

    // FIXME: Use this instead of alloc once https://github.com/rust-lang/rust/issues/29661 is closed!
    // type Buffer = [u8; SIZE];

    fn alloc() -> [u8; SIZE];

}

pub type Buffer = Box<dyn RwBuffer>;

pub trait RwBuffer: ReadableBuffer + WritableBuffer {
}

pub trait GeneralBuffer {

    fn alloc_new(size: usize) -> Self where Self: Sized;

}

pub trait ReadableBuffer: GeneralBuffer {

    // TODO: Implement char such that it somewhat works for any language!

    fn read_bool(&self) -> Result<bool, OOBSError>; // TODO: Check if it's worth it to replace Result with Option for performance reasons

    fn read_i8(&self) -> Result<i8, OOBSError>;

    fn read_u8(&self) -> Result<u8, OOBSError>;

    fn read_i16(&self) -> Result<i16, OOBSError>;

    fn read_u16(&self) -> Result<u16, OOBSError>;

    fn read_i32(&self) -> Result<i32, OOBSError>;

    fn read_u32(&self) -> Result<u32, OOBSError>;

    fn read_i64(&self) -> Result<i64, OOBSError>;

    fn read_u64(&self) -> Result<u64, OOBSError>;

    fn read_f32(&self) -> Result<f32, OOBSError>;

    fn read_f64(&self) -> Result<f64, OOBSError>;

    fn read_bytes(&self, byte_count: usize) -> Result<Box<[u8]>, OOBSError>;

    fn set_reader_index(&self, reader_index: usize);

    fn get_reader_index(&self) -> usize;

    /// Returns the previous reader index.
    #[inline]
    fn reset_reader_index(&self) {
        self.set_reader_index(0)
    }

    fn readable_bytes(&self) -> usize;

    #[inline]
    fn has_readable_bytes(&self, bytes: usize) -> bool {
        self.readable_bytes() >= bytes
    }

}

pub trait WritableBuffer: GeneralBuffer {

    // TODO: Implement char such that it somewhat works for any language!

    // TODO: Does the output assembly change if i change Option<ERROR> to Result<(), ERROR> ?

    fn write_bool(&self, _: bool) -> Option<OOBSError>;

    fn write_i8(&self, _: i8) -> Option<OOBSError>;

    fn write_u8(&self, _: u8) -> Option<OOBSError>;

    fn write_i16(&self, _: i16) -> Option<OOBSError>;

    fn write_u16(&self, _: u16) -> Option<OOBSError>;

    fn write_i32(&self, _: i32) -> Option<OOBSError>;

    fn write_u32(&self, _: u32) -> Option<OOBSError>;

    fn write_i64(&self, _: i64) -> Option<OOBSError>;

    fn write_u64(&self, _: u64) -> Option<OOBSError>;

    fn write_f32(&self, _: f32) -> Option<OOBSError>;

    fn write_f64(&self, _: f64) -> Option<OOBSError>;

    fn write_bytes(&self, _: Box<[u8]>) -> Option<OOBSError>;

    fn set_writer_index(&self, reader_index: usize);

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

pub trait ThreadSafeBuffer {}

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

// pub type Buffer = ArcSwap<Vec<Arc<Box<Mutex<[u8]>>>>>; // TODO: Can this size be inlined into
pub type CSBuffer<const SIZE: usize> = ArcSwap<Vec<Arc<Mutex<[u8; SIZE]>>>>;