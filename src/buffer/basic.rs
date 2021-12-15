use crate::buffer::{OOBSError, GeneralBuffer, ReadableBuffer};
use std::mem::transmute;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::sync::Arc;
use parking_lot::Mutex;
use arc_swap::ArcSwap;
use std::cell::RefCell;
use std::rc::Rc;
use std::convert::TryInto;
use std::borrow::{BorrowMut, Borrow};

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
}


impl ReadableBuffer for BasicBuffer {
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
        let mut rdx = self.rdx.clone();
        let mut rdx = rdx.borrow_mut();
        let inner = rdx.into_inner();
        rdx.replace(inner + 1);
        let data_inner: &RefCell<Box<[u8]>> = self.inner.clone().borrow();
        Ok((data_inner.into_inner())[inner - 1])
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
        self.rdx.clone().borrow_mut().replace(reader_index);
    }

    fn get_reader_index(&self) -> usize {
        let rdx: &RefCell<usize> = self.rdx.borrow();
        rdx.into_inner()
    }

    fn readable_bytes(&self) -> usize {
        let rdx: &RefCell<usize> = self.rdx.clone().borrow();
        let inner: &RefCell<Box<[u8]>> = self.inner.clone().borrow();
        inner.into_inner().len() - rdx.into_inner()
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
}

impl ReadableBuffer for TSBasicBuffer {
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
        Ok(self.inner.clone().lock()[rdx])
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
        self.inner.clone().lock().len() - self.rdx.load(Ordering::Acquire)
    }
}

