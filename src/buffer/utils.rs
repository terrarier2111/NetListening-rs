use crate::buffer::{ReadableBuffer, WritableBuffer, RWBuffer, RWBufferType, RBuffer, GeneralBuffer};
use crate::buffer::read_only::ReadOnlyBuffer;
use crate::buffer::basic::BasicBuffer;

// TODO: Maybe try to find a better way to deal with writer indices!

pub fn readonly_buffer(size: usize) -> RBuffer {
    Box::new(ReadOnlyBuffer::alloc_new(size))
}

pub fn readonly_buffer_from_buf(buf: Box<[u8]>) -> RBuffer {
    Box::new(ReadOnlyBuffer::alloc_new_from_buf(buf))
}

pub fn readonly_buffer_from_raw_buf(buf: RawBuffer) -> RBuffer {
    let (buf, rdx) = buf;
    let result = readonly_buffer_from_buf(buf);
    result.set_reader_index(rdx);
    result
}

pub fn readonly_view<T: RWBufferType>(buffer: Box<T>) -> RBuffer {
    let rdx = buffer.get_reader_index();
    let raw = (buffer.raw_contained_bytes(), rdx);
    readonly_buffer_from_raw_buf(raw)
}

pub fn rw_buffer(size: usize) -> RWBuffer {
    Box::new(BasicBuffer::alloc_new(size))
}

pub fn rw_buffer_from_buf(buf: Box<[u8]>) -> RWBuffer {
    Box::new(BasicBuffer::alloc_new_from_buf(buf))
}

pub fn rw_buffer_from_raw_buf(buf: RawBuffer) -> RWBuffer {
    let (buf, rdx) = buf;
    let result = rw_buffer_from_buf(buf);
    result.set_reader_index(rdx);
    result
}

pub fn rw_view<T: ReadableBuffer>(buffer: Box<T>) -> RWBuffer {
    let rdx = buffer.get_reader_index();
    let raw = (buffer.raw_contained_bytes(), rdx);
    rw_buffer_from_raw_buf(raw)
}

pub trait IntoRaw<T> where T: Sized {
    fn into_raw(self) -> T;
}

pub type RawBuffer = (Box<[u8]>, usize); // inner_buffer, reader_index