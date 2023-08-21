use crate::wrappers::prelude::*;

mod citizen;
pub use citizen::*;
mod visitor;
pub use visitor::*;
mod request;
pub use request::*;

generate_wrapper_impl!(ColonyIntegrator = "colonyIntegrator");

impl<'a> ColonyIntegrator<'a> {
    pub async fn get_citizens(&self) -> Result<Vec<Citizen>> {
        self.inner
            .call_method_with("getCitizens", Value::Null)
            .await
    }

    generate_wrapped_fn!(
        is_in_colony -> bool = | | => isInColony(Value::Null);
        [Value::Bool(b)] => Ok(*b)
    );
}
