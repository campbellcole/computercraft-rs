use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use serde_json::Value;
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
    debug_feature,
    error::{Error, Result},
    peripheral::{Peripheral, PeripheralCallResult},
    request::{CCRequest, CCRequestKind, PeripheralArgs},
    response::{CCResponse, CCResponseKind, ParseResponseError},
};

#[derive(Debug)]
pub struct Computer {
    inner: Arc<ComputerInner>,
    computer_info: OnceLock<ComputerInfo>,
}

macro_rules! impl_requests {
    ($(
        $(#[$meta:meta])?
        $variant:ident = $fn_vis:vis $name:ident => |$($arg_ident:ident: $arg_type:ty),*| -> $return_type:ty;
    )*) => {
        impl Computer {
            $(
                $(#[$meta])?
                $fn_vis async fn $name(&self, $($arg_ident: $arg_type),*) -> Result<$return_type> {
                    match self.send_raw(CCRequestKind::$variant($($arg_ident),*)).await?.response {
                        CCResponseKind::Disconnected => debug_feature!(Err(Error::Disconnected)),
                        CCResponseKind::$variant(res) => Ok(res),
                        _ => unreachable!("request was resolved with a response of the wrong kind!"),
                    }
                }
            )*
        }
    };
}

impl Computer {
    pub(crate) async fn new(ws: WebSocketStream<TcpStream>) -> Result<Self> {
        let (tx, rx) = unbounded_channel();
        let handle = tokio::spawn(computer_thread(ws, rx));
        let mut inst = Self {
            inner: Arc::new(ComputerInner { handle, tx }),
            computer_info: OnceLock::new(),
        };

        inst.handshake().await?;

        Ok(inst)
    }

    async fn handshake(&mut self) -> Result<()> {
        let shake = self.send_raw(CCRequestKind::Handshake).await?;
        match shake.response {
            CCResponseKind::Handshake(info) => {
                self.computer_info
                    .set(info)
                    .map_err(|_| Error::HandShookTwice)?;
                Ok(())
            }
            _ => debug_feature!(Err(Error::WrongResponseType(shake))),
        }
    }

    pub fn computer_info(&self) -> Result<&ComputerInfo> {
        debug_feature!(self.computer_info.get().ok_or(Error::HandshakeFailed))
    }

    pub async fn find_peripheral(&self, address: impl ToString) -> Result<Peripheral<'_>> {
        let address = address.to_string();
        let connected = self.connect_peripheral(address.clone()).await?;
        if connected {
            Ok(Peripheral {
                computer: self,
                address,
            })
        } else {
            debug_feature!(Err(Error::PeripheralNotFound(address)))
        }
    }

    async fn send_raw(&self, kind: CCRequestKind) -> Result<CCResponse> {
        let (request, resolver) = CCRequest::new(kind);
        self.inner
            .tx
            .send(request)
            .map_err(|_| Error::ComputerThreadFailed)?;
        debug_feature!(resolver.await.map_err(|_| Error::ResolverDropped))
    }

    pub(crate) async fn peripheral_call_method<S: PeripheralArgs>(
        &self,
        address: String,
        method: String,
        args: S,
    ) -> PeripheralCallResult {
        let res = self
            .send_raw(CCRequestKind::CallPeripheral {
                address,
                method,
                args: Box::new(args),
            })
            .await?;
        match res.response {
            CCResponseKind::Disconnected => debug_feature!(Err(Error::Disconnected)),
            CCResponseKind::CallPeripheral {
                success,
                error,
                result,
            } => {
                if success {
                    Ok(result.unwrap_or_default())
                } else {
                    debug_feature!(Err(Error::LuaError(error.unwrap_or_default())))
                }
            }
            _ => debug_feature!(Err(Error::WrongResponseType(res))),
        }
    }

    pub(crate) async fn peripheral_call_into<S: PeripheralArgs, T: DeserializeOwned>(
        &self,
        address: String,
        method: String,
        args: S,
    ) -> Result<T> {
        match &self.peripheral_call_method(address, method, args).await?[..] {
            #[cfg(not(feature = "debug"))]
            [val] => Ok(T::deserialize(val)?),
            #[cfg(feature = "debug")]
            [val] => Ok(serde_path_to_error::deserialize(val)?),
            [] => debug_feature!(Err(Error::NoReturnValues)),
            _ => debug_feature!(Err(Error::MultipleReturnValues)),
        }
    }

    pub(crate) async fn peripheral_call_into_raw<S: PeripheralArgs, T: DeserializeOwned>(
        &self,
        address: String,
        method: String,
        args: S,
    ) -> Result<T> {
        let val = Value::Array(self.peripheral_call_method(address, method, args).await?);

        #[cfg(not(feature = "debug"))]
        return Ok(T::deserialize(val)?);

        #[cfg(feature = "debug")]
        return Ok(serde_path_to_error::deserialize(val)?);
    }
}

impl_requests! {
    Echo = pub echo => |msg: String| -> String;
    ConnectPeripheral = connect_peripheral => |address: String| -> bool;
    GetPeripheralType = pub(crate) get_peripheral_type => |address: String| -> String;
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ComputerKind {
    Computer,
    Turtle,
    Pocket,
    Command,
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ComputerInfo {
    pub name: Option<String>,
    pub kind: ComputerKind,
    pub advanced: bool,
}

#[derive(Debug)]
struct ComputerInner {
    handle: JoinHandle<()>,
    tx: UnboundedSender<CCRequest>,
}

impl Drop for ComputerInner {
    fn drop(&mut self) {
        self.handle.abort();
    }
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
                if let Some(resolver) = resolvers.remove(&response.id) {
                    resolver.send(response).map_err(|res| ComputerError::DispatchResponse(res.id))?;
                } else if response.id == Uuid::nil() { // nil Uuid means the socket was closed
                    for (_, resolver) in std::mem::take(&mut resolvers).into_iter() {
                        resolver.send(response.clone()).map_err(|res| ComputerError::DispatchResponse(res.id))?;
                    }
                } else {
                    return Err(ComputerError::UnknownResponse(response.id));
                }
            }
            else => {
                // the socket was closed and the computer was dropped
                break Ok(());
            }
        }
    }
}
