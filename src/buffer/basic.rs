use crate::buffer::{OOBSError, GeneralBuffer, ReadableBuffer, WritableBuffer, RWBufferType, ThreadSafeBuffer};
use std::mem::transmute;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;
use parking_lot::Mutex;
use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::TryInto;
use crate::buffer::utils::{IntoRaw, RawBuffer};
use std::io::Read;

pub struct BasicBuffer {

    inner: Rc<RefCell<Box<[u8]>>>,
    rdx: Rc<RefCell<usize>>,  // reader index
    wrx: Rc<RefCell<usize>>,  // writer index
    size: usize, // the size of the currently allocated space (used to bypass the arc and mutex)

}

impl GeneralBuffer for BasicBuffer {
    fn alloc_new(size: usize) -> Self {
        Self {
            inner: Rc::new(RefCell::new(vec![0; size].into_boxed_slice())),
            rdx: Default::default(),
            wrx: Default::default(),
            size,
        }
    }

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized {
        let size = buf.len();
        Self {
            inner: Rc::new(RefCell::new(buf)),
            rdx: Default::default(),
            wrx: Default::default(),
            size,
        }
    }

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized {
        let tmp = self.inner.clone();
        tmp.take()
    }
}


impl ReadableBuffer for BasicBuffer {
    fn read_bool(&self) -> Result<bool, OOBSError> {
        return self.read_u8().map(|x| x == 1);
    }

    fn read_i8(&self) -> Result<i8, OOBSError> {
        self.read_u8().map(|x| unsafe { transmute::<u8, i8>(x) })
    }

    fn read_u8(&self) -> Result<u8, OOBSError> {
        /*if !self.has_readable_bytes(1) { // TODO: Maybe add a way to enable this with some constant! - generic type parameters on the trait methods don't work sadly :(
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }*/
        let mut rdx = self.rdx.clone();
        let mut rdx = rdx.borrow_mut();
        *rdx += 1;
        let inner = self.inner.clone();
        let inner = inner.borrow();
        Ok((*inner)[*rdx - 1])
    }

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

    fn read_bytes(&self, byte_count: usize) -> Result<Box<[u8]>, OOBSError> {
        if !self.has_readable_bytes(byte_count) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        todo!()
    }

    fn read_bytes_into(&self, byte_count: usize, buffer: &mut [u8]) -> Option<OOBSError> {
        todo!()
    }

    #[inline]
    fn set_reader_index(&self, reader_index: usize) {
        *self.rdx.clone().borrow_mut() = reader_index;
    }

    #[inline]
    fn get_reader_index(&self) -> usize {
        *self.rdx.borrow()
    }

    #[inline]
    fn contained_bytes(&self) -> usize {
        self.inner.borrow().len()
    }
}

impl WritableBuffer for BasicBuffer {
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

pub struct TSBasicBuffer {

    inner: Arc<Mutex<Box<[u8]>>>, // TODO: Can this size be inlined into Mutex?
    rdx: AtomicUsize,  // reader index
    wrx: AtomicUsize,  // writer index
    size: usize, // the size of the currently allocated space (used to bypass the arc and mutex)

}

impl GeneralBuffer for TSBasicBuffer {
    fn alloc_new(size: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(vec![0; size].into_boxed_slice())), // FIXME: Use Box::new_uninit_slice(len).assume_init once it's stabilized!
            rdx: Default::default(),
            wrx: Default::default(),
            size,
        }
    }

    fn alloc_new_from_buf(buf: Box<[u8]>) -> Self where Self: Sized {
        let size = buf.len();
        Self {
            inner: Arc::new(Mutex::new(buf)), // FIXME: Use Box::new_uninit_slice(len).assume_init once it's stabilized!
            rdx: Default::default(),
            wrx: Default::default(),
            size,
        }
    }

    fn raw_contained_bytes(self) -> Box<[u8]> where Self: Sized {
        // self.inner.into_inner()
        todo!()
    }
}

impl ReadableBuffer for TSBasicBuffer {
    fn read_u8(&self) -> Result<u8, OOBSError> {
        if !self.has_readable_bytes(1) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        let rdx = self.rdx.load(Ordering::Acquire);
        self.rdx.store(rdx + 1, Ordering::Release);
        Ok(self.inner.clone().lock()[rdx])
    }

    fn read_bytes(&self, byte_count: usize) -> Result<Box<[u8]>, OOBSError> {
        if !self.has_readable_bytes(byte_count) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        todo!()
    }

    fn read_bytes_into(&self, byte_count: usize, buffer: &mut [u8]) -> Option<OOBSError> {
        todo!()
    }

    #[inline]
    fn set_reader_index(&self, reader_index: usize) {
        self.rdx.store(reader_index, Ordering::Release)
    }

    #[inline]
    fn get_reader_index(&self) -> usize {
        self.rdx.load(Ordering::Acquire)
    }

    #[inline]
    fn contained_bytes(&self) -> usize {
        self.inner.clone().lock().len()
    }
}

impl WritableBuffer for TSBasicBuffer {
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
        self.wrx.store(writer_index, Ordering::Release)
    }

    fn get_writer_index(&self) -> usize {
        self.wrx.load(Ordering::Acquire)
    }

    fn writable_bytes(&self) -> usize {
        todo!()
    }
}

unsafe impl Send for TSBasicBuffer {}
unsafe impl Sync for TSBasicBuffer {}
impl ThreadSafeBuffer for TSBasicBuffer {}