use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorInner {
    pub more: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignaldError {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub error: ErrorInner,
    pub error_type: String
}
