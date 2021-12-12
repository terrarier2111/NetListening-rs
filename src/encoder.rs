use crate::utils::{DataComponent, DataTypeMethods, DataContainer};
use std::any::Any;
use crate::connection::Connection;
use crate::application::Application;
use crate::buffer::Buffer;

pub struct Encoder<T: Application> {

    application: T,

}

pub(crate) struct EncoderConfig {



}

pub(crate) struct EncoderState {

    

}

impl<T: Application> Encoder<T> {

    pub fn encode(&mut self, connection: Connection, buffer: Buffer) {

    }

}