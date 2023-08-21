use std::sync::Arc;

use computer::Computer;
use error::{Error, Result};
use response::CCResponse;
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver},
        Mutex,
    },
    task::JoinHandle,
};

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate serde;

pub mod computer;
pub mod error;
pub mod peripheral;
pub mod protocol;
mod request;
mod response;
mod socket;

#[cfg(feature = "peripheral-wrappers")]
pub mod wrappers;

pub struct Server {
    inner: Arc<Mutex<ServerInner>>,
}

impl Server {
    pub fn listen() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ServerInner::spawn_new())),
        }
    }

    pub async fn wait_for_connection(&self) -> Result<Computer> {
        let mut inner = self.inner.lock().await;
        let computer = inner.rx.recv().await.ok_or(Error::ServerThreadFailed)?;
        Ok(computer)
    }
}

struct ServerInner {
    _handle: JoinHandle<()>,
    rx: UnboundedReceiver<Computer>,
}

impl ServerInner {
    pub fn spawn_new() -> Self {
        let (tx, rx) = unbounded_channel();
        let handle = tokio::spawn(socket::socket_thread(tx));
        Self {
            _handle: handle,
            rx,
        }
    }
}
