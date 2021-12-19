use std::cell::RefCell;
use std::convert::TryInto;
use std::mem::transmute;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::buffer::{GeneralBuffer, OOBSError, ReadableBuffer, ThreadSafeBuffer};
use crate::buffer::utils::IntoRaw;

pub struct ReadOnlyBuffer {

    inner: Box<[u8]>,
    rdx: RefCell<usize>,

}

impl GeneralBuffer for ReadOnlyBuffer {
    fn alloc_new(size: usize) -> Self where Self: Sized {
        Self {
            inner: vec![0; size].into_boxed_slice(),
            rdx: RefCell::new(0),
        }
    }

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized {
        Self {
            inner: buf,
            rdx: RefCell::new(0),
        }
    }

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized {
        self.inner
    }
}

impl ReadableBuffer for ReadOnlyBuffer {
    fn read_u8(&self) -> Result<u8, OOBSError> {
        if !self.has_readable_bytes(1) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        let rdx = *self.rdx.borrow();
        *self.rdx.borrow_mut() += 1;
        Ok(self.inner[rdx])
    }

    fn read_bytes_into(&self, byte_count: usize, buffer: &mut [u8]) -> Option<OOBSError> {
        todo!()
    }

    fn set_reader_index(&self, reader_index: usize) {
        *self.rdx.borrow_mut() = reader_index;
    }

    fn get_reader_index(&self) -> usize {
        *self.rdx.borrow()
    }

    fn contained_bytes(&self) -> usize {
        self.inner.len()
    }
}

pub struct TSReadOnlyBuffer { // ThreadSafeReadOnlyBuffer

    inner: Box<[u8]>,
    rdx: AtomicUsize, // reader index

}

impl GeneralBuffer for TSReadOnlyBuffer {
    fn alloc_new(size: usize) -> Self where Self: Sized {
        Self {
            inner: vec![0; size].into_boxed_slice(),
            rdx: Default::default(),
        }
    }

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized {
        Self {
            inner: buf,
            rdx: Default::default(),
        }
    }

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized {
        self.inner
    }
}

impl ReadableBuffer for TSReadOnlyBuffer {
    fn read_u8(&self) -> Result<u8, OOBSError> {
        if !self.has_readable_bytes(1) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        let rdx = self.rdx.load(Ordering::Acquire);
        self.rdx.store(rdx + 1, Ordering::Release);
        Ok(self.inner[rdx])
    }

    fn read_bytes_into(&self, byte_count: usize, buffer: &mut [u8]) -> Option<OOBSError> {
        todo!()
    }

    fn set_reader_index(&self, reader_index: usize) {
        self.rdx.store(reader_index, Ordering::Release)
    }

    fn get_reader_index(&self) -> usize {
        self.rdx.load(Ordering::Acquire)
    }

    #[inline]
    fn contained_bytes(&self) -> usize {
        self.inner.len()
    }
}

unsafe impl Send for TSReadOnlyBuffer {}
unsafe impl Sync for TSReadOnlyBuffer {}
impl ThreadSafeBuffer for TSReadOnlyBuffer {}