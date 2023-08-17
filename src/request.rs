use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

use crate::CCResponse;

#[derive(Debug)]
pub struct CCRequest {
    pub(crate) inner: CCRequestInner,
    pub(crate) resolver: oneshot::Sender<CCResponse>,
}

impl CCRequest {
    pub fn new(kind: CCRequestKind) -> (Self, oneshot::Receiver<CCResponse>) {
        let (tx, rx) = oneshot::channel();
        let inner = CCRequestInner {
            id: Uuid::new_v4(),
            request: kind,
        };
        let req = Self {
            inner,
            resolver: tx,
        };
        (req, rx)
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct CCRequestInner {
    pub(crate) id: Uuid,
    pub(crate) request: CCRequestKind,
}

impl CCRequestInner {
    pub fn as_message(&self) -> Message {
        Message::Text(serde_json::to_string(self).unwrap())
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", content = "data")]
pub enum CCRequestKind {
    Echo(String),
    ConnectPeripheral(String),
    CallPeripheral {
        address: String,
        method: String,
        args: serde_json::Value,
    },
    GetPeripheralType(String),
}
