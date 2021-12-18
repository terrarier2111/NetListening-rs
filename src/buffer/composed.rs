use parking_lot::Mutex;
use std::sync::Arc;
use arc_swap::ArcSwap;
use std::sync::atomic::AtomicUsize;
use crate::buffer::{GeneralBuffer, ReadableBuffer, OOBSError, WritableBuffer};
use std::rc::Rc;
use std::cell::RefCell;
use crate::buffer::utils::IntoRaw;

pub struct ComposedBuffer {

    inner: Rc<RefCell<Vec<Rc<RefCell<Box<[u8]>>>>>>, // TODO: Can this size be inlined into Mutex?
    rdx: Rc<RefCell<usize>>, // reader index
    wrx: Rc<RefCell<usize>>, // writer index

}

impl GeneralBuffer for ComposedBuffer {
    fn alloc_new(size: usize) -> Self where Self: Sized {
        todo!()
    }

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized {
        todo!()
    }

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized {
        todo!()
    }
}

impl ReadableBuffer for ComposedBuffer {
    fn read_bool(&self) -> Result<bool, OOBSError> {
        todo!()
    }

    fn read_i8(&self) -> Result<i8, OOBSError> {
        todo!()
    }

    fn read_u8(&self) -> Result<u8, OOBSError> {
        todo!()
    }

    fn read_i16(&self) -> Result<i16, OOBSError> {
        todo!()
    }

    fn read_u16(&self) -> Result<u16, OOBSError> {
        todo!()
    }

    fn read_i32(&self) -> Result<i32, OOBSError> {
        todo!()
    }

    fn read_u32(&self) -> Result<u32, OOBSError> {
        todo!()
    }

    fn read_i64(&self) -> Result<i64, OOBSError> {
        todo!()
    }

    fn read_u64(&self) -> Result<u64, OOBSError> {
        todo!()
    }

    fn read_f32(&self) -> Result<f32, OOBSError> {
        todo!()
    }

    fn read_f64(&self) -> Result<f64, OOBSError> {
        todo!()
    }

    fn read_bytes(&self, byte_count: usize) -> Result<Box<[u8]>, OOBSError> {
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

    fn write_u16(&self, _: u16) -> Option<OOBSError> {
        todo!()
    }

    fn write_u32(&self, _: u32) -> Option<OOBSError> {
        todo!()
    }

    fn write_u64(&self, _: u64) -> Option<OOBSError> {
        todo!()
    }

    fn write_bytes(&self, _: &[u8]) -> Option<OOBSError> {
        todo!()
    }

    fn set_writer_index(&self, reader_index: usize) {
        todo!()
    }

    fn get_writer_index(&self) -> usize {
        todo!()
    }

    fn writable_bytes(&self) -> usize {
        todo!()
    }
}

pub struct TSComposedBuffer {

    inner: ArcSwap<Vec<Arc<Mutex<Box<[u8]>>>>>, // TODO: Can this size be inlined into Mutex?
    rdx: AtomicUsize, // reader index
    wrx: AtomicUsize, // writer index

}

impl GeneralBuffer for TSComposedBuffer {
    fn alloc_new(size: usize) -> Self where Self: Sized {
        todo!()
    }

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized {
        todo!()
    }

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized {
        todo!()
    }
}

impl ReadableBuffer for TSComposedBuffer {
    fn read_bool(&self) -> Result<bool, OOBSError> {
        todo!()
    }

    fn read_i8(&self) -> Result<i8, OOBSError> {
        todo!()
    }

    fn read_u8(&self) -> Result<u8, OOBSError> {
        todo!()
    }

    fn read_i16(&self) -> Result<i16, OOBSError> {
        todo!()
    }

    fn read_u16(&self) -> Result<u16, OOBSError> {
        todo!()
    }

    fn read_i32(&self) -> Result<i32, OOBSError> {
        todo!()
    }

    fn read_u32(&self) -> Result<u32, OOBSError> {
        todo!()
    }

    fn read_i64(&self) -> Result<i64, OOBSError> {
        todo!()
    }

    fn read_u64(&self) -> Result<u64, OOBSError> {
        todo!()
    }

    fn read_f32(&self) -> Result<f32, OOBSError> {
        todo!()
    }

    fn read_f64(&self) -> Result<f64, OOBSError> {
        todo!()
    }

    fn read_bytes(&self, byte_count: usize) -> Result<Box<[u8]>, OOBSError> {
        todo!()
    }

    fn read_bytes_into(&self, byte_count: usize, buffer: &mut [u8]) -> Option<OOBSError> {
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

    fn write_u16(&self, _: u16) -> Option<OOBSError> {
        todo!()
    }

    fn write_u32(&self, _: u32) -> Option<OOBSError> {
        todo!()
    }

    fn write_u64(&self, _: u64) -> Option<OOBSError> {
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

    fn writable_bytes(&self) -> usize {
        todo!()
    }
}