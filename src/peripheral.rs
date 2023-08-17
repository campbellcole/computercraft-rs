use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

use crate::computer::Computer;

pub struct Peripheral<'a> {
    pub(crate) computer: &'a Computer,
    pub(crate) address: String,
}

impl<'a> Peripheral<'a> {
    pub async fn call_method(
        &self,
        method: impl Into<String>,
        args: impl Into<Value>,
    ) -> Option<PeripheralCallResult> {
        self.computer
            .peripheral_call_method(self.address.clone(), method.into(), args.into())
            .await
    }
}

pub type PeripheralCallResult = Result<Vec<Value>, Vec<Value>>;

#[derive(Debug, Error)]
pub enum WrapPeripheralError {
    #[error("Peripheral is of type {0} when we expected {1}")]
    WrongType(String, String),
    #[error("Computer is disconnected")]
    Disconnected,
}

#[async_trait]
pub trait IntoWrappedPeripheral<W> {
    async fn into_wrapped(self) -> Result<W, WrapPeripheralError>;
}
