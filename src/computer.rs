use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use thiserror::Error;
use tokio::{
    net::TcpStream,
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_tungstenite::{tungstenite::Error as WsError, WebSocketStream};
use uuid::Uuid;

use crate::{
    peripheral::{Peripheral, PeripheralCallResult},
    request::{CCRequest, CCRequestKind},
    response::{CCResponse, CCResponseKind, ParseResponseError},
};

pub struct Computer {
    inner: Arc<ComputerInner>,
}

macro_rules! impl_requests {
    ($(
        $(#[$meta:meta])?
        $variant:ident = $fn_vis:vis $name:ident => |$($arg_ident:ident: $arg_type:ty),*| -> $return_type:ty;
    )*) => {
        impl Computer {
            $(
                $(#[$meta])?
                $fn_vis async fn $name(&self, $($arg_ident: $arg_type),*) -> Option<$return_type> {
                    match self.send_raw(CCRequestKind::$variant($($arg_ident),*)).await?.response {
                        CCResponseKind::Disconnected => None,
                        CCResponseKind::$variant(res) => Some(res),
                        _ => unreachable!("request was resolved with a response of the wrong kind!"),
                    }
                }
            )*
        }
    };
}

impl Computer {
    pub(crate) fn new(ws: WebSocketStream<TcpStream>) -> Self {
        let (tx, rx) = unbounded_channel();
        let handle = tokio::spawn(computer_thread(ws, rx));
        Self {
            inner: Arc::new(ComputerInner {
                _handle: handle,
                tx,
            }),
        }
    }

    pub async fn find_peripheral<'a>(&'a self, address: impl ToString) -> Option<Peripheral<'a>> {
        let address = address.to_string();
        let connected = self.connect_peripheral(address.clone()).await?;
        if connected {
            Some(Peripheral {
                computer: self,
                address,
            })
        } else {
            None
        }
    }

    async fn send_raw(&self, kind: CCRequestKind) -> Option<CCResponse> {
        let (request, resolver) = CCRequest::new(kind);
        self.inner.tx.send(request).ok()?;
        resolver.await.ok()
    }

    pub(crate) async fn peripheral_call_method(
        &self,
        address: String,
        method: String,
        args: serde_json::Value,
    ) -> Option<PeripheralCallResult> {
        match self
            .send_raw(CCRequestKind::CallPeripheral {
                address,
                method,
                args,
            })
            .await?
            .response
        {
            CCResponseKind::Disconnected => None,
            CCResponseKind::CallPeripheral {
                success,
                error,
                result,
            } => {
                if success {
                    Some(Ok(result.unwrap_or(vec![])))
                } else {
                    Some(Err(error.unwrap_or(vec![])))
                }
            }
            _ => unreachable!(),
        }
    }

    // /// Sends an Echo request to the computer and returns the response.
    // ///
    // /// Returns `None` if and only if the computer is disconnected.
    // pub async fn echo(&self, msg: String) -> Option<String> {
    //     match self.send_raw(CCRequestKind::Echo(msg)).await?.response {
    //         CCResponseKind::Echo(msg) => Some(msg),
    //         CCResponseKind::Disconnected => None,
    //         _ => unreachable!("request was resolved with a response of the wrong kind!"),
    //     }
    // }
}

impl_requests! {
    Echo = pub echo => |msg: String| -> String;
    ConnectPeripheral = connect_peripheral => |address: String| -> bool;
    GetPeripheralType = pub(crate) get_peripheral_type => |address: String| -> String;
}

struct ComputerInner {
    _handle: JoinHandle<()>,
    tx: UnboundedSender<CCRequest>,
}

#[derive(Debug, Error)]
pub enum ComputerError {
    #[error("Failed to send message: {0}")]
    SendMessage(WsError),
    #[error("Failed to receive message: {0}")]
    ReceiveMessage(WsError),
    #[error("Failed to parse response: {0}")]
    ParseResponse(#[from] ParseResponseError),
    #[error("Received response for unknown request: {0}")]
    UnknownResponse(Uuid),
    #[error("Failed to dispatch response for request: {0}")]
    DispatchResponse(Uuid),
}

pub async fn computer_thread(ws: WebSocketStream<TcpStream>, rx: UnboundedReceiver<CCRequest>) {
    if let Err(err) = computer_thread_inner(ws, rx).await {
        error!("Computer thread failed: {}", err);
    }
}

async fn computer_thread_inner(
    mut ws: WebSocketStream<TcpStream>,
    mut rx: UnboundedReceiver<CCRequest>,
) -> Result<(), ComputerError> {
    let mut resolvers = HashMap::new();

    loop {
        select! {
            Some(request) = rx.recv() => {
                trace!("Received request: {:?}", request);
                resolvers.insert(request.inner.id, request.resolver);
                ws.send(request.inner.as_message()).await.map_err(ComputerError::SendMessage)?;
            }
            Some(msg) = ws.next() => {
                let msg = msg.map_err(ComputerError::ReceiveMessage)?;
                trace!("Received message: {:?}", msg);
                let response = CCResponse::from_message(msg)?;
                let resolver = resolvers.remove(&response.id).ok_or(ComputerError::UnknownResponse(response.id))?;
                resolver.send(response).map_err(|res| ComputerError::DispatchResponse(res.id))?;
            }
        }
    }
}
