use std::time::Instant;

const RAW_BUFFER: [u8; 5] = [0; 5];
const VAR_INT_BUFFER: [u8; 2] = [0x1, 0]; // TODO: Encode var_int(0x1) at compile time

pub(crate) struct TimeoutHandler {
    counter: u8,
    last_read: Instant,
}

impl TimeoutHandler {
    pub(crate) fn send_keep_alive(&mut self) {
        if self.counter == u8::MAX {
            self.counter = u8::MIN;
        }
        self.counter += 1;
    }
}
