use serde_json::Value;

use super::Citizen;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecruitCost {
    pub name: String,
    pub count: usize,
    pub max_stack_size: usize,
    pub display_name: String,
    #[serde(deserialize_with = "crate::wrappers::lua_compat::deserialize_with")]
    pub tags: Vec<String>,
    pub nbt: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Visitor {
    #[serde(flatten)]
    pub citizen: Citizen,
    pub recruit_cost: RecruitCost,
}
