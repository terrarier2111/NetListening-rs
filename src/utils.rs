use std::any::{Any, TypeId};

use crate::data_type::{DataType, DataTypeMethods};

/// A data structure which provides the user with a convenient and highly abstracted way to deal with data and data types.
#[derive(Default)]
pub struct DataContainer {

    inner: Vec<DataComponent>,
    rdx: usize,

}

pub struct DataComponent(/*DataType, */Box<dyn DataTypeMethods>);

impl DataContainer {

    pub fn push(&mut self, data: Box<dyn DataTypeMethods>) {
        self.inner.push(DataComponent(data))
    }

}
/*
fn data_to_data_type<T: 'static + DataTypeMethods>(data: Box<T>) -> DataType {
    const STR_ID: TypeId = TypeId::of::<Box<String>>();
    match data.type_id() {
        STR_ID => DataType::String,
        TypeId { .. } => DataType::Other,
    }
}*/

