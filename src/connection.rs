use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::buffer::RBuffer;
use crate::utils::DataContainer;

pub type Connection = Arc<InnerConnection>;

pub struct InnerConnection {
    id: u64,
    raw_connection: TcpStream,
    state: State,
    address: SocketAddr,
    pre_connect_send_queue: DataContainer,
    hmac_key: Vec<u8>,
    closed: bool,
}

impl InnerConnection {
    pub fn new(
        raw_connection: TcpStream,
        address: SocketAddr,
        connection_id: Arc<AtomicU64>,
    ) -> Self {
        let curr_id = connection_id.load(Ordering::SeqCst);
        connection_id.store(curr_id + 1, Ordering::SeqCst);
        Self {
            id: curr_id,
            raw_connection,
            state: State::Idle,
            address,
            pre_connect_send_queue: DataContainer::default(),
            hmac_key: vec![],
            closed: false,
        }
    }

    pub(crate) fn send_data(&mut self, buffer: RBuffer) -> bool {
        let tmp = buffer.as_slice(); // TODO: Replace this with as_slice ASAP
        return match self.raw_connection.write(&tmp) {
            Ok(x) => {
                if x == 0 {
                    self.closed = true;
                    return false;
                }
                true
            }
            Err(_) => false,
        };
    }

    pub fn is_connected(&self) -> bool {
        !self.closed
    }

    pub fn disconnect(&mut self) {
        self.raw_connection.shutdown(Shutdown::Both);
        self.closed = true;
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

pub(crate) trait ConMethods {
    fn new_con(
        raw_connection: TcpStream,
        address: SocketAddr,
        connection_id: Arc<AtomicU64>,
    ) -> Arc<InnerConnection>;
}

impl ConMethods for Connection {
    fn new_con(
        raw_connection: TcpStream,
        address: SocketAddr,
        connection_id: Arc<AtomicU64>,
    ) -> Arc<InnerConnection> {
        Arc::new(InnerConnection::new(raw_connection, address, connection_id))
    }
}

enum State {
    Idle,
    Sending,
    WaitingForFinish,
    Finishing,
    Finished,
}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}
