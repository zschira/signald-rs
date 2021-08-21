use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;
use async_trait::async_trait;
use crate::SocketError;

#[async_trait]
pub trait AsyncSocket {
    async fn write<'a>(&'a mut self, buf: &'a [u8], id: &Uuid) -> Result<(), SocketError>;
    async fn get_response<'a>(&'a mut self, id: Uuid) -> Result<Value, SocketError>;
}

#[derive(Serialize, Default)]
pub struct MessageCommon<T> {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String,
    #[serde(flatten)]
    pub data: T
}

#[derive(Serialize, Default)]
pub struct Protocol {
    #[serde(rename = "type")]
    pub type_: String,
    pub version: String
}

impl<T> MessageCommon<T> {
    pub fn new(id: String, type_: String, version: String, data: T) -> Self {
        MessageCommon {
            id,
            type_,
            version,
            data
        }
    }
}
