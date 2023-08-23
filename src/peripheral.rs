use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{computer::Computer, error::Result, request::PeripheralArgs};

#[derive(Debug)]
pub struct Peripheral<'a> {
    pub(crate) computer: &'a Computer,
    pub(crate) address: String,
}

impl<'a> Peripheral<'a> {
    pub async fn call_method<S: PeripheralArgs>(
        &self,
        method: impl Into<String>,
        args: S,
    ) -> PeripheralCallResult {
        self.computer
            .peripheral_call_method(self.address.clone(), method.into(), args)
            .await
    }

    pub async fn call_method_with<S: PeripheralArgs, T: DeserializeOwned>(
        &self,
        method: impl Into<String>,
        args: S,
    ) -> Result<T> {
        self.computer
            .peripheral_call_into(self.address.clone(), method.into(), args)
            .await
    }

    pub async fn call_method_with_raw<S: PeripheralArgs, T: DeserializeOwned>(
        &self,
        method: impl Into<String>,
        args: S,
    ) -> Result<T> {
        self.computer
            .peripheral_call_into_raw(self.address.clone(), method.into(), args)
            .await
    }
}

pub type PeripheralCallResult = Result<Vec<Value>>;
