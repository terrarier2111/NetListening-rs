use crate::buffer::{OOBSError, ReadableBuffer, GeneralBuffer};
use std::mem::transmute;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::cell::RefCell;
use std::convert::TryInto;

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
}

impl ReadableBuffer for ReadOnlyBuffer {
    fn read_bool(&self) -> Result<bool, OOBSError> {
        self.read_u8().map(|x| x == 1)
    }

    fn read_i8(&self) -> Result<i8, OOBSError> {
        self.read_u8().map(|x| unsafe { transmute::<u8, i8>(x) })
    }

    fn read_u8(&self) -> Result<u8, OOBSError> {
        if !self.has_readable_bytes(1) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        let rdx = *self.rdx.borrow();
        *self.rdx.borrow_mut() += 1;
        Ok(self.inner[rdx])
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
        todo!()
    }

    fn set_reader_index(&self, reader_index: usize) {
        *self.rdx.borrow_mut() = reader_index;
    }

    fn get_reader_index(&self) -> usize {
        *self.rdx.borrow()
    }

    fn readable_bytes(&self) -> usize {
        self.inner.len() - *self.rdx.borrow()
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
}

impl ReadableBuffer for TSReadOnlyBuffer {
    fn read_bool(&self) -> Result<bool, OOBSError> {
        return self.read_u8().map(|x| x == 1);
    }

    fn read_i8(&self) -> Result<i8, OOBSError> {
        self.read_u8().map(|x| unsafe { transmute::<u8, i8>(x) })
    }

    fn read_u8(&self) -> Result<u8, OOBSError> {
        if !self.has_readable_bytes(1) {
            return Err(OOBSError::new("No buffer space available!".to_string()));
        }
        let rdx = self.rdx.load(Ordering::Acquire);
        self.rdx.store(rdx + 1, Ordering::Release);
        Ok(self.inner[rdx])
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

    fn set_reader_index(&self, reader_index: usize) {
        self.rdx.store(reader_index, Ordering::Release)
    }

    fn get_reader_index(&self) -> usize {
        self.rdx.load(Ordering::Acquire)
    }

    fn readable_bytes(&self) -> usize {
        self.inner.len() - self.rdx.load(Ordering::Acquire)
    }
}