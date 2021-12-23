use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use arc_swap::ArcSwap;
use parking_lot::Mutex;

use crate::buffer::utils::IntoRaw;
use crate::buffer::{
    AsSliceArced, AsSliceBoxed, GeneralBuffer, OOBSError, ReadableBuffer, WritableBuffer,
};

pub struct ComposedBuffer {
    inner: Rc<RefCell<Vec<Rc<RefCell<Box<[u8]>>>>>>, // TODO: Can this size be inlined into Mutex?
    rdx: RefCell<usize>,                             // reader index
    wrx: RefCell<usize>,                             // writer index
}

impl ComposedBuffer {
    fn alloc_from_buf(buf: Box<[u8]>) -> Arc<Self> {
        todo!()
    }

    fn alloc_sized(size: usize) -> Arc<Self> {
        todo!()
    }
}

impl AsSliceBoxed for ComposedBuffer {
    fn as_slice(self: Box<Self>) -> Box<[u8]> {
        todo!()
    }
}

impl AsSliceArced for ComposedBuffer {
    fn as_slice(self: Arc<Self>) -> Box<[u8]> {
        todo!()
    }
}

impl GeneralBuffer for ComposedBuffer {}

impl ReadableBuffer for ComposedBuffer {
    fn read_u8(&self) -> Result<u8, OOBSError> {
        todo!()
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
        todo!()
    }
}

impl WritableBuffer for ComposedBuffer {
    fn write_u8(&self, _: u8) -> Option<OOBSError> {
        todo!()
    }

    fn write_bytes(&self, _: &[u8]) -> Option<OOBSError> {
        todo!()
    }

    fn set_writer_index(&self, writer_index: usize) {
        let mut wrx = self.wrx.borrow_mut();
        *wrx = writer_index;
    }

    fn get_writer_index(&self) -> usize {
        *self.rdx.borrow()
    }

    fn writable_bytes(&self) -> usize {
        self.contained_bytes() - self.get_writer_index()
    }
}

pub struct TSComposedBuffer {
    inner: ArcSwap<Vec<Arc<Mutex<Box<[u8]>>>>>, // TODO: Can this size be inlined into Mutex?
    rdx: AtomicUsize,                           // reader index
    wrx: AtomicUsize,                           // writer index
}

impl TSComposedBuffer {
    fn alloc_from_buf(buf: Box<[u8]>) -> Box<Self> {
        todo!()
    }

    fn alloc_sized(size: usize) -> Box<Self> {
        todo!()
    }
}

impl AsSliceBoxed for TSComposedBuffer {
    fn as_slice(self: Box<Self>) -> Box<[u8]> {
        todo!()
    }
}

impl AsSliceArced for TSComposedBuffer {
    fn as_slice(self: Arc<Self>) -> Box<[u8]> {
        todo!()
    }
}

impl GeneralBuffer for TSComposedBuffer {}

impl ReadableBuffer for TSComposedBuffer {
    fn read_u8(&self) -> Result<u8, OOBSError> {
        todo!()
    }

    fn read_bytes_into(&self, byte_count: usize, buf: &mut [u8]) -> Option<OOBSError> {
        todo!()
    }

    fn set_reader_index(&self, reader_index: usize) {
        todo!()
    }

    fn get_reader_index(&self) -> usize {
        todo!()
    }

    fn contained_bytes(&self) -> usize {
        todo!()
    }
}

impl WritableBuffer for TSComposedBuffer {
    fn write_u8(&self, _: u8) -> Option<OOBSError> {
        todo!()
    }

    fn write_bytes(&self, _: &[u8]) -> Option<OOBSError> {
        todo!()
    }

    fn set_writer_index(&self, writer_index: usize) {
        todo!()
    }

    fn get_writer_index(&self) -> usize {
        todo!()
    }

    #[inline]
    fn writable_bytes(&self) -> usize {
        self.contained_bytes() - self.get_writer_index()
    }
}
