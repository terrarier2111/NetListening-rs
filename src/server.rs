use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::thread::{Thread, JoinHandle};
use std::sync::{Arc, RwLock};
use crate::event::Event;
use crate::connection::{Connection, ConMethods};
use std::sync::atomic::AtomicU64;

pub type RawConnection = (TcpStream, SocketAddr);

pub struct Server {

    listener: Arc<TcpListener>,
    client_acceptor: JoinHandle<()>,
    connections: Arc<RwLock<Vec<Connection>>>,
    connections_id: Arc<AtomicU64>,

}

impl Server {

    #[inline]
    pub fn builder_from_port(port: u16) -> ServerBuilder {
        ServerBuilder::from_port(port)
    }

    async fn new(builder: ServerBuilder) -> Self {
        let mut result_str = String::from("127.0.0.1:");
        result_str.push_str(&*format!("{}", builder.bind_port));
        let listener = Arc::new(TcpListener::bind(&*result_str).unwrap());
        let tmp_listener = listener.clone();
        let connections_id = Arc::new(AtomicU64::default());
        let tmp_id = connections_id.clone();
        let client_acceptor = thread::spawn(move || {
            loop {
                let raw_connection = tmp_listener.clone().accept().unwrap();
                let connection = Connection::new_con(raw_connection.0, raw_connection.1, tmp_id.clone());
                let event = Event::Connect(connection);

            }
        });
        Self {
            listener,
            client_acceptor,
            connections: Arc::new(Default::default()),
            connections_id: connections_id.clone(),
        }
    }

}

pub struct ServerBuilder {

    bind_port: u16,

}

impl ServerBuilder {

    #[inline]
    pub fn from_port(port: u16) -> Self {
        Self {
            bind_port: port
        }
    }

    #[inline]
    pub async fn build(self) -> Server {
        Server::new(self).await
    }

}