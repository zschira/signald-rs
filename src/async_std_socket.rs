use async_std::os::unix::net::UnixStream;
use async_std::prelude::*;
use async_std::path::Path;
use async_std::io::Error;
use async_std::io::BufReader;
use async_std::channel::{bounded, Receiver, RecvError, Sender};
use async_std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::Debug;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use crate::errors::SignaldError;
use crate::socket::AsyncSocket;
use crate::actions::SocketWrapper;

pub enum SocketError {
    General(&'static str),
    Io(Error),
    Channel(RecvError),
    Signald(SignaldError)
}

impl Debug for SocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocketError::General(desc) => write!(f, "Error: {}", desc),
            SocketError::Io(e) => write!(f, "Error: {}", e),
            SocketError::Channel(e) => write!(f, "Error: {}", e),
            SocketError::Signald(e) => write!(f, "Signald error: {}", e.error.message)
        }
    }
}

impl From<Error> for SocketError {
    fn from(e: Error) -> Self {
        SocketError::Io(e)
    }
}

impl From<RecvError> for SocketError {
    fn from(e: RecvError) -> Self {
        SocketError::Channel(e)
    }
}

pub type Map = Arc<Mutex<HashMap<Uuid, (Sender<Value>, Receiver<Value>)>>>;

pub struct Socket<T> {
    socket: T,
    response_map: Map,
    listening: Arc<Mutex<bool>>
}

#[async_trait]
impl AsyncSocket for Socket<UnixStream> {
    async fn write<'a>(&'a mut self, buf: &'a [u8], id: &Uuid) -> Result<(), SocketError> {
        let channel = bounded(1);
        self.response_map.lock().await.insert(
            *id,
            channel
        );

        match self.socket.write_all(buf).await {
            Ok(()) => Ok(()),
            Err(e) => Err(SocketError::Io(e))
        }
    }

    async fn get_response<'a>(&'a mut self, id: Uuid) -> Result<Value, SocketError> {
        let receiver = match self.response_map.lock().await.get(&id) {
            Some(channel) => channel.1.clone(),
            None => { return Err(SocketError::General("Error: Incorrect response ID")); }
        };

        receiver.recv().await.map_err(|e| SocketError::Channel(e))
    }
}

impl Socket<UnixStream> {
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self, SocketError> {
        let socket = UnixStream::connect(path).await?;
        let response_map = Arc::new(Mutex::new(HashMap::new()));
        let listening = Arc::new(Mutex::new(true));

        let socket_wrapper = Socket {
            socket: socket.clone(),
            response_map: response_map.clone(),
            listening: listening.clone()
        };

        async_std::task::spawn(async move {
            listen(
                socket,
                response_map,
                listening
            ).await;
        });

        Ok(socket_wrapper)
    }
}

async fn listen(socket: UnixStream, map: Map, listening: Arc<Mutex<bool>>) {
    let mut reader = BufReader::new(socket);
    let mut buf = String::with_capacity(1024);

    while *listening.lock().await {
        match reader.read_line(&mut buf).await {
            Ok(_) => {
                let response: Value = serde_json::from_str(buf.as_str()).unwrap();
                if let Some(id) = response.get("id") {
                    let id = Uuid::parse_str(id.as_str().unwrap()).unwrap();
                    let sender = map.lock().await.get(&id).unwrap().0.clone();
                    match sender.send(response).await {
                        Ok(_) => {},
                        Err(e) => println!("Error sending response: {}", e)
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e.to_string());
            }
        }

        buf.clear();
    }
}

pub type Signald = SocketWrapper<Socket<UnixStream>>;

impl Signald {
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self, SocketError> {
        Ok(Signald {
            socket: Socket::connect(path).await?,
        })
    }
}
