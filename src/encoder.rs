use crate::application::Application;
use crate::buffer::RWBuffer;
use crate::connection::Connection;
use crate::utils::DataContainer;

pub struct Encoder {
    application: Application,
    connection: Connection,
}

impl Encoder {
    pub(crate) fn encode(&self, buffer: RWBuffer, data_container: DataContainer) {}
}
