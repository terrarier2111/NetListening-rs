use crate::buffer::{ReadableBuffer, WritableBuffer, OOBSError};

// TODO: Should we provide a way to access the last few bits which aren't intended to be used?
pub struct VarIntUnsigned(u32);
pub struct VarIntSigned(i32);

pub trait ReadVarIntBufferExtension: ReadableBuffer {

    fn read_var_int_unsigned(&self) -> VarIntUnsigned;

    fn read_var_int_signed(&self) -> VarIntSigned;

}

pub trait WriteVarIntBufferExtension: WritableBuffer {

    fn read_var_int_unsigned(&self, var_int: VarIntUnsigned) -> Option<OOBSError>;

    fn write_var_int_signed(&self, var_int: VarIntSigned) -> Option<OOBSError>;

}