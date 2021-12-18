use crate::utils::DataContainer;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::buffer::{RWBuffer, RBuffer};
use crate::application::Application;

#[derive(Clone)]
pub enum DataType {

    String,
    I8,
    I16,
    I32,
    I64,
    Array(Box<DataType>),
    InternalPayload,

}

pub trait DataTypeMethods {

    fn write(self, application: Application, buffer: RWBuffer);

    fn read(application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized;

}

#[derive(Debug)]
pub struct DataTypeReadError(String);

impl DataTypeReadError {

    pub fn new(msg: String) -> Self {
        Self(msg)
    }

}

impl Display for DataTypeReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Error for DataTypeReadError {}

impl DataTypeMethods for i8 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_i8(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_i8().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for u8 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_u8(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_u8().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for i16 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_i16(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_i16().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for u16 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_u16(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_u16().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for i32 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_i32(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_i32().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for u32 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_u32(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_u32().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for i64 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_i64(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_i64().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for u64 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_u64(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_u64().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for f32 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_f32(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_f32().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for f64 {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_f64(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_f64().unwrap()) // FIXME: Fix this error propagation!
    }
}

impl DataTypeMethods for bool {
    fn write(self, _application: Application, buffer: RWBuffer) {
        buffer.write_bool(self); // FIXME: Fix this error propagation!
    }

    fn read(_application: Application, buffer: RBuffer) -> Result<Self, DataTypeReadError> where Self: Sized {
        Ok(buffer.read_bool().unwrap()) // FIXME: Fix this error propagation!
    }
}