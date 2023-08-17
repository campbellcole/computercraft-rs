pub mod monitor;
pub mod shared;

#[cfg(feature = "advanced-peripherals")]
pub mod ap;

macro_rules! generate_wrapper_impl {
    ($wrapper_ty:ident = $expected_ty:literal) => {
        pub struct $wrapper_ty<'a> {
            inner: Peripheral<'a>,
        }

        #[async_trait]
        impl<'a> IntoWrappedPeripheral<$wrapper_ty<'a>> for Peripheral<'a> {
            async fn into_wrapped(self) -> Result<$wrapper_ty<'a>, WrapPeripheralError> {
                let ty = self
                    .computer
                    .get_peripheral_type(self.address.clone())
                    .await
                    .ok_or(WrapPeripheralError::Disconnected)?;

                if ty != $expected_ty {
                    return Err(WrapPeripheralError::WrongType(ty, $expected_ty.into()));
                }

                Ok($wrapper_ty { inner: self })
            }
        }
    };
}

pub(crate) use generate_wrapper_impl;
