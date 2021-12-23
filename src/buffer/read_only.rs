use std::cell::RefCell;
use std::convert::TryInto;
use std::mem::transmute;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::buffer::utils::IntoRaw;
use crate::buffer::{
    AsSliceArced, AsSliceBoxed, GeneralBuffer, OOBSError, ReadableBuffer, ThreadSafeBuffer,
};

pub struct ReadOnlyBuffer {
    inner: Box<[u8]>,
    rdx: RefCell<usize>,
}

impl ReadOnlyBuffer {
    pub(crate) fn alloc_from_buf(buf: Box<[u8]>) -> Box<Self> {
        Box::from(ReadOnlyBuffer {
            inner: buf,
            rdx: RefCell::new(0),
        })
    }

    pub(crate) fn alloc_sized(size: usize) -> Box<Self> {
        Box::from(ReadOnlyBuffer {
            inner: vec![0; size].into_boxed_slice(),
            rdx: RefCell::new(0),
        })
    }
}

impl AsSliceBoxed for ReadOnlyBuffer {
    fn as_slice(self: Box<Self>) -> Box<[u8]> {
        self.inner
    }
}

impl AsSliceArced for ReadOnlyBuffer {
    fn as_slice(self: Arc<Self>) -> Box<[u8]> {
        todo!()
    }
}

impl GeneralBuffer for ReadOnlyBuffer {}

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

pub struct TSReadOnlyBuffer {
    // ThreadSafeReadOnlyBuffer
    inner: Box<[u8]>,
    rdx: AtomicUsize, // reader index
}

impl TSReadOnlyBuffer {
    pub(crate) fn alloc_from_buf(buf: Box<[u8]>) -> Box<Self> {
        Box::from(Self {
            inner: buf,
            rdx: Default::default(),
        })
    }

    pub(crate) fn alloc_sized(size: usize) -> Box<Self> {
        Box::from(Self {
            inner: vec![0; size].into_boxed_slice(),
            rdx: Default::default(),
        })
    }
}

impl AsSliceBoxed for TSReadOnlyBuffer {
    fn as_slice(self: Box<Self>) -> Box<[u8]> {
        self.inner
    }
}

impl AsSliceArced for TSReadOnlyBuffer {
    fn as_slice(self: Arc<Self>) -> Box<[u8]> {
        todo!()
    }
}

impl GeneralBuffer for TSReadOnlyBuffer {}

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
