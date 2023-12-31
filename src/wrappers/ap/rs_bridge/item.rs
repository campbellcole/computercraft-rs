use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    pub fingerprint: Option<String>,
    pub amount: usize,
    pub display_name: String,
    pub is_craftable: bool,
    pub nbt: Option<Value>,
    pub tags: Option<Vec<String>>,
}
