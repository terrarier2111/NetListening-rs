use crate::utils::DataContainer;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::buffer::Buffer;
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

    fn write(&self, application: Application, buffer: Buffer);

    fn read(application: Application, buffer: Buffer) -> Result<Self, DataTypeReadError> where Self: Sized;

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