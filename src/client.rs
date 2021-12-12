use crate::event::EventManager;
use std::net::TcpStream;
use crate::connection::Connection;

pub struct Client {

    event_manager: EventManager,
    local_connection: TcpStream,


}

impl Client {

    #[inline]
    pub fn builder_from_destination(address: String, port: u16) -> ClientBuilder {
        ClientBuilder::from_destination(address, port)
    }

    async fn new(builder: ClientBuilder) -> Self {
        Self {
            event_manager: EventManager::default(),
            local_connection: TcpStream::connect(format!("{}:{}", builder.dst_address, builder.dst_port)).unwrap(),
        }
    }

}

pub struct ClientBuilder {

    dst_address: String,
    dst_port: u16,

}

impl ClientBuilder {

    #[inline]
    pub fn from_destination(address: String, port: u16) -> Self {
        Self {
            dst_address: address,
            dst_port: port,
        }
    }

    #[inline]
    pub fn setting(mut self, setting: ClientSetting) -> Self {

        self
    }

    #[inline]
    pub async fn build(self) -> Client {
        Client::new(self).await
    }

}

pub enum ClientSetting {



}