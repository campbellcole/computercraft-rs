use std::sync::Arc;

use computer::Computer;
use error::{Error, Result};
use response::CCResponse;
use tokio::{
    net::ToSocketAddrs,
    select,
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
        Self::listen_on("0.0.0.0:56552")
    }

    pub fn listen_on(addr: impl ToSocketAddrs + Send + 'static) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ServerInner::spawn_new(addr))),
        }
    }

    pub async fn wait_for_connection(&self) -> Result<Computer> {
        let mut inner = self.inner.lock().await;

        loop {
            select! {
                computer = inner.rx.recv() => {
                    let computer = computer.ok_or(Error::ServerThreadFailed)?;
                    return Ok(computer);
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                    // there may be computers in this list that are being
                    // stored in the `tried` list in `wait_for_connection_from`
                    // so we should keep checking here to see if they've been
                    // put back into the queue
                    if let Some(computer) = inner.computers.pop() {
                        return Ok(computer);
                    }
                }
            }
        }
    }

    pub async fn wait_for_connection_from(&self, name: &str) -> Result<Computer> {
        // we have to keep track of which ones we've tried here
        // because if we put them back into the queue, the next loop
        // will end up finding it and checking it again
        // which means this function will be spinning and not yielding
        let mut tried = Vec::new();

        let computer = loop {
            let computer = self.wait_for_connection().await?;
            match &computer.computer_info()?.name {
                Some(n) if n == name => break computer,
                _ => {
                    // put the computer back in the queue
                    tried.push(computer);
                }
            }
        };

        // put the other computers back in the queue
        self.inner.lock().await.computers.append(&mut tried);

        Ok(computer)
    }
}

struct ServerInner {
    handle: JoinHandle<()>,
    rx: UnboundedReceiver<Computer>,
    computers: Vec<Computer>,
}

impl Drop for ServerInner {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

impl ServerInner {
    pub fn spawn_new(addr: impl ToSocketAddrs + Send + 'static) -> Self {
        let (tx, rx) = unbounded_channel();
        let handle = tokio::spawn(socket::socket_thread(addr, tx));
        Self {
            handle,
            rx,
            computers: Vec::new(),
        }
    }
}
