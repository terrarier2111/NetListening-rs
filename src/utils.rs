use std::any::{Any, TypeId};
use std::mem;
use std::mem::transmute;

use crate::data_type::{DataType, DataTypeMethods};

/// A data structure which provides the user with a convenient and highly abstracted way to deal with data and data types.
#[derive(Default)]
pub struct DataContainer {
    inner: Vec<DataComponent>,
    rdx: usize,
}

pub struct DataComponent(
    /*DataType, */ Box<dyn DataTypeMethods /* + Sized*//* + Copy*/>,
);

impl DataContainer {
    pub fn push(&mut self, data: Box<dyn DataTypeMethods>) {
        self.inner.push(DataComponent(data))
    }

    pub fn read<T: DataTypeMethods + Sized>(&mut self) -> Box<T> {
        let ret = self.inner.get(self.rdx);
        self.rdx += 1;
        let tmp = ret.unwrap().0.clone();
        // unsafe { std::mem::transmute::<Box<dyn DataTypeMethods>, Box<T>>(tmp) }
        todo!()
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
