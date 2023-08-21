#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsFilter {
    name: String,
    count: Option<usize>,
    nbt: Option<String>,
}

impl RsFilter {
    pub fn from_name(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            count: None,
            nbt: None,
        }
    }

    pub fn from_tag(tag: impl ToString) -> Self {
        let tag = tag.to_string();
        let tag = if tag.starts_with('#') {
            tag
        } else {
            format!("#{tag}")
        };
        Self {
            name: tag,
            count: None,
            nbt: None,
        }
    }

    pub fn with_count(self, count: usize) -> Self {
        Self {
            count: Some(count),
            ..self
        }
    }

    pub fn with_nbt(self, nbt: impl ToString) -> Self {
        Self {
            nbt: Some(nbt.to_string()),
            ..self
        }
    }
}
