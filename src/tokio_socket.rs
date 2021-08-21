use tokio::net::UnixStream;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::io::BufReader;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use std::fmt::Debug;
use async_trait::async_trait;
use serde_json::Value;
use std::io::Error;
use std::path::Path;

use crate::errors::SignaldError;
use crate::socket::AsyncSocket;
use crate::actions::SocketWrapper;

pub enum SocketError {
    General(&'static str),
    Io(Error),
    Channel(&'static str),
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

pub type Map = Arc<Mutex<HashMap<Uuid, (Sender<Value>, Option<Receiver<Value>>)>>>;

pub struct Socket<T> {
    socket: T,
    response_map: Map,
    listening: Arc<Mutex<bool>>
}

#[async_trait]
impl AsyncSocket for Socket<OwnedWriteHalf> {
    async fn write<'a>(&'a mut self, buf: &'a [u8], id: &Uuid) -> Result<(), SocketError> {
        let channel = mpsc::channel(1);
        self.response_map.lock().unwrap().insert(
            *id,
            (channel.0, Some(channel.1))
        );

        self.socket.write_all(buf).await?;
        Ok(())
    }

    async fn get_response<'a>(&'a mut self, id: Uuid) -> Result<Value, SocketError> {
        let mut receiver = match self.response_map.lock().unwrap().get_mut(&id) {
            Some(channel) => channel.1.take().unwrap(),
            None => { return Err(SocketError::General("Error: Incorrect response ID")); }
        };

        receiver.recv().await.ok_or(SocketError::Channel("Failed to receive response"))
    }
}

impl Socket<OwnedWriteHalf> {
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self, SocketError> {
        let (reader, writer) = UnixStream::connect(path).await?.into_split();
        let response_map = Arc::new(Mutex::new(HashMap::new()));
        let listening = Arc::new(Mutex::new(true));

        let socket_wrapper = Socket {
            socket: writer,
            response_map: response_map.clone(),
            listening: listening.clone()
        };

        tokio::task::spawn(async move {
            listen(
                reader,
                response_map,
                listening
            ).await;
        });

        Ok(socket_wrapper)
    }
}

async fn listen(socket: OwnedReadHalf, map: Map, listening: Arc<Mutex<bool>>) {
    let mut reader = BufReader::new(socket);
    let mut buf = String::with_capacity(1024);

    while *listening.lock().unwrap() {
        match reader.read_line(&mut buf).await {
            Ok(_) => {
                let response: Value = serde_json::from_str(buf.as_str()).unwrap();
                if let Some(id) = response.get("id") {
                    let id = Uuid::parse_str(id.as_str().unwrap()).unwrap();
                    let sender = map.lock().unwrap().get(&id).unwrap().0.clone();
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

pub type Signald = SocketWrapper<Socket<OwnedWriteHalf>>;

impl Signald {
    pub async fn connect<P: AsRef<Path>>(path: P) -> Result<Self, SocketError> {
        Ok(Signald {
            socket: Socket::connect(path).await?,
        })
    }
}
