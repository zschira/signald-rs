pub mod types;
pub mod actions;
pub mod socket;
pub mod errors;

#[cfg(feature = "async-std")]
pub mod async_std_socket;
#[cfg(feature = "async-std")]
pub use crate::async_std_socket::{SocketError, Signald};

#[cfg(feature = "tokio")]
pub mod tokio_socket;
#[cfg(feature = "tokio")]
pub use crate::tokio_socket::{SocketError, Signald};
#[cfg(feature = "tokio")]
use tokio::test;

#[cfg(test)]
mod tests {
    use super::SocketError;

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn tokio_register() -> Result<(), SocketError> {
        register().await
    }

    #[cfg(feature = "async-std")]
    #[async_std::test]
    async fn async_std_register() -> Result<(), SocketError> {
        register().await
    }

    async fn register() -> Result<(), SocketError>  {
        use super::types::RegisterRequestV1;
        use super::Signald;

        let mut socket = Signald::connect("run/signald.sock").await?;

        let mut register = RegisterRequestV1::default();
        register.account = Some("+15551234567".to_owned());

        let response = socket.register(register).await;

        match response {
            Ok(response) => assert_eq!(response.address.unwrap().number.unwrap().as_str(), "+15551234567"),
            Err(e) => {
                if let SocketError::Signald(e) = e {
                    assert_eq!(e.error_type.as_str(), "CaptchaRequired");
                } else {
                    panic!("Received error other than CaptchaRequired");
                }
            }
        }

        Ok(())
    }
}
