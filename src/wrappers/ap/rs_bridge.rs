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
        craft_item -> bool = |item: RsFilter| => craftItem(item);
        [Value::Bool(b)] => Ok(*b)
    );
}
