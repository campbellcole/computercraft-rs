use serde_json::Value;
use thiserror::Error;

use crate::response::CCResponse;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Computer is disconnected")]
    Disconnected,
    #[error("Server thread failed")]
    ServerThreadFailed,
    #[error("Computer thread failed")]
    ComputerThreadFailed,
    #[error("Request resolver was dropped before resolving")]
    ResolverDropped,
    #[error("Cannot deserialize returned data because there are multiple return values")]
    MultipleReturnValues,
    #[error("Cannot deserialize returned data because the function returned nothing")]
    NoReturnValues,
    #[error("Handshake was performed twice")]
    HandShookTwice,
    #[error("Handshake was not performed correctly and left the computer in an invalid state")]
    HandshakeFailed,
    #[error("Peripheral {0:?} was not found")]
    PeripheralNotFound(String),
    #[error("Peripheral is of type {0:?}, expected {1:?}")]
    WrongPeripheralType(String, String),
    #[error("Lua function returned data in an unexpected format: {0:?}")]
    UnexpectedData(Vec<Value>),
    #[error("Lua function returned an error: {0:?}")]
    LuaError(Vec<Value>),
    #[error("Request was resolved with a response of the wrong type: {0:?}")]
    WrongResponseType(CCResponse),
    #[error("Error interacting with websocket: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to deserialize returned data: {0}")]
    SerdeError(#[from] serde_json::Error),
}

#[cfg(not(feature = "debug"))]
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(feature = "debug")]
pub type Result<T, E = eyre::Report> = std::result::Result<T, E>;
