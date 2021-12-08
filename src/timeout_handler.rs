use std::time::Instant;

const RAW_BUFFER: [u8; 5] = [0; 5]; // TODO: Encode var_int(0x1) at compile time
const VAR_INT_BUFFER: [u8; 2] = [0x1, 0];

pub(crate) struct TimeoutHandler {

    counter: u8,
    last_read: Instant,

}