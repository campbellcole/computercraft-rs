use serde_json::Value;

use crate::wrappers::lua_compat::LuaVec;

use super::Citizen;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecruitCost {
    pub name: String,
    pub count: usize,
    pub max_stack_size: usize,
    pub display_name: String,
    pub tags: LuaVec<String>,
    pub nbt: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visitor {
    #[serde(flatten)]
    pub citizen: Citizen,
    pub recruit_cost: RecruitCost,
}
