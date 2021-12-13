use crate::utils::DataContainer;
use crate::connection::Connection;
use crate::application::Application;
use crate::buffer::Buffer;

pub struct Encoder {

    application: Application,
    connection: Connection,

}

impl Encoder {

    pub(crate) fn encode(&self, buffer: &mut Buffer, data_container: DataContainer) {

    }

}
