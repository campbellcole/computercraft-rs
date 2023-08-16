use crate::computer::Computer;

pub struct Peripheral<'a> {
    pub(crate) computer: &'a Computer,
    pub(crate) address: String,
}
