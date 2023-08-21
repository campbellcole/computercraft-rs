use async_trait::async_trait;
use serde_json::Value;

use crate::{computer::Computer, error::Result};

pub struct Peripheral<'a> {
    pub(crate) computer: &'a Computer,
    pub(crate) address: String,
}

impl<'a> Peripheral<'a> {
    pub async fn call_method(
        &self,
        method: impl Into<String>,
        args: impl Into<Value>,
    ) -> PeripheralCallResult {
        self.computer
            .peripheral_call_method(self.address.clone(), method.into(), args.into())
            .await
    }
}

pub type PeripheralCallResult = Result<Vec<Value>>;

#[async_trait]
pub trait IntoWrappedPeripheral<W> {
    async fn into_wrapped(self) -> Result<W>;
}

macro_rules! generate_wrapper_impl {
    ($wrapper_ty:ident = $expected_ty:literal) => {
        pub struct $wrapper_ty<'a> {
            inner: Peripheral<'a>,
        }

        #[async_trait]
        impl<'a> IntoWrappedPeripheral<$wrapper_ty<'a>> for Peripheral<'a> {
            async fn into_wrapped(self) -> $crate::error::Result<$wrapper_ty<'a>> {
                let ty = self
                    .computer
                    .get_peripheral_type(self.address.clone())
                    .await?;

                if ty != $expected_ty {
                    return Err($crate::error::Error::WrongPeripheralType(
                        ty,
                        $expected_ty.into(),
                    ));
                }

                Ok($wrapper_ty { inner: self })
            }
        }
    };
}

pub(crate) use generate_wrapper_impl;
