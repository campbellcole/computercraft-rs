use crate::{error::Error, wrappers::prelude::*};

mod item;
pub use item::*;
mod filter;
pub use filter::*;
mod pattern;
pub use pattern::*;

generate_wrapper_impl!(RsBridge = "rsBridge");

impl<'a> RsBridge<'a> {
    pub async fn list_items(&self) -> Result<Vec<Item>> {
        self.inner.call_method_with("listItems", Value::Null).await
    }

    pub async fn get_pattern(&self, item: RsFilter) -> Result<Option<Pattern>> {
        match self.inner.call_method_with("getPattern", vec![item]).await {
            Ok(v) => Ok(Some(v)),
            #[cfg(not(feature = "debug"))]
            Err(Error::NoReturnValues) => Ok(None),
            #[cfg(not(feature = "debug"))]
            Err(err) => Err(err),
            #[cfg(feature = "debug")]
            Err(err) => match err.downcast_ref::<Error>().unwrap() {
                Error::NoReturnValues => Ok(None),
                _ => Err(err),
            },
        }
    }

    generate_wrapped_fn!(
        craft_item -> bool = |item: RsFilter| => craftItem(vec![item]);
        [Value::Bool(b)] => Ok(*b)
    );

    generate_wrapped_fn!(
        is_item_crafting -> bool = |item: RsFilter| => isItemCrafting(vec![item]);
        [Value::Bool(b)] => Ok(*b)
    );

    generate_wrapped_fn!(
        export_item_to_peripheral -> usize = |item: RsFilter, container: impl ToString| => exportItemToPeripheral((item, container.to_string()));
        [Value::Number(n)] => Ok(n.as_u64().unwrap() as usize)
    );
}
