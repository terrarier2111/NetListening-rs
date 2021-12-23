use parking_lot::Mutex;

use crate::application::Application;
use crate::buffer::{RBuffer, RWBuffer};
use crate::connection::Connection;
use crate::utils::DataContainer;

pub(crate) struct Decoder {
    application: Application,
    config: DecoderConfig,
    state: Mutex<DecoderState>,
}

pub(crate) struct DecoderConfig {
    max_frame_size: u32,
    keep_alive: bool,
}

pub(crate) struct DecoderState {
    connection: Connection,
    stored_data: DataContainer, // Vec<DataComponent<Box<dyn DataTypeMethods + 'static>>>,
    index: u32,
    has_id: bool,
    invalid_data: bool,
    last_keep_alive_id: u8,
    read_keep_alive_id: bool,
    framing: FramingState,
}

pub(crate) struct FramingState {
    active: bool,
    has_id: bool,
}

impl Decoder {
    // FIXME: Return result with IOError and some other type or `()`
    pub fn decode(&self, buffer: RBuffer) {
        let mut state = self.state.lock();
    }
}

// FIXME: Can this be implemented safely when using async?
//unsafe impl Send for DecoderState {}
//unsafe impl Sync for DecoderState {}
unsafe impl Send for DecoderConfig {}
unsafe impl Sync for DecoderConfig {}
