use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Age {
    Child,
    Adult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub level: f64,
    pub xp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub name: String,
    pub job: String,
    pub location: Position,
    #[serde(rename = "type")]
    pub kind: String,
    pub level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Home {
    pub location: Position,
    #[serde(rename = "type")]
    pub kind: String,
    pub level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citizen {
    pub id: String,
    pub name: String,
    pub age: Age,
    pub gender: Gender,
    pub location: Position,
    pub bed_pos: Position,
    // TODO: does this need to be a float?
    pub saturation: f64,
    pub happiness: f64,
    pub health: Option<f64>,
    pub max_health: Option<f64>,
    pub armor: Option<f64>,
    pub toughness: Option<f64>,
    pub better_food: bool,
    pub is_asleep: bool,
    pub is_idle: bool,
    pub state: String,
    pub children: Vec<String>,
    pub skills: HashMap<String, Skill>,
    pub work: Option<Work>,
    pub home: Option<Home>,
}
