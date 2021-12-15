use crate::data_type::{DataTypeMethods, DataType};

#[derive(Default)]
pub struct DataContainer(Vec<DataComponent>);

pub struct DataComponent(DataType, Box<dyn DataTypeMethods>);