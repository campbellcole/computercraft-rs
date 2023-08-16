use std::{collections::HashMap, sync::OnceLock};

use futures_util::{SinkExt, StreamExt};
use thiserror::Error;
use tokio::{
    net::TcpListener,
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tokio_tungstenite::{accept_async, tungstenite::Error as WsError};
use uuid::Uuid;

use crate::{
    computer::Computer,
    response::{CCResponse, CCResponseKind, ParseResponseError},
    CCRequest,
};

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("Failed to bind address: {0}")]
    BindError(std::io::Error),
    #[error("Failed to connect to client: {0}")]
    ConnectError(std::io::Error),
    #[error("Failed to accept connection: {0}")]
    AcceptConnection(WsError),
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

pub async fn socket_thread(tx: UnboundedSender<Computer>) {
    if let Err(e) = socket_thread_inner(tx).await {
        error!("socket thread failed: {}", e);
    }
}

#[instrument(skip(tx))]
pub async fn socket_thread_inner(tx: UnboundedSender<Computer>) -> Result<(), SocketError> {
    let server = TcpListener::bind("0.0.0.0:56552")
        .await
        .map_err(SocketError::BindError)?;

    loop {
        let accepted = server.accept().await;
        let (stream, _) = accepted.map_err(SocketError::ConnectError)?;
        let ws = accept_async(stream)
            .await
            .map_err(SocketError::AcceptConnection)?;
        let computer = Computer::new(ws);
        tx.send(computer).unwrap();
    }

    // loop {
    //     select! {
    //         accepted = server.accept() => {
    //             let (stream, _) = accepted.map_err(SocketError::ConnectError)?;
    //             let ws = accept_async(stream)
    //                 .await
    //                 .map_err(SocketError::AcceptConnection)?;
    //             let id = Uuid::new_v4();
    //             clients.insert(id, ws);
    //             tx.send(id).unwrap();
    //         }
    //         Some(request) = rx.recv() => {
    //             trace!("Received request: {:?}", request);
    //             let Some(ws) = clients.get(&request.computer_id) else {
    //                 request.resolver.send(CCResponse {
    //                     id: request.id,
    //                     data: CCResponseKind::Disconnected,
    //                 }).map_err(|res| SocketError::DispatchResponse(res.id))?;
    //                 continue;
    //             };
    //             resolvers.insert(request.inner.id, request.resolver);
    //             ws.send(request.inner.as_message()).await.map_err(SocketError::SendMessage)?;
    //         },
    //         Some(msg) = ws.next() => {
    //             let msg = msg.map_err(SocketError::ReceiveMessage)?;
    //             trace!("Received message: {:?}", msg);
    //             let response = CCResponse::from_message(msg)?;
    //             let resolver = resolvers.remove(&response.id).ok_or(SocketError::UnknownResponse(response.id))?;
    //             resolver.send(response).map_err(|res| SocketError::DispatchResponse(res.id))?;
    //         }
    //     }
    // }

    // Ok(())
}
