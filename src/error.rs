use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Computer is disconnected")]
    Disconnected,
    #[error("Peripheral {0:?} was not found")]
    PeripheralNotFound(String),
    #[error("Peripheral is of type {0:?}, expected {1:?}")]
    WrongPeripheralType(String, String),
    #[error("Lua function returned data in an unexpected format: {0:?}")]
    UnexpectedData(Vec<Value>),
    #[error("Lua function returned an error: {0:?}")]
    LuaError(Vec<Value>),
    #[error("Error interacting with websocket: {0}")]
    WsError(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
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
    #[error("Failed to deserialize returned data: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
