use std::net::{TcpStream, SocketAddr};
use crate::utils::DataContainer;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU64};

pub type Connection = Arc<InnerConnection>;

pub struct InnerConnection {

    id: u64,
    raw_connection: TcpStream,
    state: State,
    address: SocketAddr,
    pre_connect_send_queue: DataContainer,
    hmac_key: Vec<u8>

}

impl InnerConnection {

    pub fn new(raw_connection: TcpStream, address: SocketAddr, connection_id: AtomicU64) -> Self {
        let curr_id = connection_id.load(Ordering::SeqCst);
        connection_id.store(curr_id + 1, Ordering::SeqCst);
        Self {
            id: curr_id,
            raw_connection,
            state: State::Idle,
            address,
            pre_connect_send_queue: DataContainer::default(),
            hmac_key: vec![]
        }
    }

    pub fn is_connected(&self) -> bool {
        true
    }

    pub fn disconnect(&mut self) {

    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn id(&self) -> u64 {
        self.id
    }

}

pub(crate) trait ConMethods {

    fn new_con(raw_connection: TcpStream, address: SocketAddr) -> Arc<InnerConnection>;

}

impl ConMethods for Connection {
    fn new_con(raw_connection: TcpStream, address: SocketAddr) -> Arc<InnerConnection> {
        Arc::new(InnerConnection::new(raw_connection, address))
    }
}

enum State {

    Idle,
    Sending,
    WaitingForFinish,
    Finishing,
    Finished

}

impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

