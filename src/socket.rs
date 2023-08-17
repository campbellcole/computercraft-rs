use thiserror::Error;
use tokio::{net::TcpListener, sync::mpsc::UnboundedSender};
use tokio_tungstenite::{accept_async, tungstenite::Error as WsError};

use crate::computer::Computer;

#[derive(Debug, Error)]
pub enum SocketError {
    #[error("Failed to bind address: {0}")]
    BindError(std::io::Error),
    #[error("Failed to connect to client: {0}")]
    ConnectError(std::io::Error),
    #[error("Failed to accept connection: {0}")]
    AcceptConnection(WsError),
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
}
