use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Attribute {
    #[serde(rename = "strength")]
    Strength,
    #[serde(rename = "dexterity")]
    Dexterity,
    #[serde(rename = "constitution")]
    Constitution,
    #[serde(rename = "intelligence")]
    Intelligence,
    #[serde(rename = "wisdom")]
    Wisdom,
    #[serde(rename = "charisma")]
    Charisma,
}

impl From<Attribute> for String {
    fn from(val: Attribute) -> Self {
        match val {
            Attribute::Strength => "Strength".to_string(),
            Attribute::Dexterity => "Dexterity".to_string(),
            Attribute::Constitution => "Constitution".to_string(),
            Attribute::Intelligence => "Intelligence".to_string(),
            Attribute::Wisdom => "Wisdom".to_string(),
            Attribute::Charisma => "Charisma".to_string(),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}
