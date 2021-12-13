use crate::utils::{DataComponent, DataTypeMethods, DataContainer};
use std::any::Any;
use crate::connection::Connection;
use crate::application::Application;
use crate::buffer::Buffer;

pub struct Encoder<T: Application> {

    application: T,
    config: EncoderConfig,

}

pub(crate) struct EncoderConfig {



}

pub(crate) struct EncoderState {

    connection: Connection,

}

impl<T: Application> Encoder<T> {

    pub fn encode(&self, state: &mut EncoderState, buffer: &mut Buffer) {

    }

}
