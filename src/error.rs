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
    #[error("Lua function returned data in an unexpected format")]
    UnexpectedData,
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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
