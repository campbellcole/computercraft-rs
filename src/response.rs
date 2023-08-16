use thiserror::Error;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CCResponse {
    pub(crate) id: Uuid,
    pub(crate) response: CCResponseKind,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum CCResponseKind {
    Disconnected,
    Echo(String),
    ConnectPeripheral(bool),
}

#[derive(Debug, Error)]
pub enum ParseResponseError {
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Wrong message type: {0}")]
    WrongMessageType(String),
}

impl CCResponse {
    pub fn from_message(msg: Message) -> Result<Self, ParseResponseError> {
        let Message::Text(text) = msg else {
            let kind = match msg {
                Message::Binary(_) => "binary",
                Message::Close(_) => "close",
                Message::Ping(_) => "ping",
                Message::Pong(_) => "pong",
                Message::Frame(_) => "frame",
                Message::Text(_) => unreachable!(),
            }
            .into();

            return Err(ParseResponseError::WrongMessageType(kind));
        };

        Ok(serde_json::from_str(&text)?)
    }
}
