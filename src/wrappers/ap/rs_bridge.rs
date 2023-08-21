use crate::wrappers::prelude::*;

mod item;
pub use item::*;
mod filter;
pub use filter::*;

generate_wrapper_impl!(RsBridge = "rsBridge");

impl<'a> RsBridge<'a> {
    pub async fn list_items(&self) -> Result<Vec<Item>> {
        self.inner.call_method_with("listItems", Value::Null).await
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
