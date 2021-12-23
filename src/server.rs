use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::AtomicU64;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;

use crate::connection::{ConMethods, Connection};
use crate::event::Event;

// use socket2::{Socket, Domain, Type};

pub type RawConnection = (TcpStream, SocketAddr);

pub struct Server {
    listener: Arc<TcpListener>,
    client_acceptor: JoinHandle<()>,
    connections: Arc<RwLock<Vec<Connection>>>,
    connections_id: Arc<AtomicU64>,
}

impl Server {
    #[inline]
    pub fn builder_from_port(port: u16) -> ServerBuilder<'static> {
        ServerBuilder::from_port(port)
    }

    async fn new(builder: ServerBuilder<'_>) -> Self {
        let listener = if let Some(bind_port) = builder.bind_port {
            let mut result_str = String::from("127.0.0.1:");
            result_str.push_str(&*format!("{}", bind_port));
            Arc::new(TcpListener::bind(&*result_str).unwrap())
        } else {
            // Arc::new(Socket::new(Domain::UNIX, Type::DGRAM, None))
            todo!()
        };
        let tmp_listener = listener.clone();
        let connections_id = Arc::new(AtomicU64::default());
        let tmp_id = connections_id.clone();
        let client_acceptor = thread::spawn(move || loop {
            let raw_connection = tmp_listener.clone().accept().unwrap();
            let connection =
                Connection::new_con(raw_connection.0, raw_connection.1, tmp_id.clone());
            let event = Event::Connect(connection);
        });
        Self {
            listener,
            client_acceptor,
            connections: Arc::new(Default::default()),
            connections_id: connections_id.clone(),
        }
    }
}

pub struct ServerBuilder<'a> {
    bind_port: Option<u16>,
    path: Option<&'a str>,
}

impl<'a> ServerBuilder<'a> {
    #[inline]
    pub fn from_port(port: u16) -> Self {
        Self {
            bind_port: Some(port),
            path: None,
        }
    }

    /// This builder is used to construct a Server based on UDS(UnixDomainSockets) and thus this isn't available on every platform!
    #[inline]
    pub fn from_path(path: &'a str) -> Self {
        Self {
            bind_port: None,
            path: Some(path),
        }
    }

    #[inline]
    pub async fn build(self) -> Server {
        Server::new(self).await
    }
}

pub enum ClientSetting {
    BufferSize(usize),
    Timeout(Option<usize>),
    StringEncoding(),
    Compression(),
    Encryption(),
}
