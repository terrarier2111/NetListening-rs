use crate::buffer::basic::BasicBuffer;
use crate::buffer::read_only::ReadOnlyBuffer;
use crate::buffer::{
    GeneralBuffer, RBuffer, RWBuffer, RWBufferType, ReadableBuffer, WritableBuffer,
};

// TODO: Maybe try to find a better way to deal with writer indices!

pub fn readonly_buffer(size: usize) -> RBuffer {
    ReadOnlyBuffer::alloc_sized(size)
}

pub fn readonly_buffer_from_buf(buf: Box<[u8]>) -> RBuffer {
    ReadOnlyBuffer::alloc_from_buf(buf)
}

pub fn readonly_buffer_from_raw_buf(buf: RawBuffer) -> RBuffer {
    let (buf, rdx) = buf;
    let result = readonly_buffer_from_buf(buf);
    result.set_reader_index(rdx);
    result
}

pub fn readonly_view<T: RWBufferType>(buffer: Box<T>) -> RBuffer {
    let rdx = buffer.get_reader_index();
    let raw = (buffer.as_slice(), rdx);
    readonly_buffer_from_raw_buf(raw)
}

pub fn rw_buffer(size: usize) -> RWBuffer {
    BasicBuffer::alloc_sized(size)
}

pub fn rw_buffer_from_buf(buf: Box<[u8]>) -> RWBuffer {
    BasicBuffer::alloc_from_buf(buf)
}

pub fn rw_buffer_from_raw_buf(buf: RawBuffer) -> RWBuffer {
    let (buf, rdx) = buf;
    let result = rw_buffer_from_buf(buf);
    result.set_reader_index(rdx);
    result
}

pub fn rw_view<T: ReadableBuffer>(buffer: Box<T>) -> RWBuffer {
    let rdx = buffer.get_reader_index();
    let raw = (buffer.as_slice(), rdx);
    rw_buffer_from_raw_buf(raw)
}

pub trait IntoRaw<T>
where
    T: Sized,
{
    fn into_raw(self) -> T;
}

pub type RawBuffer = (Box<[u8]>, usize); // inner_buffer, reader_index
