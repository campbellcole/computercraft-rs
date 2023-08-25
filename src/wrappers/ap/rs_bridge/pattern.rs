use crate::wrappers::lua_compat::LuaVec;

use super::Item;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CraftingSlot {
    Item(Vec<Item>),
    Empty(LuaVec<()>),
}

impl CraftingSlot {
    pub fn items(&self) -> Option<&Vec<Item>> {
        match self {
            Self::Item(item) => Some(item),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub inputs: Vec<CraftingSlot>,
    pub outputs: Vec<Item>,
    pub byproducts: [Item; 9],
    pub processing: bool,
}
