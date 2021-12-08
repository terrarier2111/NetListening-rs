use crate::event::EventManager;
use std::net::TcpStream;
use crate::connection::Connection;

pub struct Client {

    event_manager: EventManager,
    local_connection: TcpStream,


}

impl Client {

    fn new(builder: ClientBuilder) -> Self {
        Self {
            event_manager: EventManager::default(),
            local_connection: TcpStream::connect(format!("{}:{}", builder.target_address, builder.target_port)).unwrap(),
        }
    }

    pub async fn connect(&self) {

    }

}

pub struct ClientBuilder {

    target_address: String,
    target_port: u16,

}

impl ClientBuilder {

    #[inline]
    pub fn target_address(mut self, target_address: String) -> Self {
        self.target_address = target_address;
        self
    }

    #[inline]
    pub fn target_port(mut self, target_port: u16) -> Self {
        self.target_port = target_port;
        self
    }

    #[inline]
    pub fn build(self) -> Client {
        Client::new(self)
    }

}