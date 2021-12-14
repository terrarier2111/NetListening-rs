use arc_swap::ArcSwap;
use std::sync::Arc;
use parking_lot::Mutex;
use std::mem::MaybeUninit;

// These buffers are inspired by Netty

// TODO: MAYBE use this BufferAllocator ZST in the future
pub trait BufferAllocator<const SIZE: usize> {

    // FIXME: Use this instead of alloc once https://github.com/rust-lang/rust/issues/29661 is closed!
    // type Buffer = [u8; SIZE];

    fn alloc() -> [u8; SIZE];

}

pub type Buffer = Box<dyn RwBuffer>;

pub trait RwBuffer: GeneralBuffer + ReadableBuffer + WritableBuffer {
}

pub trait GeneralBuffer {

    fn alloc_new(size: usize) -> Self where Self: Sized;

}

pub trait ReadableBuffer {

    // TODO: Implement char such that it somewhat works for any language!

    fn read_bool(&self) -> bool;

    fn read_i8(&self) -> i8;

    fn read_i16(&self) -> i16;

    fn read_i32(&self) -> i32;

    fn read_i64(&self) -> i64;

    fn read_f32(&self) -> f32;

    fn read_f64(&self) -> f64;

    fn read_bytes(&self, byte_count: usize) -> Box<[u8]>;

}

pub trait WritableBuffer {

    // TODO: Implement char such that it somewhat works for any language!

    fn write_bool(&self, _: bool) -> Result<(), OOBSError>;

    fn write_i8(&self, _: i8) -> Result<(), OOBSError>;

    fn write_i16(&self, _: i16) -> Result<(), OOBSError>;

    fn write_i32(&self, _: i32) -> Result<(), OOBSError>;

    fn write_i64(&self, _: i64) -> Result<(), OOBSError>;

    fn write_f32(&self, _: f32) -> Result<(), OOBSError>;

    fn write_f64(&self, _: f64) -> Result<(), OOBSError>;

}

pub struct OOBSError(&'static str); // OutOfBufferSpaceError

// pub type Buffer = ArcSwap<Vec<Arc<Box<Mutex<[u8]>>>>>; // TODO: Can this size be inlined into
pub type CSBuffer<const SIZE: usize> = ArcSwap<Vec<Arc<Mutex<[u8; SIZE]>>>>;


pub struct BasicBuffer {

    inner: Arc<Mutex<Box<[u8]>>>, // TODO: Can this size be inlined into
    rdx: usize, // reader index
    wrx: usize, // writer index

}

impl GeneralBuffer for BasicBuffer {
    fn alloc_new(size: usize) -> Self {
        Self {
            inner: Arc::new(Mutex::new(vec![0; size].into_boxed_slice())), // FIXME: Use Box::new_uninit_slice(len).assume_init once it's stabilized!
            rdx: 0,
            wrx: 0,
        }
    }
}

pub struct ComposedBuffer {

    inner: ArcSwap<Vec<Arc<Mutex<[u8]>>>>, // TODO: Can this size be inlined into
    rdx: usize, // reader index
    wrx: usize, // writer index

}
