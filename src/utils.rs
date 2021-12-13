#[derive(Default)]
pub struct DataContainer(Vec<DataComponent<dyn DataTypeMethods + 'static>>);

pub struct DataComponent<T: DataTypeMethods + ?Sized>(DataType, Box<T>);

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

    fn write(&self, data_container: DataContainer);

    fn read(data_container: DataContainer) -> Box<Self> where Self: Sized;

}

