use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;
use std::thread::{Thread, JoinHandle};
use std::sync::{Arc, RwLock};
use crate::event::Event;
use crate::connection::{Connection, ConMethods};

pub type RawConnection = (TcpStream, SocketAddr);

pub struct Server {

    listener: Arc<TcpListener>,
    client_acceptor: JoinHandle<()>,
    connections: Arc<RwLock<Vec<Connection>>>,

}

impl Server {

    async fn new(builder: ServerBuilder) -> Self {
        let mut result_str = String::from("127.0.0.1:");
        result_str.push_str(&*format!("{}", builder.bind_port));
        let listener = Arc::new(TcpListener::bind(&*result_str).unwrap());
        let tmp_listener = listener.clone();
        let client_acceptor = thread::spawn(move || {
            loop {
                let raw_connection = tmp_listener.clone().accept().unwrap();
                let connection = Connection::new_con(raw_connection.0, raw_connection.1);
                let event = Event::Connect(connection);

            }
        });
        Self {
            listener,
            client_acceptor,
            connections: Arc::new(Default::default())
        }
    }

}

pub struct ServerBuilder {

    bind_port: u16,

}

impl ServerBuilder {

    #[inline]
    pub fn port(mut self, port: u16) -> Self {
        self.bind_port = port;
        self
    }

    #[inline]
    pub fn build(self) -> Server {
        Server::new(self)
    }

}