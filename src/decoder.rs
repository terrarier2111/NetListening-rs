use crate::utils::{DataComponent, DataTypeMethods, DataContainer};
use std::any::Any;
use std::io::Read;
use crate::connection::Connection;
use crate::buffer::Buffer;

pub struct Decoder {



}

pub(crate) struct DecoderConfig {

    max_frame_size: u32,
    keep_alive: bool,

}

pub(crate) struct DecoderState {

    stored_data: DataContainer,// Vec<DataComponent<Box<dyn DataTypeMethods + 'static>>>,
    index: u32,
    has_id: bool,
    invalid_data: bool,
    last_keep_alive_id: u8,
    read_keep_alive_id: bool,

}

pub(crate) struct FramingState {

    active: bool,
    has_id: bool,


}

impl Decoder {

    pub fn decode(&mut self, connection: Connection, buffer: Buffer) {

    }

}