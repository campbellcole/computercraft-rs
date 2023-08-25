use crate::wrappers::lua_compat::LuaVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestedItem {
    pub name: String,
    pub count: usize,
    pub max_stack_size: usize,
    pub display_name: String,
    pub tags: LuaVec<String>,
    // #[cfg(feature = "fastnbt")]
    // pub nbt: fastnbt::Value,
    // #[cfg(not(feature = "fastnbt"))]
    pub nbt: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColonyRequest {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub state: String,
    pub count: usize,
    pub min_count: usize,
    pub target: String,
    pub items: LuaVec<RequestedItem>,
}
