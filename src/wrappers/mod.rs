pub mod lua_compat;
pub mod monitor;
pub mod printer;
pub mod shared;

#[cfg(feature = "advanced-peripherals")]
pub mod ap;

macro_rules! generate_wrapped_fn {
    ($fn_name:ident -> void = |$($arg_ident:ident: $arg_ty:ty),*| => $remote_fn:ident($remote_arg:expr)) => {
        pub async fn $fn_name(&self, $($arg_ident: $arg_ty),*) -> Result<()> {
            self.inner.call_method(stringify!($remote_fn), $remote_arg).await?;

            Ok(())
        }
    };
    (
        $fn_name:ident -> $return_ty:ty = |$($arg_ident:ident: $arg_ty:ty),*| => $remote_fn:ident($remote_arg:expr);
        $arm:pat => $ret:expr
    ) => {
        pub async fn $fn_name(&self$(, $arg_ident: $arg_ty)*) -> Result<$return_ty> {
            match &self.inner.call_method(stringify!($remote_fn), $remote_arg).await?[..] {
                $arm => $ret,
                ret => Err($crate::error::Error::UnexpectedData(ret.to_vec())),
            }
        }
    };
}

use async_trait::async_trait;
pub(crate) use generate_wrapped_fn;

#[async_trait]
pub trait IntoWrappedPeripheral<W> {
    async fn into_wrapped(self) -> crate::error::Result<W>;
}

macro_rules! generate_wrapper_impl {
    ($wrapper_ty:ident = $expected_ty:literal) => {
        #[derive(Debug)]
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

pub(crate) mod prelude {
    pub use async_trait::async_trait;
    pub use serde_json::Value;

    pub(crate) use crate::{
        error::Result,
        peripheral::Peripheral,
        wrappers::{
            generate_wrapped_fn, generate_wrapper_impl, shared::color::Color, IntoWrappedPeripheral,
        },
    };
}
