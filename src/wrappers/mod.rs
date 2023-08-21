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

pub(crate) use generate_wrapped_fn;

pub(crate) mod prelude {
    pub use async_trait::async_trait;
    pub use serde_json::Value;

    pub(crate) use crate::{
        error::Result,
        peripheral::{generate_wrapper_impl, IntoWrappedPeripheral, Peripheral},
        wrappers::{generate_wrapped_fn, shared::color::Color},
    };
}
